use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// Note: advanced feature - type alias
// Define a type alias 'Job' for a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    // 1. the thread pool contains a vector of worker threads
    workers: Vec<Worker>,
    // 2. the thread pool also includes a sender to send tasks to the worker threads
    // the sender is used to send jobs to the workers via a channel.
    // Why do we use Option here?
    // Because we want to be able to take ownership of the sender when we drop the ThreadPool,
    // allowing us to close the channel and signal the workers to stop.
    // More about using Option to retain ownership in misc/use_option_to_move_ownership.md
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        // this receiver is shared among multiple worker threads,
        // hence we need to wrap it in an Arc to ensure thread-safe shared ownership
        // also use Mutex to ensure that only one worker can access the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("ThreadPool::execute called");
        // start sending the job to the worker threads
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // signal all workers to shut down by dropping the sender
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Enable debug printing for Worker struct
#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread_handle = thread::spawn(move || {
            loop {
                // lock the receiver to get a job
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        // release the lock before executing the job, such that other workers can get jobs
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Err(_) => {
                        println!(
                            "Sender indicated no more messages, stop pulling for worker {}.",
                            id
                        );
                        break;
                    }
                }
            }
            // Note: why do we use loop instead of while let Some(job) = receiver.lock().unwrap().recv().ok() ?
            // the while let (also if let) statement will hold the lock for the entire duration of the job execution,
            // using loop allows us to release the lock after receiving the job and before executing it.
        });
        Worker {
            id,
            thread: Some(thread_handle),
        }
    }
}
