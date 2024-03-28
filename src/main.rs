use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let command = String::from_utf8_lossy(&buffer);
                        if command.trim() == "PING" {
                            let response = "+PONG\r\n";
                            stream.write_all(response.as_bytes()).unwrap();
                        } else {
                            println!("Received unsupported command: {}", command.trim());
                        }
                    }
                    Err(e) => {
                        println!("Error reading from stream: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
