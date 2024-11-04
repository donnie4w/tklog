
### tklog is a high-performance structured logging library  for Rust  [[中文]](https://github.com/donnie4w/tklog/blob/main/README_ZH.md "[中文]")

###### tklog featuring ease-of-use, efficiency, and a rich feature suite. It supports functionalities such as console logging, file logging, both synchronous and asynchronous logging modes, alongside advanced capabilities like log slicing by time or size and compressed backup of log files.

#### Features
- Function support includes console logging, file logging, synchronous logging, asynchronous logging.
- Log level settings mirror those of the standard library: trace, debug, info, warn, error, fatal.
- Formatted output with customizable formats that can include log level flags, formatted timestamps, and log file locations.
- Log file slicing by time intervals: hourly, daily, or monthly.
- Log file slicing by specified file size.
- File rolling mechanism that automatically deletes older log files once a maximum backup count is reached to prevent excess logs from accumulating.
- Compression of archived backup log files.
- Supports the official log library standard API
- Independent log parameters can be set by module
- Independent log parameters can be set by log level
- The environment variable RUST_LOG is supported for setting the log level.

### [official website](https://tlnet.top/tklogen "official website")

### [Github](https://github.com/donnie4w/tklog "Github")

### [crates.io](https://crates.io/crates/tklog "crates.io")

------------

## Simple Usage Description

##### Use tklog

```rust
[dependencies]
tklog = "0.2.6"   #   "0.x.x" current version
```

The simplest way to use tklog involves direct macro calls:

```rust
use tklog::{trace, debug, error, fatal, info, warn};
fn testlog() {
    trace!("trace>>>>", "aaaaaaaaa", 1, 2, 3, 4);
    debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
    info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
    warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
    error!("error>>>>", "eeeeeeee", 1, 2, 3, 7);
    fatal!("fatal>>>>", "ffffffff", 1, 2, 3, 8);
}
```
###### By default, it will print console log, not files. Execution Result:

```
[TRACE] 2024-05-26 11:47:22 testlog.rs 27:trace>>>>,aaaaaaaaa,1,2,3,4
[DEBUG] 2024-05-26 11:47:22 testlog.rs 28:debug>>>>,bbbbbbbbb,1,2,3,5
[INFO] 2024-05-26 11:47:22 testlog.rs 29:info>>>>,ccccccccc,1,2,3,5
[WARN] 2024-05-26 11:47:22 testlog.rs 30:warn>>>>,dddddddddd,1,2,3,6
[ERROR] 2024-05-26 11:47:22 testlog.rs 31:error>>>>,eeeeeeee,1,2,3,7
[FATAL] 2024-05-26 11:47:22 testlog.rs 32:fatal>>>>,ffffffff,1,2,3,8
```

###### For initialization and customization, tklog furnishes methods to configure options such as console output, log levels, formatting styles, cutting strategies, and custom formatters.

```rust
use tklog::{
    sync::Logger,LEVEL, LOG,
    Format, MODE,
};

fn log_init() {
    LOG.set_console(true)       // Enables console logging
        .set_level(LEVEL::Info)  // Sets the log level; default is Debug
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)  // Defines structured log output with chosen details
        .set_cutmode_by_size("tklogsize.txt", 1<<20, 10, true)  // Cuts logs by file size (1 MB), keeps 10 backups, compresses backups
        .set_formatter("{level}{time} {file}:{message}\n");   // Customizes log output format; default is "{level}{time} {file}:{message}"
}
```
This illustrates global, singleton-style logging setup. Additionally, tklog facilitates custom multi-instance logging configurations, useful in systems requiring distinct logging structures across different components.

------------

### Multi-Instance Logging

`tklog` also accommodates multiple instances for scenarios that require distinct logging configurations. Each instance can possess its unique settings for console output, log level, file rotation, and a custom formatter.

```rust
use tklog::{
    debugs, errors, fatals, infos,
    sync::Logger,LEVEL, LOG,
    traces, warns, Format, MODE,
};
fn testmutlilog() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug) //Set the log level to Debug
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true)   //Split log files daily, keep up to 10 backups, and compress them
        .set_formatter("{message} | {time} {file}{level}\n");  //Customize the log structure's output format and additional content
    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();
    traces!(log, "traces>>>>", "AAAAAAAAA", 1, 2, 3, 4);
    debugs!(log, "debugs>>>>", "BBBBBBBBB", 1, 2, 3, 5);
    infos!(log, "infos>>>>", "CCCCCCCCC", 1, 2, 3, 5);
    warns!(log, "warns>>>>", "DDDDDDDDDD", 1, 2, 3, 6);
    errors!(log, "errors>>>>", "EEEEEEEE", 1, 2, 3, 7);
    fatals!(log, "fatals>>>>", "FFFFFFFF", 1, 2, 3, 8);
    thread::sleep(Duration::from_secs(1))
}
```

###### Execution Result

```
debugs>>>>,BBBBBBBBB,1,2,3,5 | 2024-05-26 14:13:25 testlog.rs 70[DEBUG]
infos>>>>,CCCCCCCCC,1,2,3,5 | 2024-05-26 14:13:25 testlog.rs 71[INFO]
warns>>>>,DDDDDDDDDD,1,2,3,6 | 2024-05-26 14:13:25 testlog.rs 72[WARN]
errors>>>>,EEEEEEEE,1,2,3,7 | 2024-05-26 14:13:25 testlog.rs 73[ERROR]
fatals>>>>,FFFFFFFF,1,2,3,8 | 2024-05-26 14:13:25 testlog.rs 74[FATAL]
```

###### Note: The structured log output above conforms to the format specified by "{message} | {time} {file}{level}\n". The formatter includes identifiers like {message}, {time}, {file}, {level}, and any additional text or separators outside these placeholders.

------------


## Detailed Usage Guide

#### 1. Log Levels: Trace < Debug < Info < Warn < Error < Fatal.

   Example:
```rust
   LOG.set_level(LEVEL::Info) //Sets the log level to Info
```

#### 2. Console Logging: Enable or disable via `.set_console(bool)`.

```rust
   LOG.set_console(false) //Disables console logging (default is true)
```

#### 3. Log Formats:

```rust
Format::Nano ： No formatting
Format::Date  ： Outputs date (e.g., 2024-05-26)
Format::Time  ： Outputs time to seconds (e.g., 14:13:25)
Format::Microseconds ：Outputs time with microseconds (e.g., 18:09:17.462245)
Format::LongFileName ：Full file path with line number (e.g., tests/testlog.rs 25)
Format::ShortFileName ： Abbreviated file path with line number (e.g., testlog.rs 25)
Format::LevelFlag ： Log level marker (e.g., [Debug]).
```

   For custom formats:

```rust
LOG.set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)
```

#### 4. Custom Format Strings:

 Default is "{level}{time} {file}:{message}\n".

-  `{level}`: Log level indicator, e.g., [Debug].

-  `{time}`: Logged timestamp.

-  `{file}`: Filename and line number.

- `{message}`: Log content.

######   Example:

```rust
   LOG.set_formatter("{message} | {time} {file}{level}\n")
```

   Reminder: Text outside the `{level}`, `{time}`, `{file}`, and `{message}` tags is output verbatim, including delimiters, spaces, and newlines.

####  5. Time-Based Log File Rotation:

   Modes: `MODE::HOUR`, `MODE::DAY`, `MODE::MONTH`.

   Use `.set_cutmode_by_time()` with:
   - File path
   - Time mode
   - Maximum backup count
   - Compression option

######   Example:

```rust
   let mut log = Logger::new(); 
   log.set_cutmode_by_time("/usr/local/tklogs.log", MODE::DAY, 0, false);
```

   This configures the log to be stored at `/usr/local/tklogs.log`, rotated daily, with no limit on backups, and without compressing daily logs.

**Backup Naming Conventions:**

- Daily: 
	- `tklogs_20240521.log`
	- `tklogs_20240522.log`
- Hourly: 
	- `tklogs_2024052110.log`
	- `tklogs_2024052211.log`
- Monthly:
	- `tklogs_202403.log`
	- `tklogs_202404.log`

#### 6. Size-Based Log File Rotation:

Utilize `.set_cutmode_by_size()` with the following parameters:

- File path
- Roll size
- Max backups
- Compress backups

######   Example:

```rust
let mut log = Logger::new(); 
log.set_cutmode_by_size("tklogs.log", 100<<20, 10, true);
```

Here, `tklogs.log` denotes the path, with files rolling at 100 MB each, retaining 10 backups, and compressing them.

**Backup File Naming Convention:**

```
tklogs_1.log.gz
tklogs_2.log.gz
tklogs_3.log.gz
```

**Log Printing Methods:**

- **Global Singleton:**
  - `trace!`, `debug!`, `info!`, `warn!`, `error!`, `fatal!`

- **Multiple Instances:**
  - `traces!`, `debugs!`, `infos!`, `warns!`, `errors!`, `fatals!`

**Asynchronous Logging**

- **Global Singleton Async:**
  - `async_trace!`, `async_debug!`, `async_info!`, `async_warn!`, `async_error!`, `async_fatal!`

- **Multiple Instances Async:**
  - `async_traces!`, `async_debugs!`, `async_infos!`, `async_warns!`, `async_errors!`, `async_fatals!`

**Example: Global Asynchronous Usage**

```rust
use tklog::{
    async_debug, async_error, async_fatal, async_info, async_trace, async_warn, LEVEL, Format, ASYNC_LOG
};

async fn async_log_init() {
    // Configure global singleton
    ASYNC_LOG
        .set_console(false) // Disable console output
        .set_level(LEVEL::Trace) // Set log level to Trace
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName) // Define structured logging output
        .set_cutmode_by_size("tklog_async.txt", 10000, 10, false) // Rotate log files by size, every 10,000 bytes, with 10 backups
        .await;
}

#[tokio::test]
async fn testlog() {
    async_log_init().await;
    async_trace!("trace>>>>", "aaaaaaa", 1, 2, 3);
    async_debug!("debug>>>>", "aaaaaaa", 1, 2, 3);
    async_info!("info>>>>", "bbbbbbbbb", 1, 2, 3);
    async_warn!("warn>>>>", "cccccccccc", 1, 2, 3);
    async_error!("error>>>>", "ddddddddddddd", 1, 2, 3);
    async_fatal!("fatal>>>>", "eeeeeeeeeeeeee", 1, 2, 3);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
```

###### Execution Result:

```text
[TRACE] 20:03:32 testasynclog.rs 20:trace>>>>,aaaaaaa,1,2,3
[DEBUG] 20:03:32 testasynclog.rs 21:debug>>>>,aaaaaaa,1,2,3
[INFO] 20:03:32 testasynclog.rs 22:info>>>>,bbbbbbbbb,1,2,3
[WARN] 20:03:32 testasynclog.rs 23:warn>>>>,cccccccccc,1,2,3
[ERROR] 20:03:32 testasynclog.rs 24:error>>>>,ddddddddddddd,1,2,3
[FATAL] 20:03:32 testasynclog.rs 25:fatal>>>>,eeeeeeeeeeeeee,1,2,3
```

###### Multiple Instance Asynchronous

```rust
use std::sync::Arc;

use tklog::{
    async_debugs, async_errors, async_fatals, async_infos, async_traces, async_warns, LEVEL, Format, ASYNC_LOG, MODE
};

#[tokio::test]
async fn testmultilogs() {
    let mut log = tklog::Async::Logger::new();
    log.set_console(false)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogasync.log", MODE::DAY, 10, true)
        .await
        .set_formatter("{message} | {time} {file}{level}\n");

    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();
    async_traces!(log, "async_traces>>>>", "AAAAAAAAAA", 1, 2, 3);
    async_debugs!(log, "async_debugs>>>>", "BBBBBBBBBB", 1, 2, 3);
    async_infos!(log, "async_infos>>>>", "CCCCCCCCCC", 1, 2, 3);
    async_warns!(log, "async_warns>>>>", "DDDDDDDDDD", 1, 2, 3);
    async_errors!(log, "async_errors>>>>", "EEEEEEEEEEE", 1, 2, 3);
    async_fatals!(log, "async_fatals>>>>", "FFFFFFFFFFFF", 1, 2, 3);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
```

###### Execution Result:

```text
async_debugs>>>>,BBBBBBBBBB,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 45[DEBUG]
async_infos>>>>,CCCCCCCCCC,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 46[INFO]
async_warns>>>>,DDDDDDDDDD,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 47[WARN]
async_errors>>>>,EEEEEEEEEEE,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 48[ERROR]
async_fatals>>>>,FFFFFFFFFFFF,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 49[FATAL]
```

------------


## Supports the official log library standard API

1.  tklog implements the regular use of the official Log interface API
2.  Implement the official log library API to be used in asynchronous scenarios

##### How to enable the official log library API： 

###### tklog enables API support for official logs by calling the `uselog()` function


###### Use example

```rust
use std::{thread, time::Duration};
use tklog::{Format, LEVEL, LOG};
fn test_synclog() {
    //init  LOG
    LOG.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_size("logsize.log", 10000, 10, true)
        .uselog();  //Enable the official log library
	
	log::trace!("trace>>>>{}{}{}{}{}", "aaaa", 1, 2, 3, 4);
	log::debug!("debug>>>>{}{}",1,2);
    log::info!("info log");
    log::warn!("warn log");
    log::error!("error log");
	thread::sleep(Duration::from_secs(1))
}
```


####  Enable the log library API in asynchronous scenarios

```rust
use std::{thread, time::Duration};
use tklog::{Format, LEVEL, ASYNC_LOG};
async fn test_synclog() {
    //init ASYNC  LOG 
    ASYNC_LOG.set_console(false)
        .set_cutmode_by_size("asynclogsize.log", 10000, 10, true).await
        .uselog(); //Enable the official log library
	
    log::trace!("trace async log>>>>{}{}{}{}{}", "aaaaaaaaa", 1, 2, 3, 4);
    log::debug!("debug async log>>>>{}{}",1,2);
	log::info!("info async log");
    log::warn!("warn async log");
    log::error!("error async log");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
```

------------


## The module sets  log parameters

1. tklog provides `set_option` and `set_mod_option` to set the global log parameters of the Logger object and specify the log parameters of the mod
2. In the project, you can use the global LOG object to set  log parameters for multiple mod at the same time
3. Different mod can set different log level, log formats, log file, etc
4. The log parameter of mod  for ASYNC_LOG is the same as LOG object


#####  `set_option` example：

	tklog::LOG.set_option(LogOption{level:Some(LEVEL::Debug),console: Some(false),format:None,formatter:None,fileoption: Some(Box::new(FileTimeMode::new("day.log",tklog::MODE::DAY,0,true)))});

##### LogOption instruction

- level      level of  log
- format    format of log
- formatter   user-defined log output format
- console    console log setting
- fileoption		file log setting


#####  `set_mod_option` example：

	tklog::LOG.set_mod_option("testlog::module1",LogOption{level:Some(LEVEL::Debug),console: Some(false),format:None,formatter:None,fileoption: Some(Box::new(FileTimeMode::new("day.log", tklog::MODE::DAY, 0,true)))});


- `testlog::module1` is the module name，you can use  `module_path!()`  to print out the current module name
- When tklog is used in the module `testlog::module1`, tklog will use the LogOption object

#####  `set_mod_option` example2：

	tklog::LOG.set_mod_option("testlog::*",LogOption{level:Some(LEVEL::Debug),console: Some(false),format:None,formatter:None,fileoption: Some(Box::new(FileTimeMode::new("day.log", tklog::MODE::DAY, 0,true)))});


-  `testlog::*`: tklog supports using * to match all submodules. testlog::* indicates all submodules of testlog.
- `testlog::module1::*` indicates all submodules of `testlog::module1`


#### Complete mod example

```rust
mod module1 {
    use std::{thread, time::Duration};
    use tklog::{handle::FileTimeMode, LogOption, LEVEL};
    pub fn testmod() {
        tklog::LOG.set_mod_option("testlog::module1", LogOption { level: Some(LEVEL::Debug), format: None, formatter: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("module1.log", tklog::MODE::DAY, 0, true))) });
        tklog::debug!("module1,tklog api,LOG debug log>>", 123);
        tklog::info!("module1,tklog api,LOG info log>>", 456);
        thread::sleep(Duration::from_secs(1))
    }
}

mod module2 {
    use std::{thread, time::Duration};
    use tklog::{handle::FileTimeMode, LogOption, LEVEL};
    pub fn testmod() {
        tklog::LOG.set_mod_option("testlog::module2::*", LogOption { level: Some(LEVEL::Info), format: None, formatter: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("module2.log", tklog::MODE::DAY, 0, true))) });
    }
    mod m2 {
        pub fn testmod() {
            tklog::debug!("module2,tklog api,LOG debug log>>", 123);
            tklog::info!("module2,tklog api,LOG info log>>", 456);
            thread::sleep(Duration::from_secs(1))
        }
    }
}

#[test]
fn testmod2() {
    module1::testmod();
    module2::m2::testmod();
}
```

##### Execution Result:

```text
[DEBUG] 2024-06-19 10:54:07 testlog.rs 54:module1,tklog api,LOG debug log>>,123
[INFO] 2024-06-19 10:54:07 testlog.rs 55:module1,tklog api,LOG info log>>,456
[DEBUG] 2024-06-19 10:54:07 testlog.rs 56:module1,log api,debug log>>111
[INFO] 2024-06-19 10:54:07 testlog.rs 57:module1,log api,info log>>222
[INFO] 2024-06-19 10:54:08 testlog.rs 68:module2,tklog api,LOG info log>>,456
[INFO] 2024-06-19 10:54:08 testlog.rs 70:module2,log api,info log>>222
```

#### Example 2: Asynchronous logging

```rust

mod module3 {
    use tklog::{handle::FileTimeMode, Format, LogOption, LEVEL};
    pub async fn testmod() {
        tklog::ASYNC_LOG.set_mod_option("testlog::module3", LogOption { level: Some(LEVEL::Debug), format: Some(Format::Date), formatter: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("module3.log", tklog::MODE::DAY, 0, true))) }).await.uselog();
        tklog::async_debug!("async module3,tklog api,LOG debug log>>", 123);
        tklog::async_info!("async module3,tklog api,LOG info log>>", 456);
        log::debug!("async module3,log api,debug log>>{}", 333);
        log::info!("async module3,log api,info log>>{}", 444);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

mod module4 {
    use tklog::{handle::FileTimeMode, Format, LogOption, LEVEL};
    pub async fn testmod() {
        tklog::ASYNC_LOG.set_mod_option("testlog::module4", LogOption { level: Some(LEVEL::Info), format: Some(Format::Date), formatter: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("module4.log", tklog::MODE::DAY, 0, true))) }).await.uselog();
        tklog::async_debug!("async module4,tklog api,LOG debug log>>", 123);
        tklog::async_info!("async module4,tklog api,LOG info log>>", 456);
        log::debug!("async module4,log api,debug log>>{}", 333);
        log::info!("async module4,log api,info log>>{}", 444);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[tokio::test]
async fn testmod4() {
    module3::testmod().await;
    module4::testmod().await;
}
```

##### Execution Result:

```text
[DEBUG] 2024-06-19 10:59:26 testlog.rs 85:async module3,tklog api,LOG debug log>>,123
[INFO] 2024-06-19 10:59:26 testlog.rs 86:async module3,tklog api,LOG info log>>,456
[DEBUG] 2024-06-19 10:59:26 testlog.rs 87:async module3,log api,debug log>>333
[INFO] 2024-06-19 10:59:26 testlog.rs 88:async module3,log api,info log>>444
[INFO] 2024-06-19 10:59:27 testlog.rs 98:async module4,tklog api,LOG info log>>,456
[INFO] 2024-06-19 10:59:27 testlog.rs 100:async module4,log api,info log>>444

```

------------

## tklog supports  multi-instance formatting format! And asynchronous format!

###### Example：

```rust
#[test]
fn testformats() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true);
    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();

    let v = vec![1, 2, 3];
    tklog::formats!(log, LEVEL::Debug, "Debug>>>{},{}>>>{:?}", 1, 2, v);

    let v2 = vec!['a', 'b'];
    tklog::formats!(log, LEVEL::Info, "Info>>>{},{}>>{:?}", 1, 2, v2);
    tklog::formats!(log, LEVEL::Warn, "Warn>>>{},{}", 1, 2);
    tklog::formats!(log, LEVEL::Error, "Error>>>{},{}", 1, 2);
    tklog::formats!(log, LEVEL::Fatal, "Fatal>>>{},{}", 1, 2);

    thread::sleep(Duration::from_secs(1))
}
```

###### Execution Result:

```rust
[DEBUG] 2024-06-06 15:54:07 testsynclog.rs 80:Debug>>>1,2>>>[1, 2, 3]
[INFO] 2024-06-06 15:54:07 testsynclog.rs 83:Info>>>1,2>>['a', 'b']
[WARN] 2024-06-06 15:54:07 testsynclog.rs 84:Warn>>>1,2
[ERROR] 2024-06-06 15:54:07 testsynclog.rs 85:Error>>>1,2
[FATAL] 2024-06-06 15:54:07 testsynclog.rs 86:Fatal>>>1,2
```

###### asynchronous Example

```rust
#[tokio::test]
async fn testformats() {
    let mut log = tklog::Async::Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogasyncs.log", MODE::DAY, 10, true)
        .await;
    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();

    let v = vec![1, 2, 3];
    tklog::async_formats!(log, LEVEL::Debug, "Debug>>>{},{}>>>{:?}", 1, 2, v);

    let v2 = vec!['a', 'b'];
    tklog::async_formats!(log, LEVEL::Info, "Info>>>{},{}>>{:?}", 1, 2, v2);
    tklog::async_formats!(log, LEVEL::Warn, "Warn>>>{},{}", 1, 2);
    tklog::async_formats!(log, LEVEL::Error, "Error>>>{},{}", 1, 2);
    tklog::async_formats!(log, LEVEL::Fatal, "Fatal>>>{},{}", 1, 2);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
```

###### Execution Result:

```rust
[DEBUG] 2024-06-06 16:09:26 testasynclog.rs 61:Debug>>>1,2>>>[1, 2, 3]
[INFO] 2024-06-06 16:09:26 testasynclog.rs 64:Info>>>1,2>>['a', 'b']
[WARN] 2024-06-06 16:09:26 testasynclog.rs 65:Warn>>>1,2
[ERROR] 2024-06-06 16:09:26 testasynclog.rs 66:Error>>>1,2
[FATAL] 2024-06-06 16:09:26 testasynclog.rs 67:Fatal>>>1,2
```

------

## tklog supports custom log processing functions.

###### tklog allows the addition of external custom functions through `set_custom_handler()`, enabling control over the log processing flow and logic.

###### Example

```rust
#[test]
fn test_custom() {
    fn custom_handler(lc: &LogContext) -> bool {
        println!("level >>>>>>>>>>>>>>>>>{:?}", lc.level);
        println!("message >>>>>>>>>>>>>>>>>{:?}", lc.log_body);
        println!("filename >>>>>>>>>>>>>>>>>{:?}", lc.filename);
        println!("line >>>>>>>>>>>>>>>>>{:?}", lc.line);
        println!("modname >>>>>>>>>>>>>>>>>{:?}", lc.modname);
        if lc.level == LEVEL::Debug {
            println!("{}", "debug now");
            return false;
        }
        true
    }

    LOG.set_custom_handler(custom_handler);
    debug!("000000000000000000");
    info!("1111111111111111111");
    thread::sleep(Duration::from_secs(1))
}
```

###### Execution Result

```rust
---- test_custom stdout ----
level >>>>>>>>>>>>>>>>>Debug
message >>>>>>>>>>>>>>>>>"000000000000000000"
filename >>>>>>>>>>>>>>>>>"tests\\testsynclog.rs"
line >>>>>>>>>>>>>>>>>143
modname >>>>>>>>>>>>>>>>>"testsynclog"
debug now
level >>>>>>>>>>>>>>>>>Info
message >>>>>>>>>>>>>>>>>"1111111111111111111"
filename >>>>>>>>>>>>>>>>>"tests\\testsynclog.rs"
line >>>>>>>>>>>>>>>>>144
modname >>>>>>>>>>>>>>>>>"testsynclog"
[INFO] 2024-08-05 15:39:07 testsynclog.rs 144:1111111111111111111
```

##### Explanation

When the function `fn custom_handler(lc: &LogContext) -> bool` returns `true`, **tklog** calls the `custom_handler` to execute the custom function and then continues with **tklog**'s logging process. When it returns `false`, **tklog** does not proceed with its logging process and directly returns. As shown in the example, when the log level is `Debug`, it returns `false`, so **tklog** does not print the `Debug` log.


## `tklog` Supports Custom Log Multi-Parameter Separators

###### `tklog` allows setting custom separators using the `set_separator()` method

The following Rust code demonstrates how to configure and use custom separators for log entries in the `tklog` framework:

```rust
#[test]
fn testlog() {
    log_init();
    trace!("trace>>>>", "aaaaaaaaa", 1, 2, 3, 4);
    debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
    LOG.set_separator("|");
    info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
    warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
    LOG.set_separator(",");
    error!("error>>>>", "eeeeeeee", 1, 2, 3, 7);
    fatal!("fatal>>>>", "ffffffff", 1, 2, 3, 8);
    thread::sleep(Duration::from_secs(1))
}
```

###### Execution Result

The output generated by the testlog function demonstrates the impact of setting different separators on the log messages:

```rust
---- testlog stdout ----
[TRACE] 2024-08-15 14:14:19.289590 tests\testsynclog.rs 22:trace>>>>aaaaaaaaa1234
[DEBUG] 2024-08-15 14:14:19.289744 tests\testsynclog.rs 23:debug>>>>bbbbbbbbb1235
[INFO] 2024-08-15 14:14:19.289761 tests\testsynclog.rs 25:info>>>>|ccccccccc|1|2|3|5
[WARN] 2024-08-15 14:14:19.289774 tests\testsynclog.rs 26:warn>>>>|dddddddddd|1|2|3|6
[ERROR] 2024-08-15 14:14:19.289789 tests\testsynclog.rs 28:error>>>>,eeeeeeee,1,2,3,7
[FATAL] 2024-08-15 14:14:19.289802 tests\testsynclog.rs 29:fatal>>>>,ffffffff,1,2,3,8
```

### tklog Supports Independent Logging Format Parameters for Different Log Levels

###### tklog sets independent logging parameters for different log levels via `set_level_option()`

###### `set_level_option()` accepts objects of any type that implements the `OptionTrait` trait

##### Example 1: Using `LevelOption` object to set log formatting

```rust
#[test]
fn testlog() {
    // Set the Info level log format to Format::LevelFlag
    // Set the Fatal level log format to Format::LevelFlag | Format::Date
    LOG.set_level_option(LEVEL::Info, LevelOption { format: Some(Format::LevelFlag), formatter: None })
    .set_level_option(LEVEL::Fatal, LevelOption { format: Some(Format::LevelFlag | Format::Date), formatter: None });

    trace!("this is trace log");
    debug!("this is debug log");
    info!("this is info log");
    warn!("this is warn log");
    error!("this is error log");
    fatal!("this is fatal log");
    thread::sleep(Duration::from_secs(1));
}
```

###### Execution Result

```rust
---- testlog stdout ----
[DEBUG] 2024-08-24 15:06:02 test_0100.rs 17:this is debug log
[INFO] this is info log
[WARN] 2024-08-24 15:06:02 test_0100.rs 19:this is warn log
[ERROR] 2024-08-24 15:06:02 test_0100.rs 20:this is error log
[FATAL] 2024-08-24 this is fatal log
```

##### Example 2: Using `LogOption` object to set more parameters including separate log files

```rust
#[test]
fn testlog() {
    LOG.set_level_option(LEVEL::Info, LogOption { format: None, formatter: None, level: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("0200time.log", tklog::MODE::DAY, 0, false))) })
    .set_level_option(LEVEL::Fatal, LogOption { format: None, formatter: None, level: None, console: None, fileoption: Some(Box::new(FileSizeMode::new("0200size.log", 1<<10, 0, false))) });

    trace!("this is trace log");
    debug!("this is debug log");
    info!("this is info log");
    warn!("this is warn log");
    error!("this is error log");
    fatal!("this is fatal log");
    thread::sleep(Duration::from_secs(1));
}
```
**Example description:** 

1. The file logs at the Info level are separated by day and the file name is 0200time.log
2. The file logs of the Fatal level are separated by size and the file name is 0200sisie.log

------------

## tklog supports formatting settings for log attribute identifiers

##### Set log identifier and time format through the `set_attr_format` function

##### Example:

```rust
fn testlog() {
    tklog::LOG.set_attr_format(|fmt| {
        fmt.set_level_fmt(|level| {
            match level {
                LEVEL::Trace => "[T]",
                LEVEL::Debug => "[D]",
                LEVEL::Info => "[I]",
                LEVEL::Warn => "[W]",
                LEVEL::Error => "[E]",
                LEVEL::Fatal => "[F]",
                LEVEL::Off => "",
            }.to_string()
        });

        fmt.set_time_fmt(|| {
            let now: DateTime<Local> = Local::now();
            (now.format("%Y/%m/%d").to_string(), now.format("%H:%M:%S").to_string(), "".to_string())
        });


        fmt.set_body_fmt(|level, body| {
            //Handles the last newline character of the body
            let trimmed_body = if body.ends_with('\n') {
                format!("{}{}", body.as_str()[..body.len() - 1].to_string(), "\x1b[0m\n")
            } else {
                format!("{}{}", body, "\x1b[0m\n")
            };

            match level {
                LEVEL::Trace => format!("{}{}", "\x1b[34m", trimmed_body), //blue
                LEVEL::Debug => format!("{}{}", "\x1b[36m", trimmed_body), //cyan
                LEVEL::Info => format!("{}{}", "\x1b[32m", trimmed_body),  //green
                LEVEL::Warn => format!("{}{}", "\x1b[33m", trimmed_body),  //yellow
                LEVEL::Error => format!("{}{}", "\x1b[31m", trimmed_body), //red
                LEVEL::Fatal => format!("{}{}", "\x1b[41m", trimmed_body), //background red
                LEVEL::Off => "".to_string(),
            }
        });
    });

    trace!("trace!", "this is sync log");
    debug!("debug!","this is sync log");
    info!("info!","this is sync log");
    warn!("warn!","this is sync log");
    error!("error!","this is sync log");
    fatal!("fata!","this is sync log");
    thread::sleep(Duration::from_secs(1))
}
```
###### Execution Result
```text
[D] 2024/10/17 19:41:20 test_0230.rs 32:debug!this is sync log
[I] 2024/10/17 19:41:20 test_0230.rs 33:info!this is sync log
[W] 2024/10/17 19:41:20 test_0230.rs 34:warn!this is sync log
[E] 2024/10/17 19:41:20 test_0230.rs 35:error!this is sync log
[F] 2024/10/17 19:41:20 test_0230.rs 36:fata!this is sync log
```

------------

## Benchmark Test


```test
log_benchmark           time:   [2.3949 µs 2.4428 µs 2.4941 µs]
                        change: [-0.5586% +1.9685% +4.4040%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

mod_benchmark           time:   [2.1946 µs 2.2325 µs 2.2718 µs]
                        change: [-2.5723% +0.0728% +2.8784%] (p = 0.96 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
```

```text
log_benchmark           time:   [2.3992 µs 2.4307 µs 2.4632 µs]
                        change: [-12.388% -9.7287% -6.8751%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe

mod_benchmark           time:   [2.2126 µs 2.2508 µs 2.2920 µs]
                        change: [-11.895% -9.0113% -6.2389%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```

```text
log_benchmark           time:   [2.4525 µs 2.5059 µs 2.5632 µs]
                        change: [-10.548% -7.0786% -3.6963%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

mod_benchmark           time:   [2.2603 µs 2.3113 µs 2.3693 µs]
                        change: [-12.539% -9.5519% -6.4982%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

```text
log_benchmark           time:   [2.5650 µs 2.6194 µs 2.6775 µs]
                        change: [-3.5311% -0.4742% +3.3119%] (p = 0.79 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

mod_benchmark           time:   [2.4908 µs 2.5655 µs 2.6440 µs]
                        change: [-1.3617% +1.9010% +5.2711%] (p = 0.29 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```

##### **log_benchmark**
| Test Number | Minimum Time (µs) | Maximum Time (µs) | Average Time (µs) | Percentage Change (%) | p-value |
|-------------|--------------------|--------------------|--------------------|-----------------------|---------|
| 1           | 2.3949             | 2.4941             | 2.4428             | -0.5586%              | 0.14    |
| 2           | 2.3992             | 2.4632             | 2.4307             | -12.388%              | 0.00    |
| 3           | 2.4525             | 2.5632             | 2.5059             | -10.548%              | 0.00    |
| 4           | 2.5650             | 2.6775             | 2.6194             | -3.5311%              | 0.79    |

##### **mod_benchmark**
| Test Number | Minimum Time (µs) | Maximum Time (µs) | Average Time (µs) | Percentage Change (%) | p-value |
|-------------|--------------------|--------------------|--------------------|-----------------------|---------|
| 1           | 2.1946             | 2.2718             | 2.2325             | -2.5723%              | 0.96    |
| 2           | 2.2126             | 2.2920             | 2.2508             | -11.895%              | 0.00    |
| 3           | 2.2603             | 2.3693             | 2.3113             | -12.539%              | 0.00    |
| 4           | 2.4908             | 2.6440             | 2.5655             | -1.3617%              | 0.29    |

#### 2. **Summary Statistics**
- **log_benchmark**
  - **Minimum Time**: 2.3949 µs
  - **Maximum Time**: 2.6775 µs
  - **Average Time**: 2.5160 µs
  - **Change Range**: From -0.5586% to -12.388%
  - **p-value**: Most tests show significant results (p < 0.05).

- **mod_benchmark**
  - **Minimum Time**: 2.1946 µs
  - **Maximum Time**: 2.6440 µs
  - **Average Time**: 2.3430 µs
  - **Change Range**: From -2.5723% to -12.539%
  - **p-value**: Most tests show significant results (p < 0.05).

### Performance Statistics (Response Times)
1. Minimum Time: 2.1946 µs
2. Maximum Time: 2.6775 µs
3. Average Time: 2.3946 µs
