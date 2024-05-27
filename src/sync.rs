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
    handle::{FileSizeMode, FileTimeMode, Handle, Handler},
    syncfile::FileHandler,
    tklog::{synclog, LEVEL, PRINTMODE},
    MODE,
};
use crossbeam_channel::{unbounded, Sender};
use std::sync::Mutex;
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
/// use tklog::tklog::{LEVEL, synclog};
/// use tklog::Format;
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
    sender: Sender<String>,
    #[allow(unused)]
    worker_thread: thread::JoinHandle<()>,
    loghandle: Handle,
    mutex: Mutex<u32>,
    pub mode: PRINTMODE,
}

impl Logger {
    pub fn new() -> Self {
        Self::new_with_handle(Handle::new(None))
    }

    pub fn new_with_handle(handle: Handle) -> Self {
        let (sender, receiver) = unbounded();
        let worker_thread = thread::spawn(move || loop {
            match receiver.recv() {
                Ok(task) => {
                    let msg: String = task;
                    crate::log!(msg.as_str());
                }
                Err(_) => {
                    break;
                }
            }
        });
        Logger {
            sender: sender,
            worker_thread,
            loghandle: handle,
            mutex: Mutex::new(0),
            mode: PRINTMODE::DELAY,
        }
    }

    pub fn print(&mut self, message: &str) {
        let _ = self.loghandle.print(message);
    }

    pub fn safeprint(&mut self, message: &str) {
        let _ = &self.mutex.lock();
        let _ = self.loghandle.print(message);
    }

    pub fn log(&self, message: String) {
        self.sender.send(message).expect("send error");
    }

    pub fn get_level(&self) -> LEVEL {
        return self.loghandle.get_level();
    }

    pub fn set_handler(&mut self, handler: Handler) {
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

    pub fn set_cutmode_by_size(
        &mut self,
        filename: &str,
        maxsize: u64,
        maxbackups: u32,
        compress: bool,
    ) -> &mut Self {
        let fsm = FileSizeMode::new(filename, maxsize, maxbackups, compress);
        let fh = FileHandler::new(Box::new(fsm));
        self.loghandle.handler.set_file_handler(fh.unwrap());
        self
    }

    pub fn set_cutmode_by_time(
        &mut self,
        filename: &str,
        mode: MODE,
        maxbackups: u32,
        compress: bool,
    ) -> &mut Self {
        let ftm = FileTimeMode::new(filename, mode, maxbackups, compress);
        let fh = FileHandler::new(Box::new(ftm));
        self.loghandle.handler.set_file_handler(fh.unwrap());
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

    pub fn set_cutmode_by_size(
        &self,
        filename: &str,
        maxsize: u64,
        maxbackups: u32,
        compress: bool,
    ) -> &Self {
        unsafe {
            synclog.set_cutmode_by_size(filename, maxsize, maxbackups, compress);
        }
        self
    }

    pub fn set_cutmode_by_time(
        &self,
        filename: &str,
        mode: MODE,
        maxbackups: u32,
        compress: bool,
    ) -> &Self {
        unsafe {
            synclog.set_cutmode_by_time(filename, mode, maxbackups, compress);
        }
        self
    }
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        let msg: &str = $msg;
        unsafe {
            synclog.print(msg);
        }
    };
}

#[macro_export]
macro_rules! trace {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::tklog::LEVEL::Trace, $($arg),*);
    };
}

#[macro_export]
macro_rules! debug {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::tklog::LEVEL::Debug, $($arg),*);
    };
}

#[macro_export]
macro_rules! info {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::tklog::LEVEL::Info, $($arg),*);
    };
}

#[macro_export]
macro_rules! warn {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::tklog::LEVEL::Warn, $($arg),*);
    };
}

#[macro_export]
macro_rules! error {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::tklog::LEVEL::Error, $($arg),*);
    };
}

#[macro_export]
macro_rules! fatal {
    () => {};
    ($($arg:expr),*) => {
        $crate::log_common!($crate::tklog::LEVEL::Fatal, $($arg),*);
    };
}

#[macro_export]
macro_rules! log_common {
    ($level:expr, $($arg:expr),*) => {
        unsafe {
            if $crate::tklog::synclog.get_level() <= $level {
                let formatted_args: Vec<String> = vec![$(format!("{}", $arg)),*];
                let mut file = "";
                let mut line = 0;
                if $crate::tklog::synclog.is_file_line() {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join(",");
                if  $crate::tklog::synclog.mode==$crate::tklog::PRINTMODE::DELAY {
                    $crate::tklog::synclog.log($crate::tklog::synclog.fmt($level, file, line, msg));
                }else {
                    $crate::tklog::synclog.safeprint($crate::tklog::synclog.fmt($level, file, line, msg).as_str());
                }
            }
        }
    };
    () => {};
}
