// A quote from Golang blog:
// "Do not communicate by sharing memory; instead, share memory by communicating."

fn main() {
    simple_channel_example();
    send_multiple_values();
    multiple_producers_example();
}

fn simple_channel_example() {
    // multiple producer, single consumer
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hello from thread");
        // Note: the ownership of val is moved into the channel
        tx.send(val).unwrap();
        // println!("Sent value: {}", val); // -> compile error: borrow of moved value: `val`
    });
    // Note: recv() is a blocking call that waits for a value to be sent.
    // The ownership of the received value is moved out of the channel to the receiver rx.
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}

fn send_multiple_values() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("Message 1"),
            String::from("Message 2"),
            String::from("Message 3"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Note: receiver can be used as an iterator.
    for received in rx {
        println!("Received: {}", received);
    }
}

fn multiple_producers_example() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    // Clone the transmitter to create multiple producers
    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("Producer 1 - Message 1"),
            String::from("Producer 1 - Message 2"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("Producer 2 - Message 1"),
            String::from("Producer 2 - Message 2"),
        ];

        for val in values {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Drop the original transmitter to close the channel when all producers are done.
    //
    // * Why is this necessary?
    // Because the receiver will keep waiting for messages as long as there is at least one transmitter alive.
    //
    // * How does it work with spawned threads using the clones?
    // The cloned transmitters in the spawned threads will keep the channel open until they are dropped.
    //
    // * Does it mean that even that we dropped the original transmitter here, the clones in the threads are still alive?
    // Yes, exactly. The channel will only close when all transmitters (original and clones) are dropped.
    //
    // * What's the sequence of dropping here?
    // 1. The original transmitter is dropped here.
    // 2. Each spawned thread will drop its cloned transmitter when the thread finishes executing.
    // 3. Once all transmitters are dropped, the channel is closed, and the receiver will stop waiting for messages.
    drop(tx);

    for received in rx {
        println!("Received: {}", received);
    }
}