use std::{thread, time::Duration};

use tklog::{async_debug, async_error, async_fatal, async_info, async_trace, async_warn, debug, error, fatal, handle::FileSizeMode, info, trace, warn, LogOption, ASYNC_LOG, LOG};

#[test]
fn testlog() {
    let mut option = LogOption::new();
    let op = option.set_fileoption(FileSizeMode::new("bench_mod.log", 1 << 30, 0, true));
    LOG.set_mod_option(&format!("{}{}", module_path!(), "::m1"), op.take()).uselog();
    LOG.set_level_option(tklog::LEVEL::Error, op.take());
    LOG.set_cutmode_by_size("tklog_sync.log", 1 << 20, 0, false);
    trace!("trace>>>>", "aaaaaaaaa", 1, 2, 3, 4);
    debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
    info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
    warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
    error!("error>>>>", "eeeeeeee", 1, 2, 3, 7);
    fatal!("fatal>>>>", "ffffffff", 1, 2, 3, 8);
    thread::sleep(Duration::from_secs(1))
}

#[test]
fn testlog_threads() {
    LOG.set_cutmode_by_size("tklog_thread_sync.log", 1 << 20, 0, false);
    for _ in 0..20 {
        thread::scope(|s| {
            for i in 0..100 {
                s.spawn(move || {
                    for _ in 0..10 {
                        debug!("debug>>>>", "thread aaaaaaaaaaaaaaaaa>>", i);
                    }
                });
            }
        });
    }
    thread::sleep(Duration::from_secs(2));
}

#[tokio::test]
async fn testalog() {
    ASYNC_LOG.set_cutmode_by_size("tklog_async.log", 1 << 20, 10, true).await;
    async_trace!("async_trace>>>>", "aaaaaaaaaaaa", 1, 2, 3);
    async_debug!("async_debug>>>>", "bbbbbbbbbbbb", 1, 2, 3);
    async_info!("async_info>>>>", "cccccccccccc", 1, 2, 3);
    async_warn!("async_warn>>>>", "dddddddddddd", 1, 2, 3);
    async_error!("async_error>>>>", "eeeeeeeeeeee", 1, 2, 3);
    async_fatal!("async_fatal>>>>", "ffffffffffff", 1, 2, 3);
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
