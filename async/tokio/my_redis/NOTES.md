# Notes on Tokio and Async Rust

Mainly from https://tokio.rs/tokio/tutorial/

## Concurrency v.s. Parallelism

Concurrency is the ability of a system to handle multiple tasks at the same time, while parallelism is the ability to execute multiple tasks simultaneously.

Using an analogy, concurrency is like a single chef preparing multiple dishes by switching between them, while parallelism is like having multiple chefs working on different dishes at the same time.

Tokio is designed to handle concurrency.

## When NOT to use Tokio

- If your application is CPU-bound (instead of IO-bound) and requires heavy computation, using Tokio may not be the best choice.
- If your application is simple and does not require asynchronous operations, using Tokio may add unnecessary complexity.
- If your application relies heavily on blocking operations, using Tokio may lead to performance issues.

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

- `'static` bound to ensure that the future and its output can live for the entire duration of the program.
- `Send` bound to ensure that the future and its output can be safely transferred across thread boundaries. It ensures that when we switch between tasks, the intermediate state of the future can be safely persisted in a storage, such that later, when a new task is spawned, the state can be restored. More details in https://tokio.rs/tokio/tutorial/spawning#send-bound.

More details at https://tokio.rs/tokio/tutorial/spawning. Make sure understand the `yield_now` example.

## tokio IO

https://tokio.rs/tokio/tutorial/io

Main learning points:

- If we use a buffer to read and write data asynchronously, we need to ensure the buffer can be sent across threads. Therefore, we use `Vec` instead of stack-allocated arrays.
- EOF - when reading from a stream, if we get 0 bytes read, it indicates that the stream has reached the end (EOF). We need to return from the read loop to avoid busy-waiting.

## async in depth

Details are in https://tokio.rs/tokio/tutorial/async. Below is a brief summary:

### `async` functions return a future.

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context)
        -> Poll<Self::Output>;
}
```

- The associated type `Output` represents the type of the value that the future will produce when it completes.
- The `poll` method is invoked by the Tokio runtime, from time to time, to check the status of the future.
- The return value `Poll`, if `Poll::Pending`, it means the future is not ready yet and the Tokio runtime should schedule it to be polled again later. If `Poll::Ready`, it means the future has completed and the Tokio runtime should resume the task that is waiting on the future.
- Why `Pin`? `Pin` is used to prevent the value from being moved while it is being polled. This is important because the future may be polled multiple times, and we need to ensure that the state of the future is consistent across all polls.
- What is `Context`? `Context` is used to provide information about the current task to the future. It includes the task's wake-up function, which is used to schedule the task to be polled again when it is ready.

Rust futures are state machines.

### The high-level design of async

- Asynchronous Rust operations are lazy and require a caller to poll them.
- Wakers are passed to futures to link a future to the task calling it.
- When a resource is not ready to complete an operation, Poll::Pending is returned and the task's waker is recorded.
- When the resource becomes ready, the task's waker is notified.
- The executor receives the notification and schedules the task to execute.
- The task is polled again, this time the resource is ready and the task makes progress.

## `tokio::select!`

https://tokio.rs/tokio/tutorial/select

The `tokio::select!` macro allows waiting on multiple async computations and returns when a single computation completes.

A tokio select runs on a single thread as a single task, hence, it can only run one computation at a time. Whereas, a tokio spawn creates a new task and runs it concurrently with other tasks.

## How to run the server and client

`cargo run --bin server` and `cargo run --bin client`
