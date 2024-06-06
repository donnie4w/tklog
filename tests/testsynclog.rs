use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use tklog::{
    debug, debugs, error, errors, fatal, fatals, info, infos, sync::Logger, trace, traces, warn,
    warns, Format, LEVEL, LOG, MODE,
};

fn log_init() {
    LOG.set_console(true)
        .set_level(LEVEL::Trace)
        .set_format(Format::LevelFlag | Format::Microseconds | Format::LongFileName)
        .set_cutmode_by_size("tklogsize.log", 10000, 10, true)
        .set_formatter("{level}{time} {file}:{message}\n");
}

#[test]
fn testlog() {
    log_init();
    trace!("trace>>>>", "aaaaaaaaa", 1, 2, 3, 4);
    debug!("debug>>>>", "bbbbbbbbb", 1, 2, 3, 5);
    info!("info>>>>", "ccccccccc", 1, 2, 3, 5);
    warn!("warn>>>>", "dddddddddd", 1, 2, 3, 6);
    error!("error>>>>", "eeeeeeee", 1, 2, 3, 7);
    fatal!("fatal>>>>", "ffffffff", 1, 2, 3, 8);
    thread::sleep(Duration::from_secs(1))
}

#[test]
fn testthreads() {
    log_init();
    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || {
                debug!("testthreads", i, format!("{:?}", Instant::now()));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn testmultilog() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true)
        .set_formatter("{message} | {time} {file}{level}\n");
    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();
    traces!(log, "traces>>>>", "AAAAAAAAA", 1, 2, 3, 4);

    debugs!(log, "debugs>>>>", "BBBBBBBBB", 1, 2, 3, 5);
    infos!(log, "infos>>>>", "CCCCCCCCC", 1, 2, 3, 5);
    warns!(log, "warns>>>>", "DDDDDDDDDD", 1, 2, 3, 6);
    errors!(log, "errors>>>>", "EEEEEEEE", 1, 2, 3, 7);
    fatals!(log, "fatals>>>>", "FFFFFFFF", 1, 2, 3, 8);

    thread::sleep(Duration::from_secs(1))
}

#[test]
fn testformats() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogs.log", MODE::DAY, 10, true);
    let mut logger = Arc::clone(&Arc::new(Mutex::new(log)));
    let log = logger.borrow_mut();

    let v = vec![1, 2, 3];
    tklog::formats!(log, LEVEL::Debug, "Debug>>>{},{}>>>{:?}", 1, 2, v);

    let v2 = vec!['a', 'b'];
    tklog::formats!(log, LEVEL::Info, "Info>>>{},{}>>{:?}", 1, 2, v2);
    tklog::formats!(log, LEVEL::Warn, "Warn>>>{},{}", 1, 2);
    tklog::formats!(log, LEVEL::Error, "Error>>>{},{}", 1, 2);
    tklog::formats!(log, LEVEL::Fatal, "Fatal>>>{},{}", 1, 2);

    thread::sleep(Duration::from_secs(1))
}

#[test]
fn testlogssize() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_size("tklogsize.log", 10 << 20, 10, false);
    let logger = Arc::new(Mutex::new(log));
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let mut log = logger.clone();
            thread::spawn(move || {
                let logmut = log.borrow_mut();
                for _ in 0..10 {
                    debugs!(logmut, "debugs>>>>", "thread>>", i);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn testlogstime() {
    let mut log = Logger::new();
    log.set_console(true)
        .set_level(LEVEL::Debug)
        .set_cutmode_by_time("tklogtime.log", MODE::DAY, 10, false);
    let logger = Arc::new(Mutex::new(log));
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let mut log = logger.clone();
            thread::spawn(move || {
                let logmut = log.borrow_mut();
                for _ in 0..10 {
                    debugs!(logmut, "debugs>>>>", "thread>>", i);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
