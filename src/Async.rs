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

use tokio::sync::mpsc;
use crate::asyncfile::FileHandler;
use crate::handle::{FileSizeMode, FileTimeMode, Handle, Handler};
use crate::tklog::asynclog;
use crate::{arguments_to_string, l2tk, tk2l, LEVEL, MODE, PRINTMODE, TKLOG2ASYNC_LOG};

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
    sender: mpsc::UnboundedSender<String>,
    loghandle: Handle,
    mutex: tokio::sync::Mutex<u32>,
    pub mode: PRINTMODE,
}

impl Logger {
    pub fn new() -> Self {
        Self::new_with_handle(Handle::new(None))
    }

    pub fn new_with_handle(handle: Handle) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                let msg: String = message;
                crate::async_log!(msg.as_str());
            }
        });
        Logger {
            sender,
            loghandle: handle,
            mutex: tokio::sync::Mutex::new(0),
            mode: PRINTMODE::DELAY,
        }
    }

    pub fn log(&self, message: String) {
        self.sender.send(message).expect("send error");
    }

    pub async fn print(&mut self, message: &str) {
        let _ = self.loghandle.async_print(message).await;
    }

    pub async fn safeprint(&mut self, message: &str) {
        let _ = &self.mutex.lock().await;
        let _ = self.loghandle.async_print(message).await;
    }

    pub fn get_level(&self) -> LEVEL {
        return self.loghandle.get_level();
    }

    pub async fn set_handler(&mut self, handler: Handler) {
        let _ = &self.mutex.lock().await;
        self.loghandle.set_handler(handler);
    }

    pub fn is_file_line(&self) -> bool {
        self.loghandle.is_file_line()
    }

    pub fn fmt(&mut self, level: LEVEL, filename: &str, line: u32, message: String) -> String {
        self.loghandle.format(level, filename, line, message)
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

    pub async fn set_cutmode_by_size(
        &mut self,
        filename: &str,
        maxsize: u64,
        maxbackups: u32,
        compress: bool,
    ) -> &mut Self {
        let fsm = FileSizeMode::new(filename, maxsize, maxbackups, compress);
        let fh = FileHandler::new(Box::new(fsm)).await;
        self.loghandle.handler.set_async_file_handler(fh.unwrap());
        self
    }

    pub async fn set_cutmode_by_time(
        &mut self,
        filename: &str,
        mode: MODE,
        maxbackups: u32,
        compress: bool,
    ) -> &mut Self {
        let ftm = FileTimeMode::new(filename, mode, maxbackups, compress);
        let fh = FileHandler::new(Box::new(ftm)).await;
        self.loghandle.handler.set_async_file_handler(fh.unwrap());
        self
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

    pub async fn set_cutmode_by_size(
        &self,
        filename: &str,
        maxsize: u64,
        maxbackups: u32,
        compress: bool,
    ) -> &Self {
        unsafe {
            asynclog
                .set_cutmode_by_size(filename, maxsize, maxbackups, compress)
                .await;
        }
        self
    }

    pub async fn set_cutmode_by_time(
        &self,
        filename: &str,
        mode: MODE,
        maxbackups: u32,
        compress: bool,
    ) -> &Self {
        unsafe {
            asynclog
                .set_cutmode_by_time(filename, mode, maxbackups, compress)
                .await;
        }
        self
    }

    fn is_file_line(&self) -> bool {
        unsafe {
            return asynclog.is_file_line();
        }
    }

    pub fn uselog(&self) -> &Self {
        let _ = log::set_logger(&TKLOG2ASYNC_LOG);
        unsafe {
            log::set_max_level(tk2l(asynclog.get_level()));
        }
        self
    }
}

impl log::Log for Log {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        unsafe { l2tk(metadata.level()) >= asynclog.get_level() }
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = l2tk(record.level());
            let args = record.args();
            let mut file = "";
            let mut line: u32 = 0;
            if self.is_file_line() {
                line = record.line().unwrap_or(0);
                file = record.file().unwrap_or("");
            }
            unsafe {
                asynclog.log(asynclog.fmt(level, file, line, arguments_to_string(args)));
            }
        }
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! async_log {
    ($msg:expr) => {
        let msg: &str = $msg;
        unsafe {
            crate::tklog::asynclog.print(msg).await;
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
            if $crate::tklog::asynclog.get_level() <= $level {
                let formatted_args: Vec<String> = vec![$(format!("{}", $arg)),*];
                let mut file = "";
                let mut line = 0;
                if $crate::tklog::asynclog.is_file_line() {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join(",");
                if  $crate::tklog::asynclog.mode==$crate::PRINTMODE::DELAY {
                    $crate::tklog::asynclog.log($crate::tklog::asynclog.fmt($level, file, line, msg));
                }else {
                    $crate::tklog::asynclog.safeprint($crate::tklog::asynclog.fmt($level, file, line, msg).as_str()).await;
                }
            }
        }
    };
    () => {};
}
