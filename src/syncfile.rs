// Copyright (c) 2024, donnie4w <donnie4w@gmail.com>
// All rights reserved.
// https://github.com/donnie4w/tklog
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    env,
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{self, Error, ErrorKind, Write},
    path::{Path, PathBuf},
};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    getbackup_with_time, gzip, handle::FileOption, passtimemode, threadPool::ThreadPool, timesec,
    ErrCode, CUTMODE, MODE,
};

pub struct FileHandler {
    filename: String, //Log file path
    max_size: u64,    //Maximum size for each log file to be saved
    max_backups: u32, //The maximum number of old log files that can be retained
    compress: bool,   //Whether to compress old log files
    cutmode: CUTMODE,
    timemode: MODE,
    filesize: u64,
    filehandle: File,
    startsec: u64,
}

impl FileHandler {
    pub fn new(option: Box<dyn FileOption>) -> Result<Self, Error> {
        let fo = &*option;
        let filename = fo.filename();
        let log_path = Path::new(&filename);
        let _ = mkdirs(log_path);

        let file = Self::newfile(filename.clone());

        if file.is_err() {
            return Err(file.err().unwrap());
        }

        let f = file.unwrap();
        let metadata = f.metadata()?.modified()?;
        let startsec = metadata.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        let fh = FileHandler {
            filename: fo.filename(),
            max_size: fo.size(),
            max_backups: fo.maxbackups(),
            compress: fo.compress(),
            cutmode: fo.mode(),
            timemode: fo.timemode(),
            filesize: fs::metadata(&log_path)?.len(),
            filehandle: f,
            startsec,
        };
        Ok(fh)
    }

    pub fn new_from_clone(&mut self) -> io::Result<()> {
        let filename = self.filename.clone();
        let log_path = Path::new(&filename);
        mkdirs(log_path)?;
        let file = Self::newfile(filename)?;
        self.filesize = 0;
        self.filehandle = file;
        Ok(())
    }

    fn newfile(filename: String) -> io::Result<File> {
        OpenOptions::new().append(true).create(true).open(filename)
    }

    fn rename(&self) -> io::Result<()> {
        let log_path = Path::new(&self.filename);
        match self.cutmode {
            CUTMODE::TIME => rename(
                &log_path,
                self.compress,
                self.max_backups,
                Some(getbackup_with_time(self.startsec, self.timemode)),
            ),
            CUTMODE::SIZE => rename(&log_path, self.compress, self.max_backups, None),
        }
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        match self.cutmode {
            CUTMODE::TIME => {
                if passtimemode(self.startsec, self.timemode) {
                    if let Ok(_) = self.rename() {
                        let _ = self.new_from_clone();
                        self.startsec = timesec();
                    }
                }
            }
            CUTMODE::SIZE => {
                if self.max_size > 0 && self.filesize + data.len() as u64 > self.max_size {
                    if let Ok(_) = self.rename() {
                        let _ = self.new_from_clone();
                    }
                }
            }
        }
        self.filehandle.write(data)?;
        self.filesize += data.len() as u64;
        Ok(())
    }
}

fn mkdirs(dir_path: &Path) -> io::Result<()> {
    let parent_dir = dir_path
        .parent()
        .ok_or_else(|| Error::new(ErrorKind::Other, ErrCode::NotFound.to_string()))?;
    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?;
    }
    Ok(())
}

static POOL: Lazy<ThreadPool> = Lazy::new(|| ThreadPool::new(4));

fn rename(
    log_path: &Path,
    compress: bool,
    maxbackup: u32,
    backupsuffix: Option<String>,
) -> io::Result<()> {
    let mut counter = 1;
    let file_stem = log_path.file_stem().unwrap_or_else(|| OsStr::new("tklog"));
    let extension = log_path
        .extension()
        .map_or("", |e| e.to_str().unwrap())
        .to_owned();
    let mut maxloop = 1 << 20;
    while maxloop > 0 {
        let mut parent = log_path
            .parent()
            .ok_or_else(|| Error::new(ErrorKind::Other, ErrCode::NotFound.to_string()))?
            .to_path_buf();

        if parent.as_os_str().is_empty() {
            if let Ok(current_dir) = env::current_dir() {
                parent = current_dir;
            }
        }

        maxloop -= 1;
        let mut suffix = String::new();
        if !extension.is_empty() {
            suffix.push('.');
            suffix.push_str(extension.as_str());
        }

        let new_name: String;

        if backupsuffix.is_some() {
            new_name = format!(
                "{}_{}_{}{}",
                file_stem.to_string_lossy(),
                backupsuffix.as_ref().unwrap(),
                counter,
                suffix
            );
        } else {
            new_name = format!("{}_{}{}", file_stem.to_string_lossy(), counter, suffix);
        }

        let new_path = parent.join(&new_name);

        let new_path_gz = parent.join(format!("{}.gz", new_path.display().to_string()));
        if !new_path.exists() && !new_path_gz.exists() {
            let r = fs::rename(log_path, &new_path);
            if r.is_err() && maxloop <= 0 {
                return Err(r.err().unwrap());
            } else {
                let p = parent.clone();
                let e = extension.clone();
                let fname = file_stem.to_string_lossy().to_string().clone();
                POOL.execute(move || {
                    if compress {
                        let _ = gzip(new_path.to_str().unwrap());
                    }
                    if maxbackup > 0 {
                        let _ = maxbackup_with_size(&p, e, fname, maxbackup);
                    }
                });
                return Ok(());
            }
        }
        counter += 1;
    }
    Ok(())
}

fn filter_files(
    dir_path: &Path,
    extension: String,
    filename: String,
    maxbackup: u32,
) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut sortvec = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let md = entry.metadata()?;
        let sec = md
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)
            .expect("")
            .as_secs();

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            let mut suffix = String::new();
            if !extension.is_empty() {
                suffix.push_str("\\.");
                suffix.push_str(extension.as_str());
            }
            let parrent = format!(
                "{}{}{}{}{}{}",
                "^", filename, "(_\\d+){0,}", "_\\d+", suffix, "(\\.gz){0,}$"
            );
            let re = Regex::new(parrent.as_str()).unwrap();
            if re.is_match(file_name) {
                sortvec.push((sec, path.clone()))
            }
        }
    }
    sortvec.sort_by(|a, b| a.0.cmp(&b.0));
    if sortvec.len() > maxbackup as usize {
        for tuple in sortvec.iter().take(sortvec.len() - maxbackup as usize) {
            files.push(tuple.1.clone());
        }
    }
    Ok(files)
}

fn delete_files(files: Vec<PathBuf>) -> io::Result<()> {
    Ok(for file in files {
        fs::remove_file(file)?;
    })
}

fn maxbackup_with_size(
    parant: &PathBuf,
    extension: String,
    filename: String,
    maxbackup: u32,
) -> io::Result<()> {
    let matched_files = filter_files(parant, extension, filename, maxbackup)?;
    delete_files(matched_files)
}
