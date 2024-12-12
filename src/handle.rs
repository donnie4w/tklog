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

use crate::{asyncfile, syncfile, Format, CUTMODE, LEVEL, MODE};

pub trait FileOption: Send + Sync {
    fn mode(&self) -> CUTMODE;
    fn timemode(&self) -> MODE;
    fn filename(&self) -> String;
    fn size(&self) -> u64;
    fn maxbackups(&self) -> u32;
    fn compress(&self) -> bool;
}

pub struct FileOptionType {
    pub mode: CUTMODE,
    pub timemode: MODE,
    pub filename: String,
    pub size: u64,
    pub maxbackups: u32,
    pub compress: bool,
}

impl FileOptionType {
    pub fn new(mode: CUTMODE, timemode: MODE, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> Self {
        FileOptionType { mode: mode, timemode: timemode, filename: filename.to_string(), size: maxsize, maxbackups, compress }
    }
}

impl FileOption for FileOptionType {
    fn mode(&self) -> CUTMODE {
        self.mode
    }

    fn timemode(&self) -> MODE {
        self.timemode
    }

    fn filename(&self) -> String {
        self.filename.clone()
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn maxbackups(&self) -> u32 {
        self.maxbackups
    }

    fn compress(&self) -> bool {
        self.compress
    }
}

pub struct FileTimeMode {
    filename: String, //Log file path
    mode: MODE,       //Maximum size for each log file to be saved
    max_backups: u32, //The maximum number of old log files that can be retained
    compress: bool,   //Whether to compress old log files
}

impl FileTimeMode {
    pub fn new(filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> Self {
        FileTimeMode { filename: filename.to_string(), mode, max_backups: maxbackups, compress }
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
        FileSizeMode { filename: filename.to_string(), max_size: maxsize, max_backups: maxbackups, compress }
    }
}

pub struct FmtHandler {
    level: LEVEL,              // log level
    format: u8,                // log format
    console: bool,             // log console
    formatter: Option<String>, // log formatter
}

impl FmtHandler {
    pub fn new() -> Self {
        let f = Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName;
        FmtHandler { level: crate::env_level(), format: f, console: true, formatter: None }
    }

    pub async fn async_console(&self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = tokio::io::stdout();
        tokio::io::stdout().write_all(s.as_bytes()).await?;
        stdout.flush().await?;
        Ok(())
    }

    /** LEVEL::Debug */
    pub fn set_level(&mut self, level: LEVEL) {
        self.level = level;
    }

    pub fn get_level(&self) -> LEVEL {
        self.level
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&mut self, format: u8) {
        self.format = format;
    }

    pub fn get_format(&self) -> u8 {
        self.format
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&mut self, formatter: String) {
        self.formatter = Some(formatter);
    }

    pub fn get_formatter(&self) -> Option<&String> {
        self.formatter.as_ref()
    }

    /** default：true */
    pub fn set_console(&mut self, console: bool) {
        self.console = console;
    }

    pub fn get_console(&self) -> bool {
        self.console
    }

    pub fn is_console(&self) -> bool {
        self.console
    }

    pub fn is_file_line(&self) -> bool {
        self.format & (Format::LongFileName | Format::ShortFileName) != 0
    }
}

pub struct FHandler {
    file_handler: Option<syncfile::FileHandler>,
    async_file_handler: Option<asyncfile::FileHandler>,
    async_console: Option<Console>,
}

impl FHandler {
    pub fn new() -> Self {
        FHandler { file_handler: None, async_file_handler: None, async_console: None }
    }

    pub fn new_with_handler(fh: Box<syncfile::FileHandler>) -> Self {
        FHandler { file_handler: Some(*fh), async_file_handler: None, async_console: None }
    }

    pub fn new_with_asynchandler(fh: Box<asyncfile::FileHandler>) -> Self {
        FHandler { file_handler: None, async_file_handler: Some(*fh), async_console: Some(Console::new()) }
    }

    pub fn print(&mut self, is_console: bool, console: &str, s: &str) -> io::Result<()> {
        if is_console {
            print!("{}", if console.is_empty() { s } else { console });
        }
        if let Some(f) = self.file_handler.as_mut() {
            f.write(s.as_bytes())?;
        }
        Ok(())
    }

    pub async fn async_print(&mut self, is_console: bool, console: &str, s: &str) -> io::Result<()> {
        if is_console {
            if self.async_console.is_none() {
                let cs = Console::new();
                let _ = cs.async_print(if console.is_empty() { s } else { console }).await;
                self.async_console = Some(cs)
            } else if let Some(c) = self.async_console.as_mut() {
                let _ = c.async_print(if console.is_empty() { s } else { console }).await;
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
