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

use std::collections::HashMap;

use crate::asyncfile::FileHandler;
use crate::handle::{FHandler, FileOptionType, FmtHandler};
use crate::tklog::asynclog;
use crate::trie::Trie;
use crate::{arguments_to_string, l2tk, log_fmt, Format, LogContext, AttrFormat, LogOption, LogOptionConst, OptionTrait, LEVEL, MODE, PRINTMODE, TKLOG2ASYNC_LOG};
use tokio::sync::mpsc;

/// this is the tklog encapsulated Logger whose File operations
/// are based on tokio, Therefore, it supports asynchronous scenarios
/// and Logger allows you to set parameters for async log printing of tklog.
///
/// # Examples
///
/// Create a async Logger Object  and set the parameters:
///
/// ```no_run
/// use tklog::LEVEL;
/// use tklog::Format;
/// use tklog::MODE;
///
/// let mut log = tklog::Async::Logger::new();
///
/// let  init =  async ||  {
/// log.set_console(true)
/// .set_level(LEVEL::Debug)
/// .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true).await;
/// };
/// ```
pub struct Logger {
    sender: mpsc::UnboundedSender<(LEVEL, String, String)>,
    fmthandle: FmtHandler,
    filehandle: (String, FHandler),
    mutex: tokio::sync::Mutex<u32>,
    pub mode: PRINTMODE,
    modmap: Trie<(LogOptionConst, String)>,
    fmap: HashMap<String, FHandler>,
    custom_handler: Option<fn(&LogContext) -> bool>,
    separator: String,
    levels: [Option<(LogOption, String)>; 7],
    // levelfmt: Option<Box<dyn Fn(LEVEL) -> String + Send + Sync>>,
    // timefmt: Option<Box<dyn Fn() -> (String, String, String) + Send + Sync>>,
    attrfmt: AttrFormat,
}

impl Logger {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                let (level, module, msg) = message;
                let m1: String = module;
                let m2: String = msg;
                crate::async_log!(level, m1.as_str(), m2.as_str());
            }
        });
        Logger {
            sender,
            fmthandle: FmtHandler::new(),
            filehandle: ("".to_string(), FHandler::new()),
            mutex: tokio::sync::Mutex::new(0),
            mode: PRINTMODE::DELAY,
            modmap: Trie::new(),
            fmap: HashMap::new(),
            custom_handler: None,
            separator: "".to_string(),
            levels: std::array::from_fn(|_| None),
            // levelfmt: None,
            // timefmt: None,
            attrfmt: AttrFormat::new(),
        }
    }

    pub async fn print(&mut self, level: LEVEL, module: &str, message: &str) {
        let mut console = self.fmthandle.get_console();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, filename) = mm;
                if let Some(cs) = lo.console {
                    console = cs
                }
                if filename != "" {
                    if *filename == self.filehandle.0 {
                        let _ = self.filehandle.1.async_print(console, message).await;
                    } else {
                        if let Some(fm) = self.fmap.get_mut(filename) {
                            let _ = fm.async_print(console, message).await;
                        }
                    }
                    return;
                }
            }
        }

        if let Some(lp) = &mut self.levels[level as usize - 1] {
            let (lo, filename) = lp;
            if let Some(cs) = lo.console {
                console = cs
            }
            if filename != "" {
                if *filename == self.filehandle.0 {
                    let _ = self.filehandle.1.async_print(console, message).await;
                } else {
                    if let Some(fm) = self.fmap.get_mut(filename) {
                        let _ = fm.async_print(console, message).await;
                    }
                }
                return;
            }
        }
        let _ = self.filehandle.1.async_print(console, message).await;
    }

    pub async fn safeprint(&mut self, level: LEVEL, module: &str, message: &str) {
        let _mutex_guard = self.mutex.lock().await;
        let mut console = self.fmthandle.get_console();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, filename) = mm;
                if let Some(cs) = lo.console {
                    console = cs
                }
                if filename != "" {
                    if *filename == self.filehandle.0 {
                        let _ = self.filehandle.1.async_print(console, message).await;
                    } else {
                        if let Some(fm) = self.fmap.get_mut(filename) {
                            let _ = fm.async_print(console, message).await;
                        }
                    }
                    return;
                }
            }
        }

        if let Some(lp) = &mut self.levels[level as usize - 1] {
            let (lo, filename) = lp;
            if let Some(cs) = lo.console {
                console = cs
            }
            if filename != "" {
                if *filename == self.filehandle.0 {
                    let _ = self.filehandle.1.async_print(console, message).await;
                } else {
                    if let Some(fm) = self.fmap.get_mut(filename) {
                        let _ = fm.async_print(console, message).await;
                    }
                }
                return;
            }
        }
        let _ = self.filehandle.1.async_print(console, message).await;
    }

    pub fn log(&self, level: LEVEL, module: String, message: String) {
        self.sender.send((level, module, message)).expect("send error");
    }

    pub fn get_level(&mut self, module: &str) -> LEVEL {
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, _) = mm;
                if let Some(level) = lo.level {
                    return level;
                }
            }
        }
        self.fmthandle.get_level()
    }

    pub fn is_file_line(&mut self, level: LEVEL, module: &str) -> bool {
        if let Some(lp) = &self.levels[level as usize - 1] {
            let (lo, _) = lp;
            if let Some(v) = lo.format {
                return v & (Format::LongFileName | Format::ShortFileName) != 0;
            }
        }

        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, _) = mm;
                if let Some(v) = lo.format {
                    return v & (Format::LongFileName | Format::ShortFileName) != 0;
                }
            }
        }
        self.fmthandle.is_file_line()
    }

    pub fn fmt(&mut self, module: &str, level: LEVEL, filename: &str, line: u32, message: String) -> String {
        if self.custom_handler.is_some() {
            if let Some(ch) = &self.custom_handler {
                if !ch(&LogContext { level: level, filename: filename.to_string(), line: line, log_body: message.clone(), modname: module.to_string() }) {
                    return String::new();
                }
            }
        }
        let mut fmat = self.fmthandle.get_format();
        let mut formatter = self.fmthandle.get_formatter();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, _) = mm;
                if let Some(v) = lo.format {
                    fmat = v;
                }
                if let Some(v) = &lo.formatter {
                    formatter = v.to_string();
                }
            }
        }

        if let Some(lp) = &self.levels[level as usize - 1] {
            let (lo, _) = lp;
            if let Some(v) = lo.format {
                fmat = v;
            }
            if let Some(v) = &lo.formatter {
                formatter = v.to_string();
            }
        }

        log_fmt(self.attrfmt.levelfmt.as_ref(), self.attrfmt.timefmt.as_ref(), fmat, formatter.as_str(), level, filename, line, message)
    }

    pub fn set_printmode(&mut self, mode: PRINTMODE) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn set_level(&mut self, level: LEVEL) -> &mut Self {
        self.fmthandle.set_level(level);
        self
    }

    pub fn set_console(&mut self, console: bool) -> &mut Self {
        self.fmthandle.set_console(console);
        self
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&mut self, format: u8) -> &mut Self {
        self.fmthandle.set_format(format);
        self
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&mut self, formatter: &str) -> &mut Self {
        self.fmthandle.set_formatter(formatter.to_string());
        self
    }

    pub async fn set_cutmode_by_size(&mut self, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> &mut Self {
        let fsm = FileOptionType::new(crate::CUTMODE::SIZE, MODE::DAY, filename, maxsize, maxbackups, compress);
        let fh = FileHandler::new(Box::new(fsm)).await;
        self.filehandle.0 = filename.to_string();
        self.filehandle.1.set_async_file_handler(fh.unwrap());
        self
    }

    pub async fn set_cutmode_by_time(&mut self, filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> &mut Self {
        let ftm = FileOptionType::new(crate::CUTMODE::TIME, mode, filename, 0, maxbackups, compress);
        let fh = FileHandler::new(Box::new(ftm)).await;
        self.filehandle.0 = filename.to_string();
        self.filehandle.1.set_async_file_handler(fh.unwrap());
        self
    }

    pub async fn set_option(&mut self, option: LogOption) -> &mut Self {
        if let Some(v) = option.console {
            self.fmthandle.set_console(v);
        }
        if let Some(v) = option.format {
            self.fmthandle.set_format(v);
        }
        if let Some(v) = option.formatter {
            self.fmthandle.set_formatter(v);
        }
        if let Some(v) = option.level {
            self.fmthandle.set_level(v);
        }
        if let Some(v) = option.fileoption {
            match FileHandler::new(v).await {
                Ok(f) => {
                    self.filehandle.0 = f.get_file_name();
                    self.filehandle.1.set_async_file_handler(f);
                }
                Err(_) => {}
            }
        }
        self
    }

    pub async fn set_mod_option(&mut self, module: &str, option: LogOption) -> &mut Self {
        let mut filename = "".to_string();
        if let Some(v) = option.fileoption {
            match FileHandler::new(v).await {
                Ok(f) => {
                    filename = f.get_file_name();
                    if filename != self.filehandle.0 && !self.fmap.contains_key(&filename) {
                        let mut fhandler = FHandler::new();
                        fhandler.set_async_file_handler(f);
                        self.fmap.insert(filename.clone(), fhandler);
                    }
                }
                Err(_) => {}
            }
        }
        self.modmap.insert(module, (LogOptionConst { level: option.level, format: option.format, formatter: option.formatter, console: option.console }, filename.clone()));
        self
    }

    pub fn set_custom_handler(&mut self, handler: fn(&LogContext) -> bool) {
        self.custom_handler = Some(handler);
    }

    pub async fn set_level_option(&mut self, level: LEVEL, option: &dyn OptionTrait) -> &mut Self {
        let mut filename = "".to_string();
        if let Some(v) = option.get_fileoption() {
            match FileHandler::new(v).await {
                Ok(f) => {
                    filename = f.get_file_name();
                    if filename != self.filehandle.0 && !self.fmap.contains_key(&filename) {
                        let mut fhandler = FHandler::new();
                        fhandler.set_async_file_handler(f);
                        self.fmap.insert(filename.clone(), fhandler);
                    }
                }
                Err(_) => {}
            }
        }
        let lo = LogOption { level: None, format: option.get_format(), formatter: option.get_formatter(), console: option.get_console(), fileoption: option.get_fileoption() };
        self.levels[level as usize - 1] = Some((lo, filename));
        self
    }

    pub fn set_separator(&mut self, separator: &str) -> &mut Self {
        self.separator = separator.to_string();
        self
    }

    pub fn get_separator(&self) -> String {
        self.separator.clone()
    }

    pub fn set_attr_format<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut AttrFormat) + Send + Sync + 'static,
    {
        f(&mut self.attrfmt);
    }
}

pub struct Log;

impl Log {
    pub fn new() -> Self {
        Log {}
    }
    pub fn set_printmode(&self, mode: PRINTMODE) -> &Self {
        unsafe {
            asynclog.set_printmode(mode);
        }
        self
    }

    pub fn set_level(&self, level: LEVEL) -> &Self {
        unsafe {
            asynclog.set_level(level);
        }
        self
    }

    pub fn set_console(&self, console: bool) -> &Self {
        unsafe {
            asynclog.set_console(console);
        }
        self
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&self, format: u8) -> &Self {
        unsafe {
            asynclog.set_format(format);
        }
        self
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&self, formatter: &str) -> &Self {
        unsafe {
            asynclog.set_formatter(formatter);
        }
        self
    }

    pub async fn set_cutmode_by_size(&self, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> &Self {
        unsafe {
            asynclog.set_cutmode_by_size(filename, maxsize, maxbackups, compress).await;
        }
        self
    }

    pub async fn set_cutmode_by_time(&self, filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> &Self {
        unsafe {
            asynclog.set_cutmode_by_time(filename, mode, maxbackups, compress).await;
        }
        self
    }

    pub fn set_custom_handler(&self, handler: fn(&LogContext) -> bool) -> &Self {
        unsafe { asynclog.set_custom_handler(handler) }
        self
    }

    fn is_file_line(&self, level: LEVEL, module: &str) -> bool {
        unsafe {
            return asynclog.is_file_line(level, module);
        }
    }

    pub async fn set_option(&self, option: LogOption) -> &Self {
        unsafe {
            asynclog.set_option(option).await;
        }
        self
    }

    pub async fn set_mod_option(&self, module: &str, option: LogOption) -> &Self {
        unsafe {
            asynclog.set_mod_option(module, option).await;
        }
        self
    }

    pub async fn set_level_option(&self, level: LEVEL, option: impl OptionTrait) -> &Self {
        unsafe {
            asynclog.set_level_option(level, &option).await;
        }
        self
    }

    pub fn set_separator(&self, separator: &str) -> &Self {
        unsafe {
            asynclog.set_separator(separator);
        };
        self
    }

    pub fn uselog(&self) -> &Self {
        let _ = log::set_logger(&TKLOG2ASYNC_LOG);
        log::set_max_level(log::LevelFilter::Trace);
        self
    }

    pub fn set_attr_format<F>(&self, f: F)
    where
        F: FnMut(&mut AttrFormat) + Send + Sync + 'static,
    {
        unsafe {
            asynclog.set_attr_format(f);
        }
    }
}

impl log::Log for Log {
    fn enabled(&self, _: &log::Metadata) -> bool {
        return true;
    }
    fn log(&self, record: &log::Record) {
        let level = l2tk(record.level());
        let mut module = "";
        if let Some(m) = record.module_path() {
            module = m;
            unsafe {
                if asynclog.get_level(module) > level {
                    return;
                }
            }
        }

        let args = record.args();
        let mut file = "";
        let mut line: u32 = 0;
        if self.is_file_line(level, module) {
            line = record.line().unwrap_or(0);
            file = record.file().unwrap_or("");
        }
        unsafe {
            let s = asynclog.fmt(module, level, file, line, arguments_to_string(args));
            if !s.is_empty() {
                asynclog.log(level, module.to_string(), s);
            }
        }
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! async_log {
    ($level:expr,$module:expr,$msg:expr) => {
        let msg: &str = $msg;
        let module: &str = $module;
        unsafe {
            crate::tklog::asynclog.print($level, module, msg).await;
        }
    };
}

#[macro_export]
macro_rules! async_trace {
    () => {};
    ($($arg:expr),*) => {
        $crate::async_log_common!($crate::LEVEL::Trace, $($arg),*);
    };
}

#[macro_export]
macro_rules! async_debug {
    () => {};
    ($($arg:expr),*) => {
        $crate::async_log_common!($crate::LEVEL::Debug, $($arg),*);
    };
}

#[macro_export]
macro_rules! async_info {
    () => {};
    ($($arg:expr),*) => {
        $crate::async_log_common!($crate::LEVEL::Info, $($arg),*);
    };
}

#[macro_export]
macro_rules! async_warn {
    () => {};
    ($($arg:expr),*) => {
        $crate::async_log_common!($crate::LEVEL::Warn, $($arg),*);
    };
}

#[macro_export]
macro_rules! async_error {
    () => {};
    ($($arg:expr),*) => {
        $crate::async_log_common!($crate::LEVEL::Error, $($arg),*);
    };
}

#[macro_export]
macro_rules! async_fatal {
    () => {};
    ($($arg:expr),*) => {
        $crate::async_log_common!($crate::LEVEL::Fatal, $($arg),*);
    };
}

#[macro_export]
macro_rules! async_log_common {
    ($level:expr, $($arg:expr),*) => {
        unsafe {
            let module = module_path!();
            if $crate::tklog::asynclog.get_level(module) <= $level {
                let formatted_args: Vec<String> = vec![$(format!("{}", $arg)),*];
                let mut file = "";
                let mut line = 0;
                if $crate::tklog::asynclog.is_file_line($level,module) {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join($crate::tklog::asynclog.get_separator().as_str());
                if  $crate::tklog::asynclog.mode==$crate::PRINTMODE::DELAY {
                    let s = $crate::tklog::asynclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::asynclog.log($level,module.to_string(),s);
                    }
                }else {
                    let s = $crate::tklog::asynclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::asynclog.safeprint($level,module,s.as_str()).await;
                    }
                }
            }
        }
    };
    () => {};
}
