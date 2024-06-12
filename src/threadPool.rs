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

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::Arc;
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: Sender<Task>,
}

impl ThreadPool {
    pub fn new(mut size: usize) -> ThreadPool {
        if size == 0 {
            size = 1
        }

        let (sender, receiver) = unbounded();
        let receiver = Arc::new(receiver);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            _workers: workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        self.sender.send(task).expect("send error");
    }
}

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Receiver<Task>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(task) = receiver.recv() {
                task();
            }
        });
        Worker {
            _id: id,
            _thread: thread,
        }
    }
}
