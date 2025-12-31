use mini_redis::{client, Result};

/// This is a simple example of a client application.

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" to the value "world"
    client.set("hello", "world".into()).await?;

    // Get the value of the key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
