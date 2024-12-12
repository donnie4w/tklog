use std::{thread, time::Duration};
use tklog::{async_debug, async_error, async_fatal, async_info, async_trace, async_warn, ASYNC_LOG, LOG};
use tklog::{debug, error, fatal, info, trace, warn, LEVEL};

#[test]
fn testlog() {
    LOG.set_console(true).set_cutmode_by_size("028test.log", 1 << 20, 0, false).set_level(LEVEL::Trace).set_attr_format(|fmt| {
        fmt.set_console_body_fmt(|level, body| {
            //处理body的末尾换行符
            let trimmed_body = if body.ends_with('\n') { format!("{}{}", body.as_str()[..body.len() - 1].to_string(), "\x1b[0m\n") } else { format!("{}{}", body, "\x1b[0m\n") };

            match level {
                LEVEL::Trace => format!("{}{}", "\x1b[34m", trimmed_body), //蓝色
                LEVEL::Debug => format!("{}{}", "\x1b[36m", trimmed_body), //青色
                LEVEL::Info => format!("{}{}", "\x1b[32m", trimmed_body),  //绿色
                LEVEL::Warn => format!("{}{}", "\x1b[33m", trimmed_body),  //黄色
                LEVEL::Error => format!("{}{}", "\x1b[31m", trimmed_body), //红色
                LEVEL::Fatal => format!("{}{}", "\x1b[41m", trimmed_body), //背景红
                LEVEL::Off => "".to_string(),
            }
        });
    });

    trace!("trace!", "this is sync log");
    debug!("debug!", "this is sync log");
    info!("info!", "this is sync log");
    warn!("warn!", "this is sync log");
    error!("error!", "this is sync log");
    fatal!("fata!", "this is sync log");
    thread::sleep(Duration::from_secs(1))
}


#[test]
fn testlog2() {
    LOG.set_console(true).set_cutmode_by_size("028test2.log", 1 << 20, 0, false).set_level(LEVEL::Trace).set_attr_format(|fmt| {
        fmt.set_console_body_fmt(|level, body| {
            //处理body的末尾换行符
            let trimmed_body = if body.ends_with('\n') { format!("{}{}", body.as_str()[..body.len() - 1].to_string(), "\x1b[0m\n") } else { format!("{}{}", body, "\x1b[0m\n") };

            match level {
                LEVEL::Trace => format!("{}{}", "\x1b[34m", trimmed_body), //蓝色
                LEVEL::Debug => format!("{}{}", "\x1b[36m", trimmed_body), //青色
                LEVEL::Info => format!("{}{}", "\x1b[32m", trimmed_body),  //绿色
                LEVEL::Warn => format!("{}{}", "\x1b[33m", trimmed_body),  //黄色
                LEVEL::Error => format!("{}{}", "\x1b[31m", trimmed_body), //红色
                LEVEL::Fatal => format!("{}{}", "\x1b[41m", trimmed_body), //背景红
                LEVEL::Off => "".to_string(),
            }
        });

        fmt.set_body_fmt(|level, body| {
            //处理body的末尾换行符
            let trimmed_body = if body.ends_with('\n') { format!("{}{}", body.as_str()[..body.len() - 1].to_string(), "\x1b[0m\n") } else { format!("{}{}", body, "\x1b[0m\n") };
            match level {
                LEVEL::Trace => format!("{}{}", "\x1b[44m", trimmed_body), //背景蓝色
                LEVEL::Debug => format!("{}{}", "\x1b[46m", trimmed_body), //背景青色
                LEVEL::Info => format!("{}{}", "\x1b[42m", trimmed_body),  //背景绿色
                LEVEL::Warn => format!("{}{}", "\x1b[43m", trimmed_body),  //背景黄色
                LEVEL::Error => format!("{}{}", "\x1b[41m", trimmed_body), //背景红色
                LEVEL::Fatal => format!("{}{}", "\x1b[45m", trimmed_body), //背景紫色
                LEVEL::Off => "".to_string(),
            }
        });
    });

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
    ASYNC_LOG.set_level(LEVEL::Trace).set_console(true).set_cutmode_by_time("async028.log", tklog::MODE::DAY, 0, false).await.set_attr_format(|fmt| {
        fmt.set_console_body_fmt(|level, body| {
            //处理body的末尾换行符
            let trimmed_body = if body.ends_with('\n') { format!("{}{}", body.as_str()[..body.len() - 1].to_string(), "\x1b[0m\n") } else { format!("{}{}", body, "\x1b[0m\n") };
            match level {
                LEVEL::Trace => format!("{}{}", "\x1b[34m", trimmed_body), //蓝色
                LEVEL::Debug => format!("{}{}", "\x1b[36m", trimmed_body), //青色
                LEVEL::Info => format!("{}{}", "\x1b[32m", trimmed_body),  //绿色
                LEVEL::Warn => format!("{}{}", "\x1b[33m", trimmed_body),  //黄色
                LEVEL::Error => format!("{}{}", "\x1b[31m", trimmed_body), //红色
                LEVEL::Fatal => format!("{}{}", "\x1b[41m", trimmed_body), //背景红
                LEVEL::Off => "".to_string(),
            }
        });

        fmt.set_body_fmt(|level, body| {
            //处理body的末尾换行符
            let trimmed_body = if body.ends_with('\n') { format!("{}{}", body.as_str()[..body.len() - 1].to_string(), "\x1b[0m\n") } else { format!("{}{}", body, "\x1b[0m\n") };
            match level {
                LEVEL::Trace => format!("{}{}", "\x1b[44m", trimmed_body), //背景蓝色
                LEVEL::Debug => format!("{}{}", "\x1b[46m", trimmed_body), //背景青色
                LEVEL::Info => format!("{}{}", "\x1b[42m", trimmed_body),  //背景绿色
                LEVEL::Warn => format!("{}{}", "\x1b[43m", trimmed_body),  //背景黄色
                LEVEL::Error => format!("{}{}", "\x1b[41m", trimmed_body), //背景红色
                LEVEL::Fatal => format!("{}{}", "\x1b[45m", trimmed_body), //背景紫色
                LEVEL::Off => "".to_string(),
            }
        });
    });

    async_trace!("trace!", "this is async log");
    async_debug!("debug!", "this is async log");
    async_info!("info!", "this is async log");
    async_warn!("warn!", "this is async log");
    async_error!("error!", "this is async log");
    async_fatal!("fata!", "this is async log");

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
