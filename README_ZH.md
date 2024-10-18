
### tklog 是一款 Rust 语言编写的高性能结构化日志库。[[English]](https://github.com/donnie4w/tklog/blob/main/README.md "[English]")

`易用`，`高效`，`结构化`，`控制台日志`，`文件日志`，`文件切割`，`文件压缩`，`同步打印`，`异步打印`

#####   特性

- 功能：控制台日志、文件日志、同步打印、异步打印
- 日志级别配置灵活：支持 `trace`、`debug`、`info`、`warn`、`error`、`fatal` 级别的日志输出
- 格式化输出自定义：可调整日志输出格式，涵盖日志级别标签、时间格式、文件位置等元素
- 按时间切割日志文件：支持按小时、天、月进行日志文件分割
- 按大小切割日志文件：根据文件大小自动分割
- 文件数管理：可设定最大备份文件数，自动删除旧日志，避免过多日志文件累积
- 文件压缩功能：支持对归档日志文件进行压缩
- 支持官方日志库标准API
- 支持按模块设置独立日志参数
- 支持按日志级别设置独立日志参数

### [官网](https://tlnet.top/tklog "官网")

### [Github](https://github.com/donnie4w/tklog "Github")

### [仓库](https://crates.io/crates/tklog "仓库")

## 使用方法简述

##### Use tklog

```rust
[dependencies]
tklog = "0.2.3"   #   "0.x.x" current version
```

最简单常用的方式：**直接调用**

```rust
use tklog::{trace,debug, error, fatal, info,warn};
fn testlog() {
    trace!("trace>>>>", "aaaaaaaaa", 1, 2, 3, 4);
    debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
    info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
    warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
    error!("error>>>>", "eeeeeeee", 1, 2, 3, 7);
    fatal!("fatal>>>>", "ffffffff", 1, 2, 3, 8);
}
```
###### 说明：默认打开控制台日志，没有写日志文件。打印结果：

```
[TRACE] 2024-05-26 11:47:22 testlog.rs 27:trace>>>>,aaaaaaaaa,1,2,3,4
[DEBUG] 2024-05-26 11:47:22 testlog.rs 28:debug>>>>,bbbbbbbbb,1,2,3,5
[INFO] 2024-05-26 11:47:22 testlog.rs 29:info>>>>,ccccccccc,1,2,3,5
[WARN] 2024-05-26 11:47:22 testlog.rs 30:warn>>>>,dddddddddd,1,2,3,6
[ERROR] 2024-05-26 11:47:22 testlog.rs 31:error>>>>,eeeeeeee,1,2,3,7
[FATAL] 2024-05-26 11:47:22 testlog.rs 32:fatal>>>>,ffffffff,1,2,3,8
```

###### 说明：直接调用 debug！等宏进行打印，默认调用全局静态LOG对象。LOG对象支持初始化

```rust
use tklog::{
    sync::Logger,LEVEL, LOG,
    Format,MODE,
};

fn log_init() {
    LOG.set_console(true)  
       .set_level(LEVEL::Info)  
       .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)  
       .set_cutmode_by_size("tklogsize.txt", 1<<20, 10, true)   
       .set_formatter("{level}{time} {file}:{message}\n");
}
```

###### 以上是全局单实例打印的示例。tklog支持自定义多实例打印。多实例一般应用在系统要求不同打印结构的场景中。

### 多实例打印

```rust
use tklog::{
    debugs, errors, fatals, infos,
    sync::Logger,LEVEL, LOG,
    traces, warns, Format, MODE,
};
fn testmutlilog() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug) //定义日志级别为Debug
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true)   //分割日志文件的方式为按天分割，保留最多10个备份，并压缩备份文件
        .set_formatter("{message} | {time} {file}{level}\n");  //自定义日志结构信息的输入顺序与附加内容
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
**执行结果：**

```text
debugs>>>>,BBBBBBBBB,1,2,3,5 | 2024-05-26 14:13:25 testlog.rs 70[DEBUG]
infos>>>>,CCCCCCCCC,1,2,3,5 | 2024-05-26 14:13:25 testlog.rs 71[INFO]
warns>>>>,DDDDDDDDDD,1,2,3,6 | 2024-05-26 14:13:25 testlog.rs 72[WARN]
errors>>>>,EEEEEEEE,1,2,3,7 | 2024-05-26 14:13:25 testlog.rs 73[ERROR]
fatals>>>>,FFFFFFFF,1,2,3,8 | 2024-05-26 14:13:25 testlog.rs 74[FATAL]
```
###### 注意：以上输入结构化信息由 "{message} | {time} {file}{level} \n"   formatter决定。formatter中除了关键标识 `{message}`  `{time}`  `{file}`  `{level}` 外，其他内容原样输出，如 | ， 空格，换行  等。


------------

## tklog使用详细说明

#### 1. 日志级别 ： Trace < Debug < Info < Warn < Error < Fatal

 **示例**

		LOG.set_level(LEVEL::Info)  //日志级别，设置为Info

#### 2. 控制台日志

**调用 .set_console(bool) 函数**

		LOG.set_console(false)   // false表示不打印控制台日志。默认为true

#### 3. 日志格式

	- Format::Nano                            无格式
	- Format::Date                             输出日期 ：2024-05-26
	- Format::Time                             输出时间，精确到秒：14:13:25
	- Format::Microseconds              输出时间,精确到微妙：18:09:17.462245    
	- Format::LongFileName             长文件信息+行号：tests estlog.rs 25
	- Format::ShortFileName             短文件信息+行号：testlog.rs 25
	- Format::LevelFlag                      日志级别信息： [Debug]

 `LOG.set_format(Format::LevelFlag | Format::Time | Format::ShortFileName) ` 

####  4.自定义格式输出

**默认："{level}{time} {file}:{message} "**

- {level}            日志级别信息：如[Debug]
- {time}            日志时间信息
- {file}               文件位置行号信息
- {message}      日志内容


	LOG.set_formatter("{message} | {time} {file}{level} \n");  //自定义日志结构信息的输入顺序与附加内容

###### 说明：除了关键标识 {message}  {time}  {file}  {level} 外，其他内容原样输出，如 | ， 空格，换行  等。

####  5.按时间分割日志文件

###### 时间标识：`MODE::HOUR`，`MODE::DAY`，`MODE::MONTH`

###### 分别是：小时，天，月份

###### 调用 .set_cutmode_by_time() 函数，参数：

- 文件路径
- 时间模式
- 最大备份日志文件数
- 是否压缩备份的日志文件

**示例**

    let mut log = Logger::new();
    log.set_cutmode_by_time("/usr/local/tklogs.log", MODE::DAY, 0, false)

###### 说明：备份文件路径为： /usr/local/tklogs.log  ，时间模式为：按天备份，参数0表示不限制备份文件数，false表示不压缩备份的日志文件

##### 备份的文件格式：

- 按天备份日期文件，如：
	- tklogs_20240521.log
	- tklogs_20240522.log
- 按小时备份日志文件，如：
	- tklogs_2024052110.log
	- tklogs_2024052211.log
- 按月份备份日志文件，如：
	- tklogs_202403.log
	- tklogs_202404.log

#### 6.按大小分割日志文件

**调用 .set_cutmode_by_size() 函数，参数：**

- 文件路径
- 指定文件滚动大小
- 最大备份日志文件数
- 是否压缩备份的日志文件

**示例**

    let mut log = Logger::new();
    log.set_cutmode_by_time("tklogs.log", 100<<20, 10, true)

###### 说明：备份文件路径为：tklogs.log  ，按100M大小备份文件，参数10表示只保留最新10个备份文件，true表示压缩备份的日志文件

**备份的文件格式**

- tklogs_1.log.gz
- tklogs_2.log.gz
- tklogs_3.log.gz

------------

#### tklog提供常规日志打印 方法为：

- **全局单例打印**
	-  `trace!` `debug!` `info!`  `warn!`  `error!`  `fatal!`
- **多实例打印**
	-  `traces!` `debugs!` `infos!`  `warns!`  `errors!`  `fatals!`


### 异步日志

- **全局异步单例打印**
	-  `async_trace!` `async_debug!` `async_info!` `async_warn!` `async_error!` `async_fatal!`
- **多实例异步打印**
	-  `async_traces!` `async_debugs!` `async_infos!` `async_warns!` `async_errors!` `async_fatals!`

##### 示例

**全局单例异步**

```rust
use tklog::{
    async_debug,  async_error,  async_fatal,  async_info,  async_trace,  async_warn,  LEVEL, Format, ASYNC_LOG
 };

async fn async_log_init() {
    // 全局单例设置参数
   ASYNC_LOG
        .set_console(false)   //控制台
        .set_level(LEVEL::Trace)  //日志级别
        .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)  //结构化日志，定义输出的日志信息
        .set_cutmode_by_size("tklog_async.txt", 10000, 10, false).await;  //日志文件切割模式为文件大小，每10000字节切割一次，保留10个备份日志文件
 }
 
 
#[tokio::test]
async fn testlog() {
    async_log_init().await;  //参数设置
    async_trace!("trace>>>>", "aaaaaaa", 1, 2, 3);
    async_debug!("debug>>>>", "aaaaaaa", 1, 2, 3);
    async_info!("info>>>>", "bbbbbbbbb", 1, 2, 3);
    async_warn!("warn>>>>", "cccccccccc", 1, 2, 3);
    async_error!("error>>>>", "ddddddddddddd", 1, 2, 3);
    async_fatal("fatal>>>>", "eeeeeeeeeeeeee", 1, 2, 3);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
```

##### 输出结果:

```text
[TRACE] 20:03:32 testasynclog.rs 20:trace>>>>,aaaaaaa,1,2,3
[DEBUG] 20:03:32 testasynclog.rs 21:debug>>>>,aaaaaaa,1,2,3
[INFO] 20:03:32 testasynclog.rs 22:info>>>>,bbbbbbbbb,1,2,3
[WARN] 20:03:32 testasynclog.rs 23:warn>>>>,cccccccccc,1,2,3
[ERROR] 20:03:32 testasynclog.rs 24:error>>>>,ddddddddddddd,1,2,3
[FATAL] 20:03:32 testasynclog.rs 25:fatal>>>>,eeeeeeeeeeeeee,1,2,3
```

##### 多实例异步

```rust
use std::sync::Arc;

use tklog::{
     async_debugs,  async_errors,  async_fatals,  async_infos,  async_traces,  async_warns, LEVEL, Format, ASYNC_LOG, MODE
};
#[tokio::test]
async fn testmultilogs() {
    //新建 Async::Logger 对象
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
##### 输出结果:

```text
async_debugs>>>>,BBBBBBBBBB,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 45[DEBUG]
async_infos>>>>,CCCCCCCCCC,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 46[INFO]
async_warns>>>>,DDDDDDDDDD,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 47[WARN]
async_errors>>>>,EEEEEEEEEEE,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 48[ERROR]
async_fatals>>>>,FFFFFFFFFFFF,1,2,3 | 2024-05-26 20:10:24 testasynclog.rs 49[FATAL]
```

------------

## 支持官方日志库标准API

1.  tklog实现了官方Log接口，支持官方标准化日志API的调用
2.  实现了官方log库API的异步场景调用。

##### 启用官方log库API的方法： 

###### tklog通过调用  `uselog() ` 函数 来启用官方log的API支持


###### 使用示例

```rust
use std::{thread, time::Duration};
use tklog::{Format, LEVEL, LOG};
fn test_synclog() {
    //初始化
    LOG.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_size("logsize.log", 10000, 10, true)
        .uselog();  //启用官方log库
	
	log::trace!("trace>>>>{}{}{}{}{}", "aaaa", 1, 2, 3, 4);
	log::debug!("debug>>>>{}{}",1,2);
    log::info!("info log");
    log::warn!("warn log");
    log::error!("error log");
	thread::sleep(Duration::from_secs(1))
}
```


####  异步场景中启用 log库API

```rust
use std::{thread, time::Duration};
use tklog::{Format, LEVEL, ASYNC_LOG};
async fn test_synclog() {
    //初始化
    ASYNC_LOG.set_console(false)
        .set_cutmode_by_size("asynclogsize.log", 10000, 10, true).await
        .uselog(); //启用官方log库
	
    log::trace!("trace async log>>>>{}{}{}{}{}", "aaaaaaaaa", 1, 2, 3, 4);
    log::debug!("debug async log>>>>{}{}",1,2);
	log::info!("info async log");
    log::warn!("warn async log");
    log::error!("error async log");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
```

------------

## 模块设置独立日志参数

1. tklog提供了`set_option`与`set_mod_option` 设置Logger对象的全局日志参数和指定mod的日志参数
2. 在项目中，可以使用全局LOG对象，同时对多个mod设置独立的日志参数
3. 不同mod可以设置不同的日志级别，日志格式，日志文件等
4. 异步全局对象ASYNC_LOG的mod日志参数设置与同步LOG相同


#####  `set_option` 示例：

	tklog::LOG.set_option(LogOption{level:Some(LEVEL::Debug),console: Some(false),format:None,formatter:None,fileoption: Some(Box::new(FileTimeMode::new("day.log",tklog::MODE::DAY,0,true)))});

##### LogOption对象说明

- level      日志级别
- format    日志格式
- formatter   日志输出自定义格式
- console    控制台日志设置
- fileoption		文件日志设置


#####  `set_mod_option` 示例：

	tklog::LOG.set_mod_option("testlog::module1",LogOption{level:Some(LEVEL::Debug),console: Some(false),format:None,formatter:None,fileoption: Some(Box::new(FileTimeMode::new("day.log", tklog::MODE::DAY, 0,true)))});


- `testlog::module1` 为设置的模块名，可以通过rust内置宏  `module_path!()`  打印出当前模块名
- 当tklog在模块 `testlog::module1` 中使用时，将tklog将使用该LogOption对象

#####  `set_mod_option` 示例2：

	tklog::LOG.set_mod_option("testlog::*",LogOption{level:Some(LEVEL::Debug),console: Some(false),format:None,formatter:None,fileoption: Some(Box::new(FileTimeMode::new("day.log", tklog::MODE::DAY, 0,true)))});


- `testlog::*` tklog支持用*匹配所有子模块，`testlog::*`表示`testlog`的所有子模块
- `testlog::module1::*` 表示`testlog::module1`的所有子模块

#### 完整的mod示例

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
##### 执行结果

```rust
[DEBUG] 2024-06-19 10:54:07 testlog.rs 54:module1,tklog api,LOG debug log>>,123
[INFO] 2024-06-19 10:54:07 testlog.rs 55:module1,tklog api,LOG info log>>,456
[DEBUG] 2024-06-19 10:54:07 testlog.rs 56:module1,log api,debug log>>111
[INFO] 2024-06-19 10:54:07 testlog.rs 57:module1,log api,info log>>222
[INFO] 2024-06-19 10:54:08 testlog.rs 68:module2,tklog api,LOG info log>>,456
[INFO] 2024-06-19 10:54:08 testlog.rs 70:module2,log api,info log>>222
```

#### 示例2: 异步日志

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

##### 执行结果：

```rust
[DEBUG] 2024-06-19 10:59:26 testlog.rs 85:async module3,tklog api,LOG debug log>>,123
[INFO] 2024-06-19 10:59:26 testlog.rs 86:async module3,tklog api,LOG info log>>,456
[DEBUG] 2024-06-19 10:59:26 testlog.rs 87:async module3,log api,debug log>>333
[INFO] 2024-06-19 10:59:26 testlog.rs 88:async module3,log api,info log>>444
[INFO] 2024-06-19 10:59:27 testlog.rs 98:async module4,tklog api,LOG info log>>,456
[INFO] 2024-06-19 10:59:27 testlog.rs 100:async module4,log api,info log>>444

```

------------

## tklog 支持自定义多实例格式化  format!与 异步format!

##### 示例：
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

##### 执行结果

```rust
[DEBUG] 2024-06-06 15:54:07 testsynclog.rs 80:Debug>>>1,2>>>[1, 2, 3]
[INFO] 2024-06-06 15:54:07 testsynclog.rs 83:Info>>>1,2>>['a', 'b']
[WARN] 2024-06-06 15:54:07 testsynclog.rs 84:Warn>>>1,2
[ERROR] 2024-06-06 15:54:07 testsynclog.rs 85:Error>>>1,2
[FATAL] 2024-06-06 15:54:07 testsynclog.rs 86:Fatal>>>1,2
```

##### 异步 formats示例

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

###### 执行结果

```rust
[DEBUG] 2024-06-06 16:09:26 testasynclog.rs 61:Debug>>>1,2>>>[1, 2, 3]
[INFO] 2024-06-06 16:09:26 testasynclog.rs 64:Info>>>1,2>>['a', 'b']
[WARN] 2024-06-06 16:09:26 testasynclog.rs 65:Warn>>>1,2
[ERROR] 2024-06-06 16:09:26 testasynclog.rs 66:Error>>>1,2
[FATAL] 2024-06-06 16:09:26 testasynclog.rs 67:Fatal>>>1,2
```

------

## tklog 支持自定义日志处理函数

###### tklog 通过 `set_custom_handler()` 添加外部自定义函数，控制日志处理的流程与逻辑

##### 示例

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

###### 执行结果

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

###### 说明：
当 `fn custom_handler(lc: &LogContext) -> bool`  返回true时，tklog调用`custom_handler`执行自定义函数后，继续执行tklog的打印流程。当返回false时，tklog不再执行tklog的打印程序。直接返回。如示例中所示，当年日志级别为Debug时，返回false，所以，tklog的Debug日志，不再打印出来。

## tklog 支持自定义日志多参数分隔符

###### tklog 通过 `set_separator()` 设置分隔符


```rust
#[test]
fn testlog() {
    log_init();
    trace!("trace>>>>", "aaaaaaaaa", 1, 2, 3, 4);
    debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
    LOG.set_separator("|");  //设置参数分隔符 | 
    info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
    warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
    LOG.set_separator(","); //设置参数分隔符 ，
    error!("error>>>>", "eeeeeeee", 1, 2, 3, 7);
    fatal!("fatal>>>>", "ffffffff", 1, 2, 3, 8);
    thread::sleep(Duration::from_secs(1))
}
```

###### 执行结果

```rust
---- testlog stdout ----
[TRACE] 2024-08-15 14:14:19.289590 tests\testsynclog.rs 22:trace>>>>aaaaaaaaa1234
[DEBUG] 2024-08-15 14:14:19.289744 tests\testsynclog.rs 23:debug>>>>bbbbbbbbb1235
[INFO] 2024-08-15 14:14:19.289761 tests\testsynclog.rs 25:info>>>>|ccccccccc|1|2|3|5
[WARN] 2024-08-15 14:14:19.289774 tests\testsynclog.rs 26:warn>>>>|dddddddddd|1|2|3|6
[ERROR] 2024-08-15 14:14:19.289789 tests\testsynclog.rs 28:error>>>>,eeeeeeee,1,2,3,7
[FATAL] 2024-08-15 14:14:19.289802 tests\testsynclog.rs 29:fatal>>>>,ffffffff,1,2,3,8
```

## tklog 支持日志级别设置独立日志格式参数

###### tklog 通过 `set_level_option()` 设置日志级别的独立日志参数

###### `set_level_option()` 接收 任意实现 `OptionTrait`特征的对象

##### 示例1 ：参数 `LevelOption` 对象，可以设置日志格式化输出

```rust
#[test]
fn testlog() {
    //将Info级别的日志格式设置为 Format::LevelFlag
    //将Fatal级别的日志格式设置为 Format::LevelFlag | Format::Date
    LOG.set_level_option(LEVEL::Info, LevelOption { format: Some(Format::LevelFlag), formatter: None })
    .set_level_option(LEVEL::Fatal, LevelOption { format: Some(Format::LevelFlag | Format::Date), formatter: None});

    trace!("this is trace log");
    debug!("this is debug log");
    info!("this is info log");
    warn!("this is warn log");
    error!("this is error log");
    fatal!("this is fatal log");
    thread::sleep(Duration::from_secs(1))
}
```

###### 执行结果

```rust
---- testlog stdout ----
[DEBUG] 2024-08-24 15:06:02 test_0100.rs 17:this is debug log
[INFO] this is info log
[WARN] 2024-08-24 15:06:02 test_0100.rs 19:this is warn log
[ERROR] 2024-08-24 15:06:02 test_0100.rs 20:this is error log
[FATAL] 2024-08-24 this is fatal log
```

##### 示例2  参数 `LogOption` 对象，可以设置更多参数，包括设置日志文件

```rust
#[test]
fn testlog() {
    LOG.set_level_option(LEVEL::Info, LogOption { format: Some(Format::LevelFlag), formatter: None, level:None, console: None, fileoption: Some(Box::new(FileTimeMode::new("0200time.log", tklog::MODE::DAY, 0, false))) })
    .set_level_option(LEVEL::Fatal, LogOption { format: Some(Format::LevelFlag | Format::Date), formatter: None, level: None, console: None, fileoption: Some(Box::new(FileSizeMode::new("0200size.log", 1<<10, 0, false)))});

    trace!("this is trace log");
    debug!("this is debug log");
    info!("this is info log");
    warn!("this is warn log");
    error!("this is error log");
    fatal!("this is fatal log");
    thread::sleep(Duration::from_secs(1))
}
```
**示例说明：**
1. Info级别的文件日志设置为按天分割，文件名 `0200time.log`
2. Fatal级别的文件日志设置为按大小分割，文件名 `0200size.log`

------------

## tklog 支持对日志属性标识进行格式化设置

##### 通过 `set_attr_format` 函数设置日志标识与时间格式

##### 示例：

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
##### 执行结果：
```text
[D] 2024/10/17 19:41:20 test_0230.rs 32:debug!this is sync log
[I] 2024/10/17 19:41:20 test_0230.rs 33:info!this is sync log
[W] 2024/10/17 19:41:20 test_0230.rs 34:warn!this is sync log
[E] 2024/10/17 19:41:20 test_0230.rs 35:error!this is sync log
[F] 2024/10/17 19:41:20 test_0230.rs 36:fata!this is sync log
```

------------

## tklog 基准压力测试

```text
log_benchmark           time:   [2.9703 µs 2.9977 µs 3.0256 µs]
                        change: [-95.539% -95.413% -95.268%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```

```text
log_benchmark           time:   [2.9685 µs 3.0198 µs 3.0678 µs]
                        change: [-3.6839% -1.2170% +1.0120%] (p = 0.34 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
```


```text
test_debug              time:   [3.3747 µs 3.4599 µs 3.5367 µs]
                        change: [-69.185% -68.009% -66.664%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
```

```rust
test_debug              time:   [3.8377 µs 3.8881 µs 3.9408 µs]
                        change: [-66.044% -65.200% -64.363%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

##### 说明：时间范围给出了三个数据点，分别代表了测试执行时间的最小值（2.9055微秒）、平均值附近的值（2.9444微秒-3.8881微妙）、以及最大值（3.9408微秒）


##### 结论：tklog基准测试结果：2 µs/op — 3.9 µs/op   （微妙/次）