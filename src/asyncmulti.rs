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

// Trace log macros, call secondary macro processing logic
#[macro_export]
macro_rules! async_traces {
    ($logger:expr, $($arg:expr),+) => {
        $crate::async_logs_common!($logger, $crate::LEVEL::Trace, $($arg),*);
    };
    () => {};
}

//Debug log macro, call secondary macro processing logic
#[macro_export]
macro_rules! async_debugs {
    ($logger:expr, $($arg:expr),+) => {
        $crate::async_logs_common!($logger, $crate::LEVEL::Debug, $($arg),*);
    };
    () => {};
}

//Info log macro, call secondary macro processing logic
#[macro_export]
macro_rules! async_infos {
    ($logger:expr, $($arg:expr),+) => {
        $crate::async_logs_common!($logger, $crate::LEVEL::Info, $($arg),*);
    };
    () => {};
}

// warn log macro, call secondary macro processing logic
#[macro_export]
macro_rules! async_warns {
    ($logger:expr, $($arg:expr),+) => {
        $crate::async_logs_common!($logger, $crate::LEVEL::Warn, $($arg),*);
    };
    () => {};
}

// Error log macro, call secondary macro processing logic
#[macro_export]
macro_rules! async_errors {
    ($logger:expr, $($arg:expr),+) => {
        $crate::async_logs_common!($logger, $crate::LEVEL::Error, $($arg),*);
    };
    () => {};
}

// Fatal log macros, call secondary macro processing logic
#[macro_export]
macro_rules! async_fatals {
    ($logger:expr, $($arg:expr),+) => {
        $crate::async_logs_common!($logger, $crate::LEVEL::Fatal, $($arg),*);
    };
    () => {};
}

#[macro_export]
macro_rules! async_formats {
    ($logger:expr, $level:expr, $($arg:expr),*) => {
        unsafe {
            let logger_lock:&mut Arc<tokio::sync::Mutex<tklog::Async::Logger>> = $logger;
            let mut logger = logger_lock.as_ref().lock().await;
            let level:$crate::LEVEL = $level;
            let module = module_path!();
            if logger.get_level(module) <= level {
                let mut file = "";
                let mut line = 0;
                if logger.is_file_line(module) {
                    file = file!();
                    line = line!();
                }
                let ss = logger.fmt(module,$level, file, line, format!($($arg),*));
                if !ss.is_empty(){
                    logger.print(module,ss.as_str()).await;
                }
            }
        }
    };
    () => {};
}

#[macro_export]
macro_rules! async_logs_common {
    ($logger:expr, $level:expr, $($arg:expr),*) => {
        unsafe {
            let logger_lock:&mut Arc<tokio::sync::Mutex<tklog::Async::Logger>> = $logger;
            let mut logger = logger_lock.as_ref().lock().await;
            let module = module_path!();
            if logger.get_level(module) <= $level {
                let formatted_args: Vec<String> = vec![$(format!("{}", $arg)),*];
                let mut file = "";
                let mut line = 0;
                if logger.is_file_line(module) {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join(logger.get_separator().as_str());
                let ss = logger.fmt(module,$level, file, line, msg);
                if !ss.is_empty(){
                    logger.print(module,ss.as_str()).await;
                }
            }
        }
    };
    () => {};
}
