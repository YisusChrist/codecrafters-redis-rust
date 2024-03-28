use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_incoming_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_incoming_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed by the client
                    println!("Connection closed by client");
                    break;
                }

                let command = String::from_utf8_lossy(&buf[..bytes_read]);
                println!("Received command: {}", command.trim());

                if command.trim() == "ping" {
                    stream
                        .write_all("+PONG\r\n".as_bytes())
                        .expect("Failed to write response");
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
