//! WorkerPool.
//!
//! With this pool, we are able to synchronize channels, 
//! start jobs, wait for workers, and many others concurrent
//! tasks are made easy.

use std::{
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread,
};

// Basic types for concurrent tasks
type Job = Box<dyn FnOnce() + Send + 'static>;
type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
type Handle = thread::JoinHandle<()>;

/// Implements a continuous pool of rust threads thats doesn't stops
/// unless it gets out of scope.
/// 
pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
    /// Constructs a new WorkerPool of size x.
    ///
    /// **size**: usize - Is the number of workers in WorkerPool object. \
    /// **returns**: a WorkerPool object.
    ///
    /// # Examples
    ///
    /// ```
    /// use workerpool::pool::WorkerPool;
    ///
    /// let pool = WorkerPool::new(3);
    ///
    /// ``` 
    pub fn new(size: usize) -> WorkerPool {
        let (tx, rx) = mpsc::channel();
        let mut workers = Vec::<Worker>::with_capacity(size);
        let rec = Arc::new(Mutex::new(rx));
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rec)));
        }
        
        WorkerPool {
            workers,
            sender: tx,
        }
    }

    /// Executes a job. The job is moved to closure, as this function is FnOnce. \
    ///
    /// **f**: A FnOnce closure hosted by a Box smart pointer.
    /// ## Examples
    ///
    /// ```
    /// use workerpool::pool::WorkerPool;
    ///
    /// let pool = WorkerPool::new(1);
    /// pool.execute(Box::new(move || {
    ///    println!("this is a job.");
    /// }));
    ///
    /// ```
    pub fn execute(&self, f: Job) {
        let job = Box::new(f);
        self.sender.send(job).expect("Cant send job");
    }
}

// Implements Display for WorkerPool. This is usefull as we can able
// to compare and make unit tests more easily.
impl Display for WorkerPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for i in &self.workers {
            buffer.push_str(&i.to_string());
        }
        write!(f, "workers[] = {}", buffer)
    }
}

// A structure that holds an id and thread handle.
// 
// id: usize - An id for worker indentification.\
// handle: JoinHandle<()> - a handle that has a working thread.
struct Worker {
    id: usize,
    _handle: Handle,
}

impl Worker {
    // Constructs a new Worker.
    //
    // id: usize - Worker identificator.
    // handle: JoinHandle<()> - a thread handle.
    fn new(id: usize, handle: JobReceiver) -> Worker {
        let handle = thread::spawn(move || loop {
            let job = match handle
                .lock()
                .expect("Cant acquire lock")
                .recv() {
                    Ok(data) => data,
                    Err(_) => continue,
                };

            job();
        });

        Worker { id, _handle: handle }
    }
}

// Implements Display for Worker as this simplifys test writing.
impl Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id: {})", self.id,)
    }
}

// This sections are the biginning of workerspool module unit tests
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn worker_should_return_new() {
        let (_, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let w = Worker::new(1, Arc::clone(&receiver));
        assert_eq!("(id: 1)", w.to_string());
    }

    #[test]
    fn workerpool_should_return_new() {
        let expected = "workers[] = (id: 0)(id: 1)(id: 2)".to_string();
        let pool = WorkerPool::new(3);
        assert_eq!(expected.to_string(), pool.to_string());
    }

    #[test]
    fn workerpool_should_execute_job_succeed() {
        let pool = WorkerPool::new(1);
        for _ in 0 .. 10000 {
            pool.execute(Box::new(||{
                let _sum = 3 + 1;
            }));
        }
    }
}
