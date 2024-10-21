use std::{thread, time::Duration};

use chrono::{DateTime, Local};
use tklog::{async_debug, async_error, async_fatal, async_info, async_trace, async_warn, Format, ASYNC_LOG, LOG};
use tklog::{debug, error, fatal, info, trace, warn, LEVEL};

#[test]
fn testlog() {
    LOG.set_attr_format(|fmt| {
        fmt.set_level_fmt(|level| {
            match level {
                LEVEL::Trace => "trace",
                LEVEL::Debug => "Debug",
                LEVEL::Info => "Info",
                LEVEL::Warn => "Warn",
                LEVEL::Error => "Error",
                LEVEL::Fatal => "Fatal",
                LEVEL::Off => "",
            }
            .to_string()
        });

        fmt.set_time_fmt(|| {
            let now: DateTime<Local> = Local::now();
            (now.format("%Y/%m/%d").to_string(), now.format("%H:%M:%S").to_string(), "".to_string())
        });
    });

    LOG.set_formatter("[{time} {level}] {file}:{message}\n");
   
    trace!("trace!", "this is sync log");
    debug!("debug!", "this is sync log");
    info!("info!", "this is sync log");
    warn!("warn!", "this is sync log");
    error!("error!", "this is sync log");
    fatal!("fata!", "this is sync log");
    thread::sleep(Duration::from_secs(1))
}

#[tokio::test]
async fn asynctestlog() {
    ASYNC_LOG.set_attr_format(|fmt| {
        fmt.set_level_fmt(|level| {
            match level {
                LEVEL::Trace => "[AT]",
                LEVEL::Debug => "[AD]",
                LEVEL::Info => "[AI]",
                LEVEL::Warn => "[AW]",
                LEVEL::Error => "[AE]",
                LEVEL::Fatal => "[AF]",
                LEVEL::Off => "",
            }
            .to_string()
        });
    });

    ASYNC_LOG.set_format(Format::Date|Format::Time|Format::LevelFlag|Format::LongFileName);

    async_trace!("trace!", "this is async log");
    async_debug!("debug!", "this is async log");
    async_info!("info!", "this is async log");
    async_warn!("warn!", "this is async log");
    async_error!("error!", "this is async log");
    async_fatal!("fata!", "this is async log");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}
