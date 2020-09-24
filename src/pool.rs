use std::{
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
type Handle = thread::JoinHandle<()>;


pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
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

    pub fn execute(&self, f: Job) {
        let job = Box::new(f);
        self.sender.send(job).expect("Cant send job");
    }
}

impl Display for WorkerPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for i in &self.workers {
            buffer.push_str(&i.to_string());
        }
        write!(f, "workers[] = {}", buffer)
    }
}

struct Worker {
    id: usize,
    _handle: Handle,
}

impl Worker {
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

impl Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(id: {})", self.id,)
    }
}

// This sections is the biginning of workerspool module unit tests
//
//
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn should_new_return_worker() {
        let (_, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let w = Worker::new(1, Arc::clone(&receiver));
        assert_eq!("(id: 1)", w.to_string());
    }

    #[test]
    fn should_workerpool_return_new() {
        let expected = "workers[] = (id: 0)(id: 1)(id: 2)".to_string();
        let pool = WorkerPool::new(3);
        assert_eq!(expected.to_string(), pool.to_string());
    }

    #[test]
    fn should_workerpool_execute_job_succeed() {
        let pool = WorkerPool::new(1);
        for _ in 0 .. 10000 {
            pool.execute(Box::new(||{
                let _sum = 3 + 1;
            }));
        }
    }
}
