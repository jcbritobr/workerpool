use std::{sync::Arc, sync::Barrier};

use workerpool::pool;

#[test]
fn should_all_packages_work_together() {
    let pool = pool::WorkerPool::new(5);
    let njobs = 5;
    let nworkers = 7;
    let barrier = Arc::new(Barrier::new(njobs + 1));

    assert!(njobs <= nworkers, "too many jobs will deadlock");

    for i in 0..njobs {
        let b = barrier.clone();
        pool.execute(Box::new(move || {
            println!("thread id {}", i);
            b.wait();
        }));
    }
    barrier.wait();
}
