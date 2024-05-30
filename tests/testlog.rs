use std::{thread, time::Duration};

use tklog::{ASYNC_LOG, Format, LEVEL, LOG};

#[test]
fn log_syncinit() {
    LOG.set_console(true)
        .set_level(LEVEL::Debug)
        .set_format(Format::LevelFlag | Format::Microseconds | Format::ShortFileName)
        .set_cutmode_by_size("logsize.log", 10000, 10, true)
        .set_formatter("{level}{time} {file}:{message}\n").uselog();
}


#[test]
fn testsynclog() {
    log_syncinit();
    log::trace!("trace>>>>{}{}{}{}{}", "aaaaaaaaa", 1, 2, 3, 4);
    log::debug!("debug>>>>{}{}",1,2);
    log::info!("info log");
    log::warn!("warn log");
    log::error!("error log");
    thread::sleep(Duration::from_secs(1))
}

async fn log_asyncinit() {
    ASYNC_LOG.set_console(true)
        .set_level(LEVEL::Trace)
        .set_format(Format::LevelFlag | Format::Time |Format::Date)
        .set_formatter("{level}{time} {file}:{message}\n")
        .set_cutmode_by_size("asynclogsize.log", 10000, 10, true).await
        .uselog();
}


#[tokio::test]
async fn testasynclog() {
    crate::log_asyncinit().await;
    log::trace!("trace async log>>>>{}{}{}{}{}", "aaaaaaaaa", 1, 2, 3, 4);
    log::debug!("debug async log>>>>{}{}",1,2);
	log::info!("info async log");
    log::warn!("warn async log");
    log::error!("error async log");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}