use std::{
    sync::atomic::AtomicUsize, sync::atomic::Ordering, sync::mpsc, sync::Arc, sync::Barrier,
};

use workerpool::pool;

#[test]
fn pool_should_sum_atomic_variable() {
    let njobs = 20;
    let nworkers = 42;
    let pool = pool::WorkerPool::new(nworkers);
    let atomic = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(njobs + 1));

    assert!(njobs <= nworkers, "too many jobs will deadlock");

    for _ in 0..njobs {
        let b = barrier.clone();
        let atomic = atomic.clone();
        pool.execute(Box::new(move || {
            atomic.fetch_add(1, Ordering::Relaxed);
            b.wait();
        }));
    }
    barrier.wait();
    assert_eq!(atomic.load(Ordering::SeqCst), njobs);
}

#[test]
fn pool_should_synchronize_sender_and_receiver_and_fold_results() {
    let nworkers = 4;
    let njobs = 8;

    let pool = pool::WorkerPool::new(nworkers);

    let (tx, rx) = mpsc::channel();

    for _ in 0..njobs {
        let tx = tx.clone();
        pool.execute(Box::new(move || {
            tx.send(1).expect("channel waiting for pool");
        }));
    }

    assert_eq!(rx.iter().take(njobs).fold(0, |a, b| a + b), njobs);
}
