
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

### [official website](https://tlnet.top/tklogen "official website")

------------

### Simple Usage Description

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
    sync::Logger,
    tklog::{LEVEL, LOG},
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
    sync::Logger,
    tklog::{LEVEL, LOG},
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

**Execution Result:**

```
debugs>>>>,BBBBBBBBB,1,2,3,5 | 2024-05-26 14:13:25 testlog.rs 70[DEBUG]
infos>>>>,CCCCCCCCC,1,2,3,5 | 2024-05-26 14:13:25 testlog.rs 71[INFO]
warns>>>>,DDDDDDDDDD,1,2,3,6 | 2024-05-26 14:13:25 testlog.rs 72[WARN]
errors>>>>,EEEEEEEE,1,2,3,7 | 2024-05-26 14:13:25 testlog.rs 73[ERROR]
fatals>>>>,FFFFFFFF,1,2,3,8 | 2024-05-26 14:13:25 testlog.rs 74[FATAL]
```

###### Note: The structured log output above conforms to the format specified by "{message} | {time} {file}{level}\n". The formatter includes identifiers like {message}, {time}, {file}, {level}, and any additional text or separators outside these placeholders.

------------


### Detailed Usage Guide

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

	-    `{level}`: Log level indicator, e.g., [Debug].
	-    `{time}`: Logged timestamp.
	-    `{file}`: Filename and line number.
	-    `{message}`: Log content.

   Example:

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

   Example:
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

Example:

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
    async_debug, async_error, async_fatal, async_info, async_trace, async_warn, tklog::LEVEL, Format, ASYNC_LOG
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

**Execution Result:**

```text
[TRACE] 20:03:32 testasynclog.rs 20:trace>>>>,aaaaaaa,1,2,3
[DEBUG] 20:03:32 testasynclog.rs 21:debug>>>>,aaaaaaa,1,2,3
[INFO] 20:03:32 testasynclog.rs 22:info>>>>,bbbbbbbbb,1,2,3
[WARN] 20:03:32 testasynclog.rs 23:warn>>>>,cccccccccc,1,2,3
[ERROR] 20:03:32 testasynclog.rs 24:error>>>>,ddddddddddddd,1,2,3
[FATAL] 20:03:32 testasynclog.rs 25:fatal>>>>,eeeeeeeeeeeeee,1,2,3
```

**Multiple Instance Asynchronous**

```rust
use std::sync::Arc;

use tklog::{
    async_debugs, async_errors, async_fatals, async_infos, async_traces, async_warns, tklog::LEVEL, Format, ASYNC_LOG, MODE
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

**Execution Result:**

```text
async_debugs>>>>,BBBBBBBBBB,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 45[DEBUG]
async_infos>>>>,CCCCCCCCCC,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 46[INFO]
async_warns>>>>,DDDDDDDDDD,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 47[WARN]
async_errors>>>>,EEEEEEEEEEE,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 48[ERROR]
async_fatals>>>>,FFFFFFFFFFFF,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 49[FATAL]
```

------------


### Benchmark Test

```text
test_debug              time:   [3.3747 µs 3.4599 µs 3.5367 µs]
                                change: [-69.185% -68.009% -66.664%] (p = 0.00 < 0.05)
                                Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
```
###### Explanation: The time range gives three data points representing the minimum test execution time (3.3747 microseconds), the value near the average (3.4599 microseconds), and the maximum (3.5367 microseconds).

```text
test_debug              time:   [3.8377 µs 3.8881 µs 3.9408 µs]
                               change: [-66.044% -65.200% -64.363%] (p = 0.00 < 0.05)
                               Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
###### Explanation: The test runs ranged from 3.8377 microseconds to 3.9408 microseconds, covering an approximate distribution where 3.8881 microseconds is approximately the average or median execution time over this period

**Conclusion: Log printing function performance: 3µs /op - 4µs /op (microsecond/time)**