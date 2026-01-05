# Notes on Tokio and Async Rust

Mainly from https://tokio.rs/tokio/tutorial.

## Concurrency v.s. Parallelism

Concurrency is the ability of a system to handle multiple tasks at the same time, while parallelism is the ability to execute multiple tasks simultaneously.

Using an analogy, concurrency is like a single chef preparing multiple dishes by switching between them, while parallelism is like having multiple chefs working on different dishes at the same time.

Tokio is designed to handle concurrency.

## When NOT to use Tokio

* If your application is CPU-bound (instead of IO-bound) and requires heavy computation, using Tokio may not be the best choice.
* If your application is simple and does not require asynchronous operations, using Tokio may add unnecessary complexity.
* If your application relies heavily on blocking operations, using Tokio may lead to performance issues.

## Spawn

A tokio `task` is an async task that is executed by the Tokio runtime. It's created by `tokio::spawn`.

### the function signature

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
* `Send` bound to ensure that the future and its output can be safely transferred across thread boundaries. It ensures that when we switch between tasks, the intermediate state of the future can be safely persisted in a storage, such that later, when a new task is spawned, the state can be restored. More details in https://tokio.rs/tokio/tutorial/spawning#send-bound.

More details at https://tokio.rs/tokio/tutorial/spawning. Make sure understand the `yield_now` example.

## tokio IO

https://tokio.rs/tokio/tutorial/io

Main learning points:

* If we use a buffer to read and write data asynchronously, we need to ensure the buffer can be sent across threads. Therefore, we use `Vec` instead of stack-allocated arrays.
* EOF - when reading from a stream, if we get 0 bytes read, it indicates that the stream has reached the end (EOF). We need to return from the read loop to avoid busy-waiting.

## How to run the server and client

`cargo run --bin server` and `cargo run --bin client`
