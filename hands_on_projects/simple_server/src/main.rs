use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{BufReader, };
use std::thread;
use std::time::Duration;
use simple_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // accept only two connections for demonstration purposes
    // also shows the graceful shutdown of the thread pool
    for tcp_stream in listener.incoming().take(2) {
        let stream = match tcp_stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
                continue;
            }
        };
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Server is shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return,  // Timeout or error, just close the connection
    };

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Hello! This is a simple server."),
        "GET /sleep HTTP/1.1" => {
            // Mimic a slow response
            thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "Hello! This is a simple server.")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404 Not Found"),
    };
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
}
