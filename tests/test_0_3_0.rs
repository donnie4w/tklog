use chrono::{DateTime, Local};
use std::{thread, time::Duration};
use tklog::{debug, error, fatal, info, trace, warn, LEVEL};
use tklog::LOG;

#[test]
fn testlog() {
    LOG.set_cutmode_by_mixed("030test.log", 1 << 15, tklog::MODE::HOUR, 10, false);
    LOG.set_formatter("{\"level\":\"{level}\",\"time\":\"{time}\",\"file\":\"{file}\",\"message\":\"{message}\"}\n").set_console(true);
    trace!("trace!", "this is sync log");
    debug!("debug!", "this is sync log");
    info!("info!", "this is sync log");
    warn!("warn!", "this is sync log");
    error!("error!", "this is sync log");
    fatal!("fata!", "this is sync log");
    thread::sleep(Duration::from_secs(3))
}

#[test]
fn testlog2() {
    LOG.set_attr_format(|fmt| {
        fmt.set_level_fmt(|level| {
            match level {
                LEVEL::Trace => "trace",
                LEVEL::Debug => "debug",
                LEVEL::Info => "info",
                LEVEL::Warn => "warn",
                LEVEL::Error => "error",
                LEVEL::Fatal => "fatal",
                LEVEL::Off => "",
            }
            .to_string()
        });

        fmt.set_time_fmt(|| {
            let now: DateTime<Local> = Local::now();
            ("".to_string(), "".to_string(), now.format("%Y%m%d %H:%M:%S").to_string())
        });
    });
    LOG.set_formatter("{\"level\":\"{level}\",\"time\":\"{time}\",\"file\":\"{file}\",\"message\":\"{message}\"}\n").set_console(true).set_level(LEVEL::Trace);
    trace!("trace!", "this is sync log");
    debug!("debug!", "this is sync log");
    info!("info!", "this is sync log");
    warn!("warn!", "this is sync log");
    error!("error!", "this is sync log");
    fatal!("fata!", "this is sync log");
    thread::sleep(Duration::from_secs(3))
}
