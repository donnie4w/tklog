use std::{thread, time::Duration};
use tklog::{async_debug, async_error, async_fatal, async_info, async_trace, async_warn, ASYNC_LOG, LOG};
use tklog::{debug, error, fatal, info, trace, warn, LEVEL};

#[test]
fn testlog() {
    LOG.set_cutmode_by_mixed("029mixed.log", 1 << 15,tklog::MODE::HOUR, 10, false).set_level(LEVEL::Trace);

    for _ in 0..1000{
        trace!("trace!", "this is sync log");
        debug!("debug!", "this is sync log");
        info!("info!", "this is sync log");
        warn!("warn!", "this is sync log");
        error!("error!", "this is sync log");
        fatal!("fata!", "this is sync log");
        thread::sleep(Duration::from_secs(1))
    }

    thread::sleep(Duration::from_secs(3))
}


#[test]
fn testlog2() {
    let mut lo = tklog::LogOption::new();
    lo.set_console(true).set_fileoption(tklog::handle::FileMixedMode::new("029mixed2.log", 1 << 15,tklog::MODE::DAY, 10, false));
    LOG.set_option(lo);

    for _ in 0..1000{
        trace!("trace!", "this is sync log");
        debug!("debug!", "this is sync log");
        info!("info!", "this is sync log");
        warn!("warn!", "this is sync log");
        error!("error!", "this is sync log");
        fatal!("fata!", "this is sync log");
    }

    thread::sleep(Duration::from_secs(3))
}

#[tokio::test]
async fn asynctestlog() {
    ASYNC_LOG.set_level(LEVEL::Trace).set_cutmode_by_mixed("029async_mixid.log",1 << 15, tklog::MODE::DAY, 10, true).await;

    for _ in 0..1000 {
        async_trace!("trace!", "this is async log");
        async_debug!("debug!", "this is async log");
        async_info!("info!", "this is async log");
        async_warn!("warn!", "this is async log");
        async_error!("error!", "this is async log");
        async_fatal!("fata!", "this is async log");
    }


    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

#[tokio::test]
async fn asynctestlog2() {
    let mut lo = tklog::LogOption::new();
    lo.set_console(true).set_fileoption(tklog::handle::FileMixedMode::new("029async_mixid2.log", 1 << 15,tklog::MODE::DAY, 10, false));
    ASYNC_LOG.set_option(lo).await;

    for _ in 0..1000 {
        async_trace!("trace!", "this is async log");
        async_debug!("debug!", "this is async log");
        async_info!("info!", "this is async log");
        async_warn!("warn!", "this is async log");
        async_error!("error!", "this is async log");
        async_fatal!("fata!", "this is async log");
    } 

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

