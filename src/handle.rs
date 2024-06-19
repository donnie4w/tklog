// Copyright (c) 2024, donnie4w <donnie4w@gmail.com>
// All rights reserved.
// https://github.com/donnie4w/tklog
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io;

use tokio::io::AsyncWriteExt;

use crate::{
    asyncfile, get_short_file_path, parse_and_format_log, syncfile, timenow, Format, LogOption,
    CUTMODE, LEVEL, MODE,
};

pub trait FileOption: Send + Sync {
    fn mode(&self) -> CUTMODE;
    fn timemode(&self) -> MODE;
    fn filename(&self) -> String;
    fn size(&self) -> u64;
    fn maxbackups(&self) -> u32;
    fn compress(&self) -> bool;
}

pub struct FileTimeMode {
    filename: String, //Log file path
    mode: MODE,       //Maximum size for each log file to be saved
    max_backups: u32, //The maximum number of old log files that can be retained
    compress: bool,   //Whether to compress old log files
}

impl FileTimeMode {
    pub fn new(filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> Self {
        FileTimeMode {
            filename: filename.to_string(),
            mode,
            max_backups: maxbackups,
            compress,
        }
    }
}

impl FileOption for FileTimeMode {
    fn mode(&self) -> CUTMODE {
        return CUTMODE::TIME;
    }

    fn timemode(&self) -> MODE {
        return self.mode;
    }

    fn filename(&self) -> String {
        return self.filename.clone();
    }

    fn size(&self) -> u64 {
        return 0;
    }

    fn maxbackups(&self) -> u32 {
        return self.max_backups;
    }

    fn compress(&self) -> bool {
        return self.compress;
    }
}

pub struct FileSizeMode {
    filename: String, //Log file path
    max_size: u64,    //Maximum size for each log file to be saved
    max_backups: u32, //The maximum number of old log files that can be retained
    compress: bool,   //Whether to compress old log files
}

impl FileOption for FileSizeMode {
    fn mode(&self) -> CUTMODE {
        return CUTMODE::SIZE;
    }

    fn timemode(&self) -> MODE {
        return MODE::DAY;
    }

    fn filename(&self) -> String {
        return self.filename.clone();
    }

    fn size(&self) -> u64 {
        return self.max_size;
    }

    fn maxbackups(&self) -> u32 {
        return self.max_backups;
    }

    fn compress(&self) -> bool {
        return self.compress;
    }
}

impl FileSizeMode {
    pub fn new(filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> Self {
        FileSizeMode {
            filename: filename.to_string(),
            max_size: maxsize,
            max_backups: maxbackups,
            compress,
        }
    }
}

pub struct Handler {
    level: LEVEL,  // log level
    format: u8,    // log format
    console: bool, //
    formatter: String,
    file_handler: Option<syncfile::FileHandler>,
    async_file_handler: Option<asyncfile::FileHandler>,
    async_console: Option<Console>,
}

const DEFAULT_FORMATTER: &str = "{level}{time} {file}:{message}\n";

impl Handler {
    pub fn new() -> Self {
        let f = Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName;
        Handler {
            level: LEVEL::Debug,
            format: f,
            console: true,
            formatter: DEFAULT_FORMATTER.to_string(),
            file_handler: None,
            async_file_handler: None,
            async_console: None,
        }
    }

    pub fn new_with_handler(
        level: LEVEL,
        format: u8,
        console: bool,
        fh: Box<syncfile::FileHandler>,
    ) -> Self {
        Handler {
            level,
            format,
            console,
            formatter: DEFAULT_FORMATTER.to_string(),
            file_handler: Some(*fh),
            async_file_handler: None,
            async_console: None,
        }
    }

    pub fn new_with_asynchandler(
        level: LEVEL,
        format: u8,
        console: bool,
        fh: Box<asyncfile::FileHandler>,
    ) -> Self {
        Handler {
            level,
            format,
            console,
            formatter: DEFAULT_FORMATTER.to_string(),
            file_handler: None,
            async_file_handler: Some(*fh),
            async_console: Some(Console::new()),
        }
    }

    pub fn print(&mut self, s: &str) -> io::Result<()> {
        if self.is_console() {
            print!("{}", s);
        }
        if let Some(f) = self.file_handler.as_mut() {
            f.write(s.as_bytes())?;
        }
        Ok(())
    }

    pub async fn async_print(&mut self, s: &str) -> io::Result<()> {
        if self.is_console() {
            if let Some(c) = self.async_console.as_mut() {
                let _ = c.async_print(s).await;
            }
        }
        if let Some(f) = self.async_file_handler.as_mut() {
            f.write(s.as_bytes()).await?;
        }
        Ok(())
    }

    pub async fn async_console(&self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = tokio::io::stdout();
        tokio::io::stdout().write_all(s.as_bytes()).await?;
        stdout.flush().await?;
        Ok(())
    }

    pub fn set_file_handler(&mut self, filehandler: syncfile::FileHandler) {
        self.file_handler = Some(filehandler)
    }

    pub fn set_async_file_handler(&mut self, filehandler: asyncfile::FileHandler) {
        self.async_file_handler = Some(filehandler);
        self.async_console = Some(Console::new());
    }

    fn is_default_formatter(&self) -> bool {
        self.formatter.eq(DEFAULT_FORMATTER)
    }

    /** LEVEL::Debug */
    pub fn set_level(&mut self, level: LEVEL) {
        self.level = level;
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&mut self, format: u8) {
        self.format = format;
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&mut self, formatter: String) {
        self.formatter = formatter;
    }

    /** default：true */
    pub fn set_console(&mut self, console: bool) {
        self.console = console;
    }

    pub fn is_console(&self) -> bool {
        self.console
    }
}

struct Console {}
impl Console {
    pub fn new() -> Self {
        Console {}
    }
    pub async fn async_print(&self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = tokio::io::stdout();
        tokio::io::stdout().write_all(s.as_bytes()).await?;
        stdout.flush().await?;
        Ok(())
    }
}

pub struct Handle {
    pub handler: Handler,
}

impl Handle {
    pub fn new(handler: Option<Handler>) -> Self {
        if handler.is_none() {
            return Handle {
                handler: Handler::new(),
            };
        } else {
            return Handle {
                handler: handler.unwrap(),
            };
        }
    }

    pub fn print(&mut self, s: &str) -> io::Result<()> {
        self.handler.print(s)
    }

    pub async fn async_print(&mut self, s: &str) -> io::Result<()> {
        self.handler.async_print(s).await
    }

    pub fn set_handler(&mut self, handler: Handler) {
        self.handler = handler;
    }

    pub fn format(&mut self, level: LEVEL, filename: &str, line: u32, msg: String) -> String {
        let fmat = self.handler.format;

        if fmat == Format::Nano {
            return msg;
        }

        let mut levelflag = "";
        let mut time = String::new();
        let mut file = String::new();

        if fmat & Format::LevelFlag != 0 {
            levelflag = match level {
                LEVEL::Trace => "[TRACE]",
                LEVEL::Debug => "[DEBUG]",
                LEVEL::Info => "[INFO]",
                LEVEL::Warn => "[WARN]",
                LEVEL::Error => "[ERROR]",
                LEVEL::Fatal => "[FATAL]",
                LEVEL::Off => "",
            };
        }

        if fmat & (Format::Date | Format::Time | Format::Microseconds) != 0 {
            let ts = timenow();
            if fmat & Format::Date != 0 {
                time.push_str(ts[0].as_str());
            }
            if fmat & (Format::Time | Format::Microseconds) != 0 {
                if !time.is_empty() {
                    time.push(' ');
                }
                time.push_str(ts[1].as_str());
                if fmat & Format::Microseconds != 0 {
                    time.push_str(ts[2].as_str());
                }
            }
        }
        if fmat & (Format::LongFileName | Format::ShortFileName) != 0 {
            let mut f = filename;
            if fmat & Format::ShortFileName != 0 {
                f = get_short_file_path(f)
            }
            file.push_str(f);
            file.push(' ');
            file.push_str(line.to_string().as_str());
        }

        if self.handler.is_default_formatter() {
            let mut r = String::new();
            if !levelflag.is_empty() {
                r.push_str(&levelflag);
                r.push(' ');
            }
            if !time.is_empty() {
                r.push_str(&time);
                r.push(' ');
            }
            if !file.is_empty() {
                r.push_str(&file);
                r.push(':');
            }
            r.push_str(&msg);
            r.push('\n');
            return r;
        } else {
            return parse_and_format_log(
                &self.handler.formatter,
                &levelflag,
                time.as_str(),
                file.as_str(),
                msg.as_str(),
            );
        }
    }

    pub fn is_file_line(&self) -> bool {
        self.handler.format & (Format::LongFileName | Format::ShortFileName) != 0
    }

    pub fn get_level(&self) -> LEVEL {
        return self.handler.level;
    }

    pub fn set_level(&mut self, level: LEVEL) {
        self.handler.set_level(level)
    }

    pub fn get_console(&self) -> bool {
        return self.handler.console;
    }

    pub fn set_console(&mut self, console: bool) {
        self.handler.set_console(console);
    }

    pub fn get_format(&self) -> u8 {
        return self.handler.format;
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&mut self, format: u8) {
        self.handler.set_format(format);
    }

    pub fn get_formatter(&self) -> String {
        return self.handler.formatter.clone();
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&mut self, formatter: String) {
        self.handler.set_formatter(formatter);
    }

    pub fn set_option(&mut self, option: LogOption) {
        if option.level.is_some() {
            self.set_level(option.level.unwrap());
        }
        if option.console.is_some() {
            self.set_console(option.console.unwrap());
        }
        if option.format.is_some() {
            self.set_format(option.format.unwrap());
        }
        if option.formatter.is_some() {
            self.set_formatter(option.formatter.unwrap().to_string());
        }
        match option.fileoption {
            Some(fo) => {
                let fh = syncfile::FileHandler::new(fo);
                self.handler.set_file_handler(fh.unwrap());
            }
            None => {}
        }
    }

    pub async fn async_set_option(&mut self, option: LogOption) {
        if option.level.is_some() {
            self.set_level(option.level.unwrap());
        }
        if option.console.is_some() {
            self.set_console(option.console.unwrap());
        }
        if option.format.is_some() {
            self.set_format(option.format.unwrap());
        }
        if option.formatter.is_some() {
            self.set_formatter(option.formatter.unwrap().to_string());
        }
        match option.fileoption {
            Some(fo) => {
                let fh = asyncfile::FileHandler::new(fo);
                self.handler.set_async_file_handler(fh.await.unwrap());
            }
            None => {}
        }
    }
}
