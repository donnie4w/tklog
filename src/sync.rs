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

use crate::{
    arguments_to_string,
    handle::{FileSizeMode, FileTimeMode, Handle, Handler},
    l2tk, log_fmt,
    syncfile::FileHandler,
    tklog::synclog,
    HasOption, LogContext, LogOption, LEVEL, MODE, PRINTMODE, TKLOG2SYNCLOG,
};
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Sender},
};
use std::thread;

/// this is the tklog encapsulated Logger whose File operations
/// are based on the standard library std::fs::File,therefore,
/// it is  a bio sync operation object. Logger allows you to
/// set parameters for log printing of tklog.
///
/// # Examples
///
/// Create a Logger Object  and set the parameters:
///
/// ```no_run
/// use tklog::sync::Logger;
/// use tklog::Format;
/// use tklog::LEVEL;
/// use tklog::MODE;
///
/// let mut log = Logger::new();
/// log.set_console(true)
///     .set_level(LEVEL::Debug)
///     .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)
///     .set_formatter("{level}{time} {file}:{message}\n")
///     .set_cutmode_by_size("tklog.log", 1<<20, 0, true);
/// ```
pub struct Logger {
    sender: Sender<(String, String)>,
    loghandle: Handle,
    mutex: std::sync::Mutex<u32>,
    pub mode: PRINTMODE,
    modmap: HashMap<String, (HasOption, Handle)>,
    custom_handler: Option<fn(&LogContext) -> bool>,
    separator: String
}

impl Logger {
    pub fn new() -> Self {
        Self::new_with_handle(Handle::new(None))
    }

    pub fn new_with_handle(handle: Handle) -> Self {
        let (sender, receiver) = channel();
        thread::spawn(move || {
            while let Ok(s) = receiver.recv() {
                let (module, msg) = s;
                let m1: String = module;
                let m2: String = msg;
                crate::log!(m1.as_str(), m2.as_str());
            }
        });
        Logger { sender, loghandle: handle, mutex: std::sync::Mutex::new(0), mode: PRINTMODE::DELAY, modmap: HashMap::new(), custom_handler: None, separator: "".to_string() }
    }

    pub fn print(&mut self, module: &str, message: &str) {
        let mut console = self.loghandle.get_console();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get_mut(module) {
                let (has, h) = mm;
                if has.isconsole {
                    console = h.get_console();
                }
                if has.isfileoption {
                    let _ = h.print(console, message);
                    return;
                }
            }
        }
        let _ = self.loghandle.print(console, message);
    }

    pub fn safeprint(&mut self, module: &str, message: &str) {
        let mut console = self.loghandle.get_console();
        let _ = &self.mutex.lock();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get_mut(module) {
                let (has, h) = mm;
                if has.isconsole {
                    console = h.get_console();
                }
                if has.isfileoption {
                    let _ = h.print(console, message);
                    return;
                }
            }
        }
        let _ = self.loghandle.print(console, message);
    }

    pub fn log(&self, module: String, message: String) {
        self.sender.send((module, message)).expect("send error");
    }

    pub fn get_level(&self, module: &str) -> LEVEL {
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (l, h) = mm;
                if l.islevel {
                    return h.get_level();
                }
            }
        }
        self.loghandle.get_level()
    }

    pub fn set_handler(&mut self, handler: Handler) {
        self.loghandle.set_handler(handler);
    }

    pub fn is_file_line(&self, module: &str) -> bool {
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (has, h) = mm;
                if has.isformat {
                    return h.is_file_line();
                }
            }
        }
        self.loghandle.is_file_line()
    }

    pub fn fmt(&mut self, module: &str, level: LEVEL, filename: &str, line: u32, message: String) -> String {
        if self.custom_handler.is_some() {
            if let Some(ch) = &self.custom_handler {
                if !ch(&LogContext { level: level, filename: filename.to_string(), line: line, log_body: message.clone(), modname: module.to_string() }) {
                    return String::new();
                }
            }
        }
        let mut fmat = self.loghandle.get_format();
        let mut formatter = self.loghandle.get_formatter();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (has, h) = mm;
                if has.isformat {
                    fmat = h.get_format();
                }
                if has.isformatter {
                    formatter = h.get_formatter();
                }
            }
        }
        log_fmt(fmat, formatter, level, filename, line, message)
    }

    pub fn set_printmode(&mut self, mode: PRINTMODE) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn set_level(&mut self, level: LEVEL) -> &mut Self {
        self.loghandle.set_level(level);
        self
    }

    pub fn set_console(&mut self, console: bool) -> &mut Self {
        self.loghandle.set_console(console);
        self
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&mut self, format: u8) -> &mut Self {
        self.loghandle.set_format(format);
        self
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&mut self, formatter: &str) -> &mut Self {
        self.loghandle.set_formatter(formatter.to_string());
        self
    }

    pub fn set_cutmode_by_size(&mut self, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> &mut Self {
        let fsm = FileSizeMode::new(filename, maxsize, maxbackups, compress);
        let fh = FileHandler::new(Box::new(fsm));
        self.loghandle.handler.set_file_handler(fh.unwrap());
        self
    }

    pub fn set_cutmode_by_time(&mut self, filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> &mut Self {
        let ftm = FileTimeMode::new(filename, mode, maxbackups, compress);
        let fh = FileHandler::new(Box::new(ftm));
        self.loghandle.handler.set_file_handler(fh.unwrap());
        self
    }

    pub fn set_option(&mut self, option: LogOption) -> &mut Self {
        self.loghandle.set_option(option);
        self
    }

    pub fn set_mod_option(&mut self, module: &str, option: LogOption) -> &mut Self {
        let mut handler = Handle::new(None);
        let ho = HasOption {
            islevel: option.level.is_some(),
            isformat: option.format.is_some(),
            isformatter: option.formatter.is_some(),
            isconsole: option.console.is_some(),
            isfileoption: option.fileoption.is_some(),
        };
        handler.set_option(option);
        self.modmap.insert(module.to_string(), (ho, handler));
        self
    }

    pub fn set_custom_handler(&mut self, handler: fn(&LogContext) -> bool) -> &mut Self {
        self.custom_handler = Some(handler);
        self
    }

    pub fn set_separator(&mut self, separator: &str) -> &mut Self {
        self.separator = separator.to_string();
        self
    }

    pub fn get_separator(&self) -> String {
        self.separator.clone()
    }
}

pub struct Log;

impl Log {
    pub fn new() -> Self {
        Log {}
    }
    pub fn set_printmode(&self, mode: PRINTMODE) -> &Self {
        unsafe {
            synclog.set_printmode(mode);
        }
        self
    }

    pub fn set_level(&self, level: LEVEL) -> &Self {
        unsafe {
            synclog.set_level(level);
        }
        self
    }

    pub fn set_console(&self, console: bool) -> &Self {
        unsafe {
            synclog.set_console(console);
        }
        self
    }

    /**Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName; */
    pub fn set_format(&self, format: u8) -> &Self {
        unsafe {
            synclog.set_format(format);
        }
        self
    }

    /** default: "{level}{time} {file}:{message}\n" */
    pub fn set_formatter(&self, formatter: &str) -> &Self {
        unsafe {
            synclog.set_formatter(formatter);
        }
        self
    }

    pub fn set_cutmode_by_size(&self, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> &Self {
        unsafe {
            synclog.set_cutmode_by_size(filename, maxsize, maxbackups, compress);
        }
        self
    }

    pub fn set_cutmode_by_time(&self, filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> &Self {
        unsafe {
            synclog.set_cutmode_by_time(filename, mode, maxbackups, compress);
        }
        self
    }

    pub fn set_option(&self, option: LogOption) -> &Self {
        unsafe {
            synclog.set_option(option);
        }
        self
    }

    pub fn set_mod_option(&self, module: &str, option: LogOption) -> &Self {
        unsafe {
            synclog.set_mod_option(module, option);
        }
        self
    }

    pub fn set_custom_handler(&self, handler: fn(&LogContext) -> bool) -> &Self {
        unsafe {
            synclog.set_custom_handler(handler);
        }
        self
    }

    pub fn set_separator(&self, separator: &str) -> &Self {
        unsafe {
            synclog.set_separator(separator);
        }
        self
    }

    fn is_file_line(&self, module: &str) -> bool {
        unsafe {
            return synclog.is_file_line(module);
        }
    }

    pub fn uselog(&self) -> &Self {
        let _ = log::set_logger(&TKLOG2SYNCLOG);
        log::set_max_level(log::LevelFilter::Trace);
        self
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
                if synclog.get_level(module) > level {
                    return;
                }
            }
        }
        let args = record.args();
        let mut file = "";
        let mut line: u32 = 0;
        if self.is_file_line(module) {
            line = record.line().unwrap_or(0);
            file = record.file().unwrap_or("");
        }
        unsafe {
            if synclog.mode == PRINTMODE::DELAY {
                let s = synclog.fmt(module, level, file, line, arguments_to_string(args));
                if !s.is_empty() {
                    synclog.log(module.to_string(), s);
                }
            } else {
                let s = synclog.fmt(module, level, file, line, arguments_to_string(args));
                if !s.is_empty() {
                    synclog.safeprint(module, s.as_str());
                }
            }
        }
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! log {
    ($module:expr,$msg:expr) => {
        let msg: &str = $msg;
        let module: &str = $module;
        unsafe {
            synclog.print(module, msg);
        }
    };
}

#[macro_export]
macro_rules! trace {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::LEVEL::Trace, $($arg),*);
    };
}

#[macro_export]
macro_rules! debug {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::LEVEL::Debug, $($arg),*);
    };
}

#[macro_export]
macro_rules! info {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::LEVEL::Info, $($arg),*);
    };
}

#[macro_export]
macro_rules! warn {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::LEVEL::Warn, $($arg),*);
    };
}

#[macro_export]
macro_rules! error {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::LEVEL::Error, $($arg),*);
    };
}

#[macro_export]
macro_rules! fatal {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::LEVEL::Fatal, $($arg),*);
    };
}

#[macro_export]
macro_rules! log_common {
    ($level:expr, $($arg:expr),*) => {
        unsafe {
            let module = module_path!();
            if $crate::tklog::synclog.get_level(module) <= $level {
                let formatted_args: Vec<String> = vec![$(format!("{}", $arg)),*];
                let mut file = "";
                let mut line = 0;
                if $crate::tklog::synclog.is_file_line(module) {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join($crate::tklog::synclog.get_separator().as_str());
                if  $crate::tklog::synclog.mode==$crate::PRINTMODE::DELAY {
                    let s = $crate::tklog::synclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::synclog.log(module.to_string(),s);
                    }
                }else {
                    let s = $crate::tklog::synclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::synclog.safeprint(module,s.as_str());
                    }
                }
            }
        }
    };
    () => {};
}
