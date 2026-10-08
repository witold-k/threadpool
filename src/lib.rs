// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

pub mod threadpool;

use crossbeam_channel::Sender;
use std::thread;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPoolError {
    NoWorkers,
    Disconnected,
    WorkerPanicked,
}

/// A fixed-size pool of persistent worker threads.
pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}
