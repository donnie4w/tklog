use std::{thread, time::Duration};

use tklog::{debug, info, warn, LOG};

#[test]
fn testlogtime() {
    LOG.set_console(true);
    LOG.set_cutmode_by_time("filetime.log", tklog::MODE::DAY, 0, false);

    for _ in 0..1 << 10 {
        debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
        info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
        warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
        thread::sleep(Duration::from_secs(1))
    }
}
