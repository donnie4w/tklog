use std::{borrow::BorrowMut, sync::Arc};

use tklog::{
    async_debug, async_debugs, async_error, async_errors, async_fatal, async_fatals, async_info, async_infos, async_trace, async_traces, async_warn, async_warns, Format, LogContext, ASYNC_LOG, LEVEL, MODE
};
use tokio::{sync::Mutex, time::Instant};

async fn async_log_init() {
    ASYNC_LOG
        .set_console(true)
        .set_level(LEVEL::Debug)
        .set_format(Format::LevelFlag | Format::Date | Format::Time | Format::ShortFileName)
        .set_cutmode_by_size("tklog_async.log", 10000, 10, true)
        .await;
}

#[tokio::test]
async fn testlog() {
    async_log_init().await;
    async_trace!("trace>>>>", "aaaaaaaaaaaa", 1, 2, 3);
    async_debug!("debug>>>>", "aaaaaaaaaaaa", 1, 2, 3);
    ASYNC_LOG.set_separator("|");
    async_info!("info>>>>", "bbbbbbbbbbbb", 1, 2, 3);
    async_warn!("warn>>>>", "ccccccccccccc", 1, 2, 3);
    ASYNC_LOG.set_separator(",");
    async_error!("error>>>>", "ddddddddddddd", 1, 2, 3);
    async_fatal!("fatal>>>>", "eeeeeeeeeeeeee", 1, 2, 3);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

#[tokio::test]
async fn testmultilogs() {
    let mut log = tklog::Async::Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_separator(",")
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true)
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

#[tokio::test]
async fn testlogsize() {
    let mut log = tklog::Async::Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true)
        .await;

    let logger = Arc::new(Mutex::new(log));

    let mut handles = vec![];
    for i in 0..100 {
        let mut log = logger.clone();
        let handle = tokio::spawn(async move {
            let logmut = log.borrow_mut();
            async_debugs!(logmut, "thread>>", i, format!("{:?}", Instant::now()))
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}


#[tokio::test]
async fn test_custom() {
    fn custom_handle(lc: &LogContext) -> bool {
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
    ASYNC_LOG.set_custom_handler(custom_handle);
    async_debug!("000000000000000000");
    async_info!("1111111111111111111");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_custom_multi() {
    fn custom_handle(lc: &LogContext) -> bool {
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
    let mut log = tklog::Async::Logger::new();
    log.set_custom_handler(custom_handle);
    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();
    async_traces!(log, "async_traces>>>>", "AAAAAAAAAA", 1, 2, 3);
    async_debugs!(log, "async_debugs>>>>", "BBBBBBBBBB", 1, 2, 3);
    async_infos!(log, "async_infos>>>>", "CCCCCCCCCC", 1, 2, 3);
    async_warns!(log, "async_warns>>>>", "DDDDDDDDDD", 1, 2, 3);
    async_errors!(log, "async_errors>>>>", "EEEEEEEEEEE", 1, 2, 3);
    async_fatals!(log, "async_fatals>>>>", "FFFFFFFFFFFF", 1, 2, 3);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

