## Workerpool
A simple rust workerpool implementation. It uses channels to synchronize the jobs.


* Use
```rust
use std::sync::mpsc,

use workerpool::pool;

fn main() {

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
```
