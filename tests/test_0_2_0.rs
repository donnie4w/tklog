use std::{
    thread,
    time::Duration,
};

use tklog::handle::{FileSizeMode, FileTimeMode};
use tklog::{debug, error, fatal, info, trace, warn, LogOption, LEVEL, LOG};
use tklog::{
    async_debug, async_error, async_fatal, async_info, async_trace, async_warn, ASYNC_LOG
};

#[test]
fn testlog() {
    LOG
    .set_level_option(LEVEL::Debug, LogOption { format: None, formatter: None, level:None, console: None, fileoption: Some(Box::new(FileTimeMode::new("0200time.log", tklog::MODE::DAY, 0, false))) })
    .set_level_option(LEVEL::Error, LogOption { format: None, formatter: None, level: None, console: None, fileoption: Some(Box::new(FileSizeMode::new("0200size.log", 1<<10, 0, false)))})
    .set_level_option(LEVEL::Fatal, LogOption { format: None, formatter: None, level: None, console: None, fileoption: Some(Box::new(FileSizeMode::new("0200size.log", 1<<10, 0, false)))});

    trace!("this is trace log");
    debug!("this is debug log");
    info!("this is info log");
    warn!("this is warn log");
    error!("this is error log");
    fatal!("this is fatal log");
    thread::sleep(Duration::from_secs(1))
}



#[tokio::test]
async fn asynctestlog() {
    ASYNC_LOG.set_level_option(LEVEL::Info, LogOption { format: None, formatter: None, level:None, console: None, fileoption: Some(Box::new(FileTimeMode::new("0200asynctime.log", tklog::MODE::DAY, 0, false))) }).await
    .set_level_option(LEVEL::Fatal, LogOption { format: None, formatter: None, level: None, console: None, fileoption: Some(Box::new(FileSizeMode::new("0200asyncsize.log", 1<<10, 0, false)))}).await;
    async_trace!("this is async trace log");
    async_debug!("this is async debug log");
    async_info!("this is async info log");
    async_warn!("this is async warn log");
    async_error!("this is async error log");
    async_fatal!("this is async fatal log");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}