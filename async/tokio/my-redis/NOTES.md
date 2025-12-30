# Notes on Tokio and Async Rust

## Concurrency v.s. Parallelism

Concurrency is the ability of a system to handle multiple tasks at the same time, while parallelism is the ability to execute multiple tasks simultaneously.

Using an analogy, concurrency is like a single chef preparing multiple dishes by switching between them, while parallelism is like having multiple chefs working on different dishes at the same time.

Tokio is designed to handle concurrency.

## When NOT to use Tokio

* If your application is CPU-bound (instead of IO-bound) and requires heavy computation, using Tokio may not be the best choice.
* If your application is simple and does not require asynchronous operations, using Tokio may add unnecessary complexity.
* If your application relies heavily on blocking operations, using Tokio may lead to performance issues.

## tokio spawn

A tokio `task` is an async task that is executed by the Tokio runtime. It's created by `tokio::spawn`.

## the function signature

```rust

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // implementation details
}
```

* `'static` bound to ensure that the future and its output can live for the entire duration of the program.
* `Send` bound to ensure that the future and its output can be safely transferred across thread boundaries.

More details at https://tokio.rs/tokio/tutorial/spawning. Make sure understand the `yield_now` example.