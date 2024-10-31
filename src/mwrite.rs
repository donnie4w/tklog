use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex,
};

pub struct MWrite {
    ch: Sender<Vec<u8>>,
    rv: Receiver<Vec<u8>>,
    mu: Mutex<()>,
    wc: AtomicI32,
    cc: AtomicI32,
}

impl MWrite {
    pub fn new() -> MWrite {
        let (tx, rx) = channel();
        let logwrite = MWrite { ch: tx, rv: rx, mu: Mutex::new(()), wc: AtomicI32::new(0), cc: AtomicI32::new(0) };
        logwrite
    }

    pub fn write(&self, file: &mut File, bs: Vec<u8>) -> std::io::Result<usize> {
        let size = bs.len();
        self.cc.fetch_add(1, Ordering::SeqCst);
        if  self.cc.load(Ordering::SeqCst) <= 1 {
            let r = file.write(&bs);
            self.cc.fetch_sub(1, Ordering::SeqCst);
            return r;
        }

        self.wc.fetch_add(1, Ordering::SeqCst);
        if let Err(e) = self.ch.send(bs) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Send error: {}", e)));
        }
        {
            if let Ok( guard) = self.mu.try_lock() {
                let _ = self.write_internal(file);
                drop(guard);
            }
        }
        self.cc.fetch_sub(1, Ordering::SeqCst);
        Ok(size)
    }

    fn write_internal(&self, file: &mut File) -> std::io::Result<()> {
        let mut writer = BufWriter::new(file);
        while let Ok(bs) = self.rv.recv() {
            writer.write_all(&bs).map_err(|e| {
                eprintln!("Failed to write data: {}", e);
                e
            })?;
            self.wc.fetch_sub(1, Ordering::SeqCst);
            if self.wc.load(Ordering::SeqCst) == 0 {
                break;
            }
        }
        writer.flush()
    }
}

// #[test]
// fn testwrite() {
//     let mut file = std::fs::OpenOptions::new().append(true).create(true).open("logfile.log").unwrap();
//     let logger = MWrite::new();

//     let data = b"Log entry".to_vec();
//     logger.write(&mut file, data).unwrap();
// }
