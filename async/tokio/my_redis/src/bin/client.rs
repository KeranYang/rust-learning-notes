use mini_redis::client;
use tokio::sync::oneshot;
use tokio::sync::mpsc;

/// This client implementation demonstrates concurrent GET and SET operations
/// using Tokio message passing through channels.
///
/// At high level the idea is to spawn one manager task and multiple worker tasks.
///
/// The manager task is responsible for receiving requests from workers, sending them to
/// the Redis server, and returning the responses back to the workers.
/// To do this we use a mpsc channel.
///
/// The worker tasks are responsible for sending requests to the manager,
/// waiting for the responses, and processing the responses.
/// To do this we use oneshot channels.

enum Command {
    Get {
        key: String,
        resp: Responder<Option<bytes::Bytes>>,
    },
    Set {
        key: String,
        val: bytes::Bytes,
        resp: Responder<()>,
    },
}

// type alias for the oneshot responder
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // Create the mpsc channel for sending commands to the manager task.
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(32);

    // Spawn the manager task
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = cmd_rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
        // If the cmd_rx channel is closed, the manager task will exit the loop and end.
    });

    // Spawn two tasks, one gets a key and the other sets a key
    let cmd_tx2 = cmd_tx.clone();
    let t1 = tokio::spawn(async move {
        // Create an oneshot channel for receiving the response from the manager task.
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            // Include the sender half of the oneshot channel in the command.
            // Such that the manager task can use it to send the response back.
            resp: resp_tx,
        };
        cmd_tx.send(cmd).await.unwrap();
        let result = resp_rx.await.unwrap().unwrap();
        println!("got value from the server; result={:?}", result);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        cmd_tx2.send(cmd).await.unwrap();
        resp_rx.await.unwrap().unwrap();
        println!("successfully set value on the server");
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}