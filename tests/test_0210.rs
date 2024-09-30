use tklog::{handle::FileTimeMode, LogOption};

mod module1 {
    use std::{thread, time::Duration};
    pub fn testmod() {
        tklog::debug!("module1,tklog api,LOG debug log>>", 1111);
        tklog::info!("module1,tklog api,LOG info log>>", 2222);
        thread::sleep(Duration::from_secs(1))
    }
    pub mod m1 {
        pub fn testmod() {
            tklog::debug!("m1,tklog api,LOG debug log>>", 123);
        }
    }
    pub mod m2 {
        pub fn testmod() {
            tklog::debug!("m2,tklog api,LOG debug log>>", 123);
        }
    }
}

mod module2 {
    use std::{thread, time::Duration};
    pub fn testmod() {
        tklog::debug!("module2,tklog api,LOG debug log>>", 123);
        tklog::info!("module2,tklog api,LOG info log>>", 456);
        thread::sleep(Duration::from_secs(1))
    }
}

#[test]
fn testmod() {
    tklog::LOG.set_mod_option("test_0210::module1::*", LogOption { level: None, format: None, formatter: None, console: Some(true), fileoption: Some(Box::new(FileTimeMode::new("syncmodule1.log", tklog::MODE::DAY, 0, true))) });
    module1::testmod();
    module1::m1::testmod();
    module1::m2::testmod();
    module2::testmod();
}

mod module3 {
    pub async fn testmod() {
        tklog::async_debug!("async module3,tklog api,LOG debug log>>", 1111);
        tklog::async_info!("async module3,tklog api,LOG debug log>>", 2222);
    }
    pub mod m3 {
        pub async fn testmod() {
            tklog::async_debug!("async m1,tklog api,LOG debug log>>", 3333);
        }
    }
    pub mod m4 {
        pub async fn testmod() {
            tklog::async_info!("async m2,tklog api,LOG debug log>>", 4444);
        }
    }
}

mod module4 {
    pub async fn testmod() {
        tklog::async_debug!("async module4,tklog api,LOG debug log>>", 1111);
        tklog::async_info!("async module4,tklog api,LOG info log>>", 2222);
        log::debug!("async module4,log api,debug log>>{}", 3333);
        log::info!("async module4,log api,info log>>{}", 4444);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[tokio::test]
async fn testasyncmod() {
    tklog::ASYNC_LOG.set_mod_option("test_0210::module3::*", LogOption { level: None, format: None, formatter: None, console: Some(true), fileoption: Some(Box::new(FileTimeMode::new("asyncmodule2.log", tklog::MODE::DAY, 0, true))) }).await;
    module3::testmod().await;
    module3::m3::testmod().await;
    module3::m4::testmod().await;
    module4::testmod().await;
}
