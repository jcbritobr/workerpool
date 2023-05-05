## Workerpool
A simple rust workerpool implementation that uses channels to synchronize the jobs. It can spawn a fixed number of worker threads, that waits for a job queue to consum.


* Use
```rust
 use workerpool_rs::pool::WorkerPool;
 use std::sync::mpsc::channel;
 use std::sync::{Arc, Mutex};

 let n_workers = 4;
 let n_jobs = 8;
 let pool = WorkerPool::new(n_workers);

 let (tx, rx) = channel();
 let atx = Arc::new(Mutex::new(tx));
 for _ in 0..n_jobs {
     let atx = atx.clone();
     pool.execute(move|| {
         let tx = atx.lock().unwrap();
         tx.send(1).expect("channel will be there waiting for the pool");
     });
 }

 assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
```

* Test

```shell
$ cargo test
```