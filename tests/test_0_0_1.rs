use std::{
    thread,
    time::Duration,
};

use tklog::{debug, error, fatal, info, trace, warn, LOG};
use tklog::{
    async_debug, async_error, async_fatal, async_info, async_trace, async_warn, ASYNC_LOG
};

#[test]
fn testlog() {
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


#[tokio::test]
async fn asynctestlog() {
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