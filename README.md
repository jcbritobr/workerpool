## Workerpool
A simple rust workerpool implementation that uses channels to synchronize the jobs. It can spawn a fixed number of worker threads, that waits for a job queue to consum.


* Use
```rust
use std::sync::mpsc,

use workerpool_rs::pool;

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

* Test

```shell
$ cargo test
```