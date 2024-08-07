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
use crate::handle::{FileSizeMode, FileTimeMode, Handle, Handler};
use crate::tklog::asynclog;
use crate::{arguments_to_string, l2tk, log_fmt, HasOption, LogContext, LogOption, LEVEL, MODE, PRINTMODE, TKLOG2ASYNC_LOG};
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
    sender: mpsc::UnboundedSender<(String, String)>,
    loghandle: Handle,
    mutex: tokio::sync::Mutex<u32>,
    pub mode: PRINTMODE,
    modmap: HashMap<String, (HasOption, Handle)>,
    custom_handler:Option<fn(&LogContext) -> bool>,
}

impl Logger {
    pub fn new() -> Self {
        Self::new_with_handle(Handle::new(None))
    }
    pub fn new_with_handle(handle: Handle) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                let (module, msg) = message;
                let m1: String = module;
                let m2: String = msg;
                crate::async_log!(m1.as_str(), m2.as_str());
            }
        });
        Logger { sender, loghandle: handle, mutex: tokio::sync::Mutex::new(0), mode: PRINTMODE::DELAY, modmap: HashMap::new(), custom_handler: None }
    }

    pub fn log(&self, module: String, message: String) {
        self.sender.send((module, message)).expect("send error");
    }

    pub async fn print(&mut self, module: &str, message: &str) {
        let mut console = self.loghandle.get_console();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get_mut(module) {
                let (has, h) = mm;
                if has.isconsole {
                    console = h.get_console();
                }
                if has.isfileoption {
                    let _ = h.async_print(console, message).await;
                    return;
                }
            }
        }
        let _ = self.loghandle.async_print(console, message).await;
    }

    pub async fn safeprint(&mut self, module: &str, message: &str) {
        let mut console = self.loghandle.get_console();
        let _ = &self.mutex.lock().await;
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get_mut(module) {
                let (has, h) = mm;
                if has.isconsole {
                    console = h.get_console();
                }
                if has.isfileoption {
                    let _ = h.async_print(console, message).await;
                    return;
                }
            }
        }
        let _ = self.loghandle.async_print(console, message).await;
    }

    pub fn get_level(&self, module: &str) -> LEVEL {
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (has, h) = mm;
                if has.islevel {
                    return h.get_level();
                }
            }
        }
        self.loghandle.get_level()
    }

    pub async fn set_handler(&mut self, handler: Handler) {
        let _ = &self.mutex.lock().await;
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

    pub async fn set_cutmode_by_size(&mut self, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> &mut Self {
        let fsm = FileSizeMode::new(filename, maxsize, maxbackups, compress);
        let fh = FileHandler::new(Box::new(fsm)).await;
        self.loghandle.handler.set_async_file_handler(fh.unwrap());
        self
    }

    pub async fn set_cutmode_by_time(&mut self, filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> &mut Self {
        let ftm = FileTimeMode::new(filename, mode, maxbackups, compress);
        let fh = FileHandler::new(Box::new(ftm)).await;
        self.loghandle.handler.set_async_file_handler(fh.unwrap());
        self
    }

    pub async fn set_option(&mut self, option: LogOption) -> &mut Self {
        self.loghandle.async_set_option(option).await;
        self
    }

    pub async fn set_mod_option(&mut self, module: &str, option: LogOption) -> &mut Self {
        let mut handler = Handle::new(None);
        let ho = HasOption {
            islevel: option.level.is_some(),
            isformat: option.format.is_some(),
            isformatter: option.formatter.is_some(),
            isconsole: option.console.is_some(),
            isfileoption: option.fileoption.is_some(),
        };
        handler.async_set_option(option).await;
        self.modmap.insert(module.to_string(), (ho, handler));
        self
    }

    pub fn set_custom_handler(&mut self, handler: fn(&LogContext) -> bool) {
        self.custom_handler = Some(handler);
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

    pub fn set_custom_handler(&self, handler:fn(&LogContext) -> bool) -> &Self {
        unsafe { asynclog.set_custom_handler(handler) }
        self
    }

    fn is_file_line(&self, module: &str) -> bool {
        unsafe {
            return asynclog.is_file_line(module);
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

    pub fn uselog(&self) -> &Self {
        let _ = log::set_logger(&TKLOG2ASYNC_LOG);
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
                if asynclog.get_level(module) > level {
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
            let s = asynclog.fmt(module, level, file, line, arguments_to_string(args));
            if !s.is_empty() {
                asynclog.log(module.to_string(), s);
            }
        }
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! async_log {
    ($module:expr,$msg:expr) => {
        let msg: &str = $msg;
        let module: &str = $module;
        unsafe {
            crate::tklog::asynclog.print(module, msg).await;
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
                if $crate::tklog::asynclog.is_file_line(module) {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join(",");
                if  $crate::tklog::asynclog.mode==$crate::PRINTMODE::DELAY {
                    let s = $crate::tklog::asynclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::asynclog.log(module.to_string(),s);
                    }
                }else {
                    let s = $crate::tklog::asynclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::asynclog.safeprint(module,s.as_str()).await;
                    }
                }
            }
        }
    };
    () => {};
}
