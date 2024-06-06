
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

### [官网](https://tlnet.top/tklog "官网")

### 使用方法概述

最简单常用的方法：**直接调用**

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
        .set_formatter("{message} | {time} {file}{level}
");  //自定义日志结构信息的输入顺序与附加内容
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
###### 注意：以上输入结构化信息由 "{message} | {time} {file}{level} "   formatter决定。formatter中除了关键标识 `{message}`  `{time}`  `{file}`  `{level}` 外，其他内容原样输出，如 | ， 空格，换行  等。


------------

### tklog使用详细说明

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


	LOG.set_formatter("{message} | {time} {file}{level}");  //自定义日志结构信息的输入顺序与附加内容

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
        .set_formatter("{message} | {time} {file}{level}
");
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

### 支持官方日志库标准API

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

### tklog 支持自定义多实例格式化  format!与 异步format!

###### 示例：

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

###### 执行结果

	[DEBUG] 2024-06-06 15:54:07 testsynclog.rs 80:Debug>>>1,2>>>[1, 2, 3]
	[INFO] 2024-06-06 15:54:07 testsynclog.rs 83:Info>>>1,2>>['a', 'b']
	[WARN] 2024-06-06 15:54:07 testsynclog.rs 84:Warn>>>1,2
	[ERROR] 2024-06-06 15:54:07 testsynclog.rs 85:Error>>>1,2
	[FATAL] 2024-06-06 15:54:07 testsynclog.rs 86:Fatal>>>1,2


###### 异步 formats示例

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

###### 执行结果

	[DEBUG] 2024-06-06 16:09:26 testasynclog.rs 61:Debug>>>1,2>>>[1, 2, 3]
	[INFO] 2024-06-06 16:09:26 testasynclog.rs 64:Info>>>1,2>>['a', 'b']
	[WARN] 2024-06-06 16:09:26 testasynclog.rs 65:Warn>>>1,2
	[ERROR] 2024-06-06 16:09:26 testasynclog.rs 66:Error>>>1,2
	[FATAL] 2024-06-06 16:09:26 testasynclog.rs 67:Fatal>>>1,2

------------

### tklog 基准压力测试

```text
test_debug              time:   [3.3747 µs 3.4599 µs 3.5367 µs]
                               change: [-69.185% -68.009% -66.664%] (p = 0.00 < 0.05)
                               Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
```
###### 说明：时间范围给出了三个数据点，分别代表了测试执行时间的最小值（3.3747微秒）、平均值附近的值（3.4599微秒）、以及最大值（3.5367微秒）


```rust
test_debug              time:   [3.8377 µs 3.8881 µs 3.9408 µs]
                                change: [-66.044% -65.200% -64.363%] (p = 0.00 < 0.05)
                                Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
###### 说明：测试运行的时间范围是从3.8377微秒到3.9408微秒，覆盖了一个大概的分布情况，其中3.8881微秒大约是这段时间内的平均或中位数执行时间

###### 结论：日志打印函数性能：3 µs/op — 4 µs/op   （微妙/次）