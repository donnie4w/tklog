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
    arguments_to_string, handle::{FHandler, FileOptionType, FmtHandler}, l2tk, log_fmt, syncfile::FileHandler, tklog::synclog, trie::Trie, AttrFormat, Format, LogContext, LogOption, LogOptionConst, OptionTrait, LEVEL, MODE, PRINTMODE, TKLOG2SYNCLOG
};
use std::thread;
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Sender},
};

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
    sender: Sender<(LEVEL, String, String)>,
    fmthandle: FmtHandler,
    filehandle: (String, FHandler),
    mutex: std::sync::Mutex<u32>,
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
        let (sender, receiver) = channel();
        thread::spawn(move || {
            while let Ok(s) = receiver.recv() {
                let (level, module, msg) = s;
                let m1: String = module;
                let m2: String = msg;
                crate::log!(level, m1.as_str(), m2.as_str());
            }
        });
        Logger {
            sender,
            fmthandle: FmtHandler::new(),
            filehandle: ("".to_string(), FHandler::new()),
            mutex: std::sync::Mutex::new(0),
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

    pub fn print(&mut self, level: LEVEL, module: &str, message: &str) {
        let mut console = self.fmthandle.get_console();

        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, filename) = mm;
                if let Some(cs) = lo.console {
                    console = cs
                }
                if filename != "" {
                    if *filename == self.filehandle.0 {
                        let _ = self.filehandle.1.print(console, message);
                    } else {
                        if let Some(fm) = self.fmap.get_mut(filename) {
                            let _ = fm.print(console, message);
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
                    let _ = self.filehandle.1.print(console, message);
                } else {
                    if let Some(fm) = self.fmap.get_mut(filename) {
                        let _ = fm.print(console, message);
                    }
                }
                return;
            }
        }
        let _ = self.filehandle.1.print(console, message);
    }

    pub fn safeprint(&mut self, level: LEVEL, module: &str, message: &str) {
        let _guard = self.mutex.lock().expect("Failed to acquire lock");
        let mut console = self.fmthandle.get_console();
        if module != "" && self.modmap.len() > 0 {
            if let Some(mm) = self.modmap.get(module) {
                let (lo, filename) = mm;
                if let Some(cs) = lo.console {
                    console = cs
                }
                if filename != "" {
                    if *filename == self.filehandle.0 {
                        let _ = self.filehandle.1.print(console, message);
                    } else {
                        if let Some(fm) = self.fmap.get_mut(filename) {
                            let _ = fm.print(console, message);
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
                    let _ = self.filehandle.1.print(console, message);
                } else {
                    if let Some(fm) = self.fmap.get_mut(filename) {
                        let _ = fm.print(console, message);
                    }
                }
                return;
            }
        }
        let _ = self.filehandle.1.print(console, message);
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
        if let Some(ch) = &self.custom_handler {
            if !ch(&LogContext { level: level, filename: filename.to_string(), line: line, log_body: message.clone(), modname: module.to_string() }) {
                return String::new();
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
                if lo.formatter.is_some() {
                    formatter = lo.formatter.as_ref();
                }
            }
        }

        if let Some(lp) = &self.levels[level as usize - 1] {
            let (lo, _) = lp;
            if let Some(v) = lo.format {
                fmat = v;
            }
            if lo.formatter.is_some() {
                formatter = lo.formatter.as_ref();
            }
        }
        let s = log_fmt(self.attrfmt.levelfmt.as_ref(),self.attrfmt.timefmt.as_ref(),fmat, formatter, level, filename, line, message.as_str());
        if let Some(f) = &self.attrfmt.bodyfmt{
            f(level,s)
        }else{
            s
        }
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

    pub fn set_cutmode_by_size(&mut self, filename: &str, maxsize: u64, maxbackups: u32, compress: bool) -> &mut Self {
        let fsm = FileOptionType::new(crate::CUTMODE::SIZE, MODE::DAY, filename, maxsize, maxbackups, compress);
        let fh = FileHandler::new(Box::new(fsm));
        self.filehandle.0 = filename.to_string();
        self.filehandle.1.set_file_handler(fh.unwrap());
        self
    }

    pub fn set_cutmode_by_time(&mut self, filename: &str, mode: MODE, maxbackups: u32, compress: bool) -> &mut Self {
        let ftm = FileOptionType::new(crate::CUTMODE::TIME, mode, filename, 0, maxbackups, compress);
        let fh = FileHandler::new(Box::new(ftm));
        self.filehandle.0 = filename.to_string();
        self.filehandle.1.set_file_handler(fh.unwrap());
        self
    }

    pub fn set_option(&mut self, option: LogOption) -> &mut Self {
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
            match FileHandler::new(v) {
                Ok(f) => {
                    self.filehandle.0 = f.get_file_name();
                    self.filehandle.1.set_file_handler(f);
                }
                Err(_) => {}
            }
        }

        self
    }

    pub fn set_mod_option(&mut self, module: &str, option: LogOption) -> &mut Self {
        let mut filename = "".to_string();
        if let Some(v) = option.fileoption {
            match FileHandler::new(v) {
                Ok(f) => {
                    filename = f.get_file_name();
                    if filename != self.filehandle.0 && !self.fmap.contains_key(&filename) {
                        let mut fhandler = FHandler::new();
                        fhandler.set_file_handler(f);
                        self.fmap.insert(filename.clone(), fhandler);
                    }
                }
                Err(_) => {}
            }
        }
        self.modmap.insert(module, (LogOptionConst { level: option.level, format: option.format, formatter: option.formatter, console: option.console }, filename.clone()));
        self
    }

    pub fn set_custom_handler(&mut self, handler: fn(&LogContext) -> bool) -> &mut Self {
        self.custom_handler = Some(handler);
        self
    }

    pub fn set_level_option(&mut self, level: LEVEL, option: &dyn OptionTrait) -> &mut Self {
        let mut filename = "".to_string();
        if let Some(v) = option.get_fileoption() {
            match FileHandler::new(v) {
                Ok(f) => {
                    filename = f.get_file_name();
                    if filename != self.filehandle.0 && !self.fmap.contains_key(&filename) {
                        let mut fhandler = FHandler::new();
                        fhandler.set_file_handler(f);
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

    // pub fn set_levelfmt<F>(&mut self, levelfmt: F)
    // where
    //     F: Fn(LEVEL) -> String + Send + Sync + 'static,
    // {
    //     self.levelfmt = Some(Box::new(levelfmt));
    // }

    // pub fn set_timefmt<F>(&mut self, timefmt: F)
    // where
    //     F: Fn() -> (String, String, String) + Send + Sync + 'static,
    // {
    //     self.timefmt = Some(Box::new(timefmt));
    // }

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

    pub fn set_level_option(&self, level: LEVEL, option: impl OptionTrait) -> &Self {
        unsafe {
            synclog.set_level_option(level, &option);
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

    fn is_file_line(&self, level: LEVEL, module: &str) -> bool {
        unsafe {
            return synclog.is_file_line(level, module);
        }
    }

    pub fn uselog(&self) -> &Self {
        let _ = log::set_logger(&TKLOG2SYNCLOG);
        log::set_max_level(log::LevelFilter::Trace);
        self
    }

    // pub fn set_levelfmt<F>(&self, levelfmt: F)
    // where
    //     F: Fn(LEVEL) -> String + Send + Sync + 'static,
    // {
    //     unsafe { synclog.set_levelfmt(levelfmt) };
    // }

    // pub fn set_timefmt<F>(&self, timefmt: F)
    // where
    //     F: Fn() -> (String, String, String) + Send + Sync + 'static,
    // {
    //     unsafe { synclog.set_timefmt(timefmt) };
    // }

    pub fn set_attr_format<F>(&self, f: F)
    where
        F: FnMut(&mut AttrFormat) + Send + Sync + 'static,
    {
        unsafe {
            synclog.set_attr_format(f);
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
                if synclog.get_level(module) > level {
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
            if synclog.mode == PRINTMODE::DELAY {
                let s = synclog.fmt(module, level, file, line, arguments_to_string(args));
                if !s.is_empty() {
                    synclog.log(level, module.to_string(), s);
                }
            } else {
                let s = synclog.fmt(module, level, file, line, arguments_to_string(args));
                if !s.is_empty() {
                    synclog.safeprint(level, module, s.as_str());
                }
            }
        }
    }
    fn flush(&self) {}
}

#[macro_export]
macro_rules! log {
    ($level:expr, $module:expr,$msg:expr) => {
        let level: LEVEL = $level;
        let msg: &str = $msg;
        let module: &str = $module;
        unsafe {
            synclog.print(level, module, msg);
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
                if $crate::tklog::synclog.is_file_line($level,module) {
                    file = file!();
                    line = line!();
                }
                let msg: String = formatted_args.join($crate::tklog::synclog.get_separator().as_str());
                if  $crate::tklog::synclog.mode==$crate::PRINTMODE::DELAY {
                    let s = $crate::tklog::synclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::synclog.log($level,module.to_string(),s);
                    }
                }else {
                    let s = $crate::tklog::synclog.fmt(module,$level, file, line, msg);
                    if !s.is_empty(){
                        $crate::tklog::synclog.safeprint($level,module,s.as_str());
                    }
                }
            }
        }
    };
    () => {};
}
