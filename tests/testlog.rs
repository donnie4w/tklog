use std::{thread, time::Duration};

use tklog::{Format, ASYNC_LOG, LEVEL, LOG};

#[test]
fn log_syncinit() {
    LOG.set_console(true).set_level(LEVEL::Debug).set_format(Format::LevelFlag | Format::Microseconds | Format::ShortFileName).set_cutmode_by_size("logsize.log", 10000, 10, true).set_formatter("{level}{time} {file}:{message}\n").uselog();
}

#[test]
fn testsynclog() {
    LOG.uselog();
    let r = vec![1, 2, 3];
    log::debug!("debug>>>>{}{}{:?}", 1, 2, r);
    log::info!("info log");
    log::warn!("warn log");
    log::error!("error log");
    thread::sleep(Duration::from_secs(1))
}

#[test]
fn test_synclog() {
    //初始化
    LOG.set_console(true).set_level(LEVEL::Trace).set_cutmode_by_size("logsize.log", 10000, 10, true).uselog(); //启用官方log库
    let r = vec![1, 2, 3];
    log::trace!("trace>>>>{}{}{}{}{:?}", "aaaa", 1, 2, 3, r);
    log::debug!("debug>>>>{}{}{:?}", 1, 2, r);
    log::info!("info log");
    log::warn!("warn log");
    log::error!("error log");
    thread::sleep(Duration::from_secs(1))
}

async fn log_asyncinit() {
    ASYNC_LOG.set_console(true).set_level(LEVEL::Trace).set_format(Format::LevelFlag | Format::Time | Format::Date).set_formatter("{level}{time} {file}:{message}\n").set_cutmode_by_size("asynclogsize.log", 10000, 10, true).await.uselog();
}

#[tokio::test]
async fn testasynclog() {
    crate::log_asyncinit().await;
    log::trace!("trace async log>>>>{}{}{}{}{}", "aaaaaaaaa", 1, 2, 3, 4);
    log::debug!("debug async log>>>>{}{}", 1, 2);
    log::info!("info async log");
    log::warn!("warn async log");
    log::error!("error async log");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

mod module1 {
    use std::{thread, time::Duration};
    use tklog::{handle::FileTimeMode, LogOption, LEVEL};
    pub fn testmod() {
        tklog::LOG.set_mod_option("testlog::module1", LogOption { level: Some(LEVEL::Debug), format: None, formatter: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("module1.log", tklog::MODE::DAY, 0, true))) }).uselog();
        tklog::debug!("module1,tklog api,LOG debug log>>", 123);
        tklog::info!("module1,tklog api,LOG info log>>", 456);
        log::debug!("module1,log api,debug log>>{}", 111);
        log::info!("module1,log api,info log>>{}", 222);
        thread::sleep(Duration::from_secs(1))
    }
}

mod module2 {
    use std::{thread, time::Duration};
    use tklog::{handle::FileTimeMode, LogOption, LEVEL};
    pub fn testmod() {
        tklog::LOG.set_mod_option("testlog::module2", LogOption { level: Some(LEVEL::Info), format: None, formatter: None, console: None, fileoption: Some(Box::new(FileTimeMode::new("module2.log", tklog::MODE::DAY, 0, true))) }).uselog();
        tklog::debug!("module2,tklog api,LOG debug log>>", 123);
        tklog::info!("module2,tklog api,LOG info log>>", 456);
        log::debug!("module2,log api,debug log>>{}", 111);
        log::info!("module2,log api,info log>>{}", 222);
        thread::sleep(Duration::from_secs(1))
    }
}

#[test]
fn testmod2() {
    module1::testmod();
    module2::testmod();
}

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