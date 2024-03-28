use std::io::{ErrorKind, Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("cannot bind to port 6379");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_incoming_connection(stream).await;
        });
    }
}

async fn handle_incoming_connection(mut stream: TcpStream) {
    println!("accepted new connection");
    let mut buf = [0; 1024];
    for _ in 0..1024 {
        match stream.read(&mut buf).await.expect("Failed to read stream") {
            0 => break,
            _ => response(&mut stream, buf).await
        }
    }
}

async fn response(stream: &mut TcpStream, buf: [u8; 1024]) {
    if true | (String::from_utf8(buf.to_vec()).expect("invalid bytes") == String::from("ping")) {
        stream
            .write_all("+PONG\r\n".as_bytes())
            .await
            .expect("cannot write to stream");
    }
}
