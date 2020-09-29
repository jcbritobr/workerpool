//! ## Worker Pool
//!
//! This module contains constructs for dealing with concurrent tasks. It can spawn
//! any number of worker threads and sync them with other channels.
//!
//! ## Examples
//!
//! ### Synchronized with other channels
//!
//! ```
//! use workerpool::pool::WorkerPool;
//! use std::sync::mpsc::channel;
//!
//! let n_workers = 4;
//! let n_jobs = 8;
//! let pool = WorkerPool::new(n_workers);
//!
//! let (tx, rx) = channel();
//! for _ in 0..n_jobs {
//!     let tx = tx.clone();
//!     pool.execute(Box::new(move|| {
//!         tx.send(1).expect("channel will be there waiting for the pool");
//!     }));
//! }
//!
//! assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
//!```
//!
//! ### Sinchronized with Barrier
//!```
//! 
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::sync::{Arc, Barrier};
//! use workerpool::pool::WorkerPool;
//!
//! let n_workers = 42;
//! let n_jobs = 23;
//! let pool = WorkerPool::new(n_workers);
//! let an_atomic = Arc::new(AtomicUsize::new(0));
//!
//! assert!(n_jobs <= n_workers, "too many jobs, will deadlock");
//!
//! let barrier = Arc::new(Barrier::new(n_jobs + 1));
//! for _ in 0..n_jobs {
//!     let barrier = barrier.clone();
//!     let an_atomic = an_atomic.clone();
//! 
//!     pool.execute(Box::new(move|| {
//!         // do the heavy work
//!         an_atomic.fetch_add(1, Ordering::Relaxed);
//! 
//!         // then wait for the other threads
//!         barrier.wait();
//!     }));
//! }
//!
//! barrier.wait();
//! assert_eq!(an_atomic.load(Ordering::SeqCst), /* n_jobs = */ 23);
//!```


// Imports and makes pool public.
pub mod pool;
