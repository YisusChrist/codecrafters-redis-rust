use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    println!("Logs from your program will appear here!");

    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_incoming_connection(stream);
                });
            }
            Err(e) => {
                println!("Error accepting stream: {}", e);
                break;
            }
        }
    }
}

fn handle_incoming_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let mut buf = [0; 1024];
    loop {
        let n = match stream.read(&mut buf) {
            Ok(n) => n,
            Err(_) => {
                println!("Error reading from stream");
                break;
            }
        };
        let received = String::from_utf8_lossy(&buf[..n]);
        let response = if received.contains("ping") {
            "+PONG\r\n".to_string()
        } else if received.contains("echo") {
            let data = received.split_whitespace().collect::<Vec<&str>>();
            format!("${}\r\n{}\r\n", data[1].len(), data[1])
        } else {
            "Invalid command".to_string()
        };

        match stream.write(response.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                println!("Error writing to stream");
                break;
            }
        };
    }
}
