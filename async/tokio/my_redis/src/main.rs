use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use my_redis::ShardedDatabase;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    // Create a sharded database with 16 shards
    let db = ShardedDatabase::new(16);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Why do we need to clone here?
        // Because the db is wrapped in an Arc internally, cloning it only increments
        // the reference count, allowing multiple tasks to share the same database.
        let db = db.clone();
        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            process(socket, db).await
        });
    }
}

async fn process(socket: TcpStream, db: ShardedDatabase) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
