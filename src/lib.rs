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

use std::{
    env,
    fmt::{self, Debug},
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use chrono::{DateTime, Datelike, Local, NaiveDateTime, Timelike};
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use handle::FileOptionType;
use once_cell::sync::Lazy;
use tokio::io::AsyncReadExt;
#[allow(non_snake_case)]
pub mod Async;
pub mod asyncfile;
pub mod asyncmulti;
pub mod handle;
mod mwrite;
pub mod sync;
pub mod syncfile;
pub mod syncmulti;
#[allow(non_snake_case)]
mod threadPool;
mod trie;
pub enum DateType {
    Date,
    Time,
    Microseconds,
}

pub trait OptionTrait {
    fn get_level(&self) -> Option<LEVEL>;
    fn get_format(&self) -> Option<u8>;
    fn get_formatter(&self) -> Option<String>;
    fn get_console(&self) -> Option<bool>;
    fn get_fileoption(&self) -> Option<Box<dyn handle::FileOption>>;
}

pub struct LogOption {
    pub level: Option<LEVEL>,
    pub format: Option<u8>,
    pub formatter: Option<String>,
    pub console: Option<bool>,
    pub fileoption: Option<Box<dyn handle::FileOption>>,
}

impl LogOption {
    pub fn new() -> Self {
        LogOption { level: None, format: None, formatter: None, console: None, fileoption: None }
    }

    pub fn set_format(&mut self, f: u8) -> &mut Self {
        self.format = Some(f);
        self
    }

    pub fn set_formatter(&mut self, f: String) -> &mut Self {
        self.formatter = Some(f);
        self
    }

    pub fn set_level(&mut self, level: LEVEL) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn set_console(&mut self, console: bool) -> &mut Self {
        self.console = Some(console);
        self
    }

    pub fn set_fileoption(&mut self,  h:impl handle::FileOption + 'static) -> &mut Self {
        self.fileoption = Some(Box::new(h));
        self
    }

    pub fn take(&mut self) -> Self {
        LogOption {
            level: self.level.take(),
            format: self.format.take(),
            formatter: self.formatter.take(),
            console: self.console.take(),
            fileoption: self.fileoption.take(),
        }
    }
}

impl OptionTrait for LogOption {
    fn get_level(&self) -> Option<LEVEL> {
        Some(LEVEL::Trace)
    }

    fn get_format(&self) -> Option<u8> {
        self.format
    }

    fn get_formatter(&self) -> Option<String> {
        self.formatter.clone()
    }

    fn get_console(&self) -> Option<bool> {
        self.console
    }

    fn get_fileoption(&self) -> Option<Box<dyn handle::FileOption>> {
        if let Some(fo) = &self.fileoption {
            return Some(Box::new(FileOptionType { mode: fo.mode(), timemode: fo.timemode(), filename: fo.filename().clone(), size: fo.size(), maxbackups: fo.maxbackups(), compress: fo.compress() }));
        }
        None
    }
}

#[derive(Clone)]
pub struct LogOptionConst {
    pub level: Option<LEVEL>,
    pub format: Option<u8>,
    pub formatter: Option<String>,
    pub console: Option<bool>,
}

#[derive(Clone)]
pub struct LogContext {
    pub level: LEVEL,
    pub log_body: String,
    pub filename: String,
    pub line: u32,
    pub modname: String,
}

pub struct LevelOption {
    pub format: Option<u8>,
    pub formatter: Option<String>,
}

impl OptionTrait for LevelOption {
    fn get_level(&self) -> Option<LEVEL> {
        Some(LEVEL::Trace)
    }

    fn get_format(&self) -> Option<u8> {
        self.format
    }

    fn get_formatter(&self) -> Option<String> {
        self.formatter.clone()
    }

    fn get_console(&self) -> Option<bool> {
        None
    }

    fn get_fileoption(&self) -> Option<Box<dyn handle::FileOption>> {
        None
    }
}

#[allow(non_upper_case_globals, non_snake_case)]
pub mod Format {
    pub const Nano: u8 = 0;
    pub const Date: u8 = 1;
    pub const Time: u8 = 2;
    pub const Microseconds: u8 = 4;
    pub const LongFileName: u8 = 8;
    pub const ShortFileName: u8 = 16;
    pub const LevelFlag: u8 = 32;
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
enum ErrCode {
    NotFound,
}

impl ErrCode {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

// const DEFAULT_FORMATTER: &str = "{level}{time} {file}:{message}\n";
// const MWRITE: Lazy<mwrite::MWrite> = Lazy::new(|| mwrite::MWrite::new());

pub const LOG: Lazy<sync::Log> = Lazy::new(|| sync::Log::new());

static TKLOG2SYNCLOG: sync::Log = sync::Log;

pub const ASYNC_LOG: Lazy<Async::Log> = Lazy::new(|| Async::Log::new());

static TKLOG2ASYNC_LOG: Async::Log = Async::Log;

#[allow(non_upper_case_globals)]
pub mod tklog {
    use crate::{sync, Async};
    use once_cell::sync::Lazy;

    pub static mut synclog: Lazy<sync::Logger> = Lazy::new(|| sync::Logger::new());
    pub static mut asynclog: Lazy<Async::Logger> = Lazy::new(|| Async::Logger::new());
}

#[derive(PartialEq, PartialOrd)]
pub enum PRINTMODE {
    DELAY,
    PUNCTUAL,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
#[repr(u8)]
pub enum LEVEL {
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warn = 4,
    Error = 5,
    Fatal = 6,
    Off = 7,
}

impl FromStr for LEVEL {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fatal" => Ok(LEVEL::Fatal),
            "error" => Ok(LEVEL::Error),
            "warn" => Ok(LEVEL::Warn),
            "info" => Ok(LEVEL::Info),
            "debug" => Ok(LEVEL::Debug),
            "trace" => Ok(LEVEL::Trace),
            "off" => Ok(LEVEL::Off),
            _ => Err(()),
        }
    }
}

fn env_level() -> LEVEL {
    if let Ok(rust_log) = env::var("RUST_LOG") {
        match LEVEL::from_str(&rust_log) {
            Ok(level) => level,
            Err(_) => {
                println!("Warning: Unknown log level '{}', defaulting to Debug", rust_log);
                LEVEL::Debug
            }
        }
    } else {
        LEVEL::Debug
    }
}

pub enum COLUMN {
    LOGFLAG,
    TIME,
    FILEFLAG,
    COLON,
    MESSAGE,
}

#[derive(Copy, Clone)]
pub enum MODE {
    HOUR,
    DAY,
    MONTH,
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum CUTMODE {
    TIME,
    SIZE,
}

// fn timenow() -> (String, String, String) {
//     let now: DateTime<Local> = Local::now();
//     (now.format("%Y-%m-%d").to_string(), now.format("%H:%M:%S").to_string(), now.format("%.6f").to_string())
// }

// fn timenow() -> Vec<String> {
//     let now: DateTime<Local> = Local::now();
//     let full_format = now.format("%Y-%m-%d|%H:%M:%S|%.6f").to_string();
//     full_format.split('|').map(|s| s.to_string()).collect()
// }

fn now() -> DateTime<Local> {
    Local::now()
}

fn datefmt(now: DateTime<Local>) -> String {
    format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day())
}

fn datetimefmt(now: DateTime<Local>) -> String {
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

fn microsecondfmt(now: DateTime<Local>) -> String {
    format!("{:06}", now.nanosecond() / 1_000)
}

#[allow(dead_code)]
fn zlib(filename: &str) -> io::Result<()> {
    let input_file = File::open(filename)?;
    let mut reader = BufReader::new(input_file);
    let mut input_data = Vec::new();
    reader.read_to_end(&mut input_data)?;
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&input_data)?;
    let compressed_data = e.finish()?;
    let output_filename = format!("{}.zlib", filename);
    let output_file = File::create(&output_filename)?;
    let mut writer = BufWriter::new(output_file);
    let ack = writer.write_all(&compressed_data);
    if ack.is_ok() {
        let _ = fs::remove_file(filename);
    }
    Ok(())
}

fn gzip(filename: &str) -> io::Result<()> {
    let mut input_file = File::open(filename)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    io::copy(&mut input_file, &mut encoder)?;
    let compressed_data = encoder.finish()?;
    let output_filename = format!("{}.gz", filename);
    let mut output_file = File::create(&output_filename)?;
    let ack = output_file.write_all(&compressed_data);
    if ack.is_ok() {
        let _ = fs::remove_file(filename);
    }
    Ok(())
}

async fn async_gzip(filename: &str) -> io::Result<()> {
    let mut input_file = tokio::fs::File::open(filename).await?;
    let mut file_content = Vec::new();
    input_file.read_to_end(&mut file_content).await?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write_all(&file_content);
    let compressed_data = encoder.finish()?;
    let output_filename = format!("{}.gz", filename);
    let mut output_file = tokio::fs::File::create(output_filename).await?;
    tokio::io::AsyncWriteExt::write_all(&mut output_file, &compressed_data).await?;
    let _ = tokio::fs::remove_file(filename).await?;
    Ok(())
}

fn parse_and_format_log(format_str: &str, level: &str, time: &str, file: &str, message: &str) -> String {
    let mut result = String::with_capacity(format_str.len() + level.len() + time.len() + file.len() + message.len());
    let mut in_placeholder = false;
    let mut placeholder = String::new();

    for c in format_str.chars() {
        if in_placeholder {
            if c == '}' {
                in_placeholder = false;
                match placeholder.as_str() {
                    "level" => result.push_str(level),
                    "time" => result.push_str(time),
                    "file" => result.push_str(file),
                    "message" => result.push_str(message),
                    _ => (),
                }
                placeholder.clear();
            } else {
                placeholder.push(c);
            }
        } else if c == '{' {
            in_placeholder = true;
        } else {
            result.push(c);
        }
    }
    result
}

fn getbackup_with_time(startsec: u64, timemode: MODE) -> String {
    let start_time = DateTime::from_timestamp(startsec as i64, 0).expect("");
    match timemode {
        MODE::HOUR => {
            let formatted_time = start_time.format("%Y%m%d%H");
            formatted_time.to_string()
        }
        MODE::DAY => {
            let formatted_time = start_time.format("%Y%m%d");
            formatted_time.to_string()
        }
        MODE::MONTH => {
            let formatted_date = start_time.format("%Y%m");
            formatted_date.to_string()
        }
    }
}

fn log_fmt<LF, TF>(levelfmt: Option<LF>, timefmt: Option<TF>, fmat: u8, formatter: Option<&String>, level: LEVEL, filename: &str, line: u32, msg: &str) -> String
where
    LF: Fn(LEVEL) -> String,
    TF: Fn() -> (String, String, String),
{
    if fmat == Format::Nano {
        return msg.to_string();
    }

    let mut levelflag = String::new();
    let mut time = String::new();
    let mut file = String::new();

    if fmat & Format::LevelFlag != 0 {
        if let Some(f) = levelfmt {
            levelflag = f(level);
        } else {
            levelflag = match level {
                LEVEL::Trace => "[TRACE]",
                LEVEL::Debug => "[DEBUG]",
                LEVEL::Info => "[INFO]",
                LEVEL::Warn => "[WARN]",
                LEVEL::Error => "[ERROR]",
                LEVEL::Fatal => "[FATAL]",
                LEVEL::Off => "",
            }
            .to_string();
        }
    }

    if fmat & (Format::Date | Format::Time | Format::Microseconds) != 0 {
        let mut tss: (String, String, String);
        if let Some(f) = timefmt {
            tss = f();
            if fmat & Format::Date != 0 {
                tss.0.clear();
            }
            if fmat & Format::Time != 0 {
                tss.1.clear();
            }
            if fmat & Format::Microseconds != 0 {
                tss.2.clear();
            }
        } else {
            let localtime = now();
            tss = (String::new(), String::new(), String::new());
            if fmat & Format::Date != 0 {
                tss.0 = datefmt(localtime);
            }
            if fmat & (Format::Time | Format::Microseconds) != 0 {
                tss.1 = datetimefmt(localtime);
                if fmat & Format::Microseconds != 0 {
                    tss.2 = microsecondfmt(localtime);
                }
            }
        }
        if !tss.0.is_empty() {
            time.push_str(tss.0.as_str());
        }

        if !tss.1.is_empty() {
            if !time.is_empty() {
                time.push(' ');
            }
            time.push_str(tss.1.as_str());
        }

        if !tss.2.is_empty() {
            time.push_str(tss.2.as_str());
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

    if formatter.is_none() {
        let mut r = String::with_capacity(levelflag.len() + time.len() + file.len() + msg.len() + 4);
        if !levelflag.is_empty() {
            r.push_str(&levelflag);
        }
        if !time.is_empty() {
            r.push(' ');
            r.push_str(&time);
        }
        r.push(' ');
        if !file.is_empty() {
            r.push_str(&file);
            r.push(':');
        }

        r.push_str(&msg);
        r.push('\n');
        return r;
    } else {
        let fmts = formatter.unwrap();
        return parse_and_format_log(fmts.as_str(), levelflag.as_str(), time.as_str(), file.as_str(), msg);
    }
}

fn get_short_file_path(filename: &str) -> &str {
    let mut pos = None;
    for (i, c) in filename.char_indices().rev() {
        if c == '\\' || c == '/' {
            pos = Some(i);
            break;
        }
    }
    match pos {
        Some(index) => &filename[index + 1..],
        None => filename,
    }
}

fn timesec() -> u64 {
    let now: NaiveDateTime = Local::now().naive_local();
    return now.and_utc().timestamp() as u64;
}

fn passtimemode(startsec: u64, timemode: MODE) -> bool {
    let start_time = DateTime::from_timestamp(startsec as i64, 0).expect("");
    let now: NaiveDateTime = Local::now().naive_local();
    match timemode {
        MODE::HOUR => return now.hour() > start_time.hour(),
        MODE::DAY => return now.day() > start_time.day(),
        MODE::MONTH => return now.month() > start_time.month(),
    }
}

fn l2tk(level: log::Level) -> LEVEL {
    match level {
        log::Level::Error => LEVEL::Error,
        log::Level::Warn => LEVEL::Warn,
        log::Level::Info => LEVEL::Info,
        log::Level::Debug => LEVEL::Debug,
        log::Level::Trace => LEVEL::Trace,
    }
}

fn arguments_to_string(args: &std::fmt::Arguments) -> String {
    fmt::format(*args)
}

pub struct AttrFormat {
    levelfmt: Option<Box<dyn Fn(LEVEL) -> String + Send + Sync>>,
    timefmt: Option<Box<dyn Fn() -> (String, String, String) + Send + Sync>>,
    bodyfmt: Option<Box<dyn Fn(LEVEL, String) -> String + Send + Sync>>,
}

impl AttrFormat {
    pub fn new() -> AttrFormat {
        AttrFormat { levelfmt: None, timefmt: None, bodyfmt: None }
    }

    /// ### Exmaple
    /// ```rust
    /// set_level_fmt(|level| {
    ///     let s = match level {
    ///         LEVEL::Trace => "[T]",
    ///         LEVEL::Debug => "[D]",
    ///         LEVEL::Info => "[I]",
    ///         LEVEL::Warn => "[W]",
    ///         LEVEL::Error => "[E]",
    ///         LEVEL::Fatal => "[F]",
    ///         LEVEL::Off => "",
    ///     };
    ///     s.to_string()
    /// });
    /// ```
    pub fn set_level_fmt<F>(&mut self, levelfmt: F)
    where
        F: Fn(LEVEL) -> String + Send + Sync + 'static,
    {
        self.levelfmt = Some(Box::new(levelfmt));
    }

    /// - This function splits a date into three parts and returns a tuple (String, String, String).
    /// - You can customize the data for these three parts.
    /// #### Generally,
    /// - the first part is the date in the format `%Y-%m-%d`,
    /// - the second part is the time in the format `%H:%M:%S`,
    /// - and the third part is the fractional seconds, such as `6f`.
    /// ### Example
    /// ```rust
    /// set_time_fmt(|| {
    ///     let now: DateTime<Local> = Local::now();
    ///     (now.format("%Y-%m-%d").to_string(), now.format("%H:%M:%S").to_string(), ".6f".to_string())
    /// });
    /// ```
    pub fn set_time_fmt<F>(&mut self, timefmt: F)
    where
        F: Fn() -> (String, String, String) + Send + Sync + 'static,
    {
        self.timefmt = Some(Box::new(timefmt));
    }

    /// ### This function will support the reprocessing of log information
    ///
    /// ### Example
    /// ```rust
    /// fmt.set_body_fmt(|level,body| {
    ///     match level {
    ///         LEVEL::Trace =>  format!("{}{}{}","\x1b[34m" ,body,"\x1b[0m") , //blue
    ///         LEVEL::Debug => format!("{}{}{}","\x1b[36m" ,body,"\x1b[0m") , //cyan
    ///         LEVEL::Info => format!("{}{}{}","\x1b[32m" ,body,"\x1b[0m") ,  //green
    ///         LEVEL::Warn => format!("{}{}{}","\x1b[33m" ,body,"\x1b[0m") ,  //yellow
    ///         LEVEL::Error => format!("{}{}{}","\x1b[31m" ,body,"\x1b[0m") , //red
    ///         LEVEL::Fatal => format!("{}{}{}","\x1b[41m" ,body,"\x1b[0m") , //red-background
    ///         LEVEL::Off => "".to_string(),
    ///     }
    /// });
    /// ```
    pub fn set_body_fmt<F>(&mut self, bodyfmt: F)
    where
        F: Fn(LEVEL, String) -> String + Send + Sync + 'static,
    {
        self.bodyfmt = Some(Box::new(bodyfmt));
    }
}
