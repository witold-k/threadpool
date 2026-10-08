# threadpool

A small, fixed-size thread pool for Rust, backed by [crossbeam-channel](https://crates.io/crates/crossbeam-channel).

The pool keeps a fixed number of worker threads alive and accepts `FnOnce() + Send + 'static` jobs through an unbounded queue.

## Usage

Add this repository as a Git dependency:

```toml
[dependencies]
threadpool = { git = "https://github.com/witold-k/threadpool" }
```

```rust
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use threadpool::ThreadPool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let completed = Arc::new(AtomicUsize::new(0));
    let pool = ThreadPool::new(4)?;

    for _ in 0..10 {
        let completed = Arc::clone(&completed);
        pool.execute(move || {
            completed.fetch_add(1, Ordering::Relaxed);
        })?;
    }

    // Consumes the pool and waits until its queued jobs have completed.
    pool.join()?;
    assert_eq!(completed.load(Ordering::Relaxed), 10);
    Ok(())
}
```

## API

- `ThreadPool::new(num_threads)` creates exactly that many worker threads; zero returns `ThreadPoolError::NoWorkers`.
- `execute(job)` queues a job; it returns `ThreadPoolError::Disconnected` if the channel is unavailable.
- `join(self)` closes the queue, waits for workers to exit, and returns `ThreadPoolError::WorkerPanicked` if a worker panicked.
- Dropping the pool also closes the queue and waits for the workers, but does not report worker panics.

**Important:** A panic in a job terminates its worker; it is not restarted. Jobs that block indefinitely can prevent `join` or `drop` from returning. The unbounded queue applies no backpressure.

## Development

Requires a Rust toolchain that supports edition 2024.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

CI runs formatting, Clippy and tests on GitHub Actions.

## License

Apache-2.0. See [LICENSE](LICENSE).
