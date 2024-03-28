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
        let parts = received.split("\r\n").collect::<Vec<&str>>();
        if parts.len() == 0 || !(parts[0].starts_with("*")) {
            continue;
        }

        let response = if parts[2] == "ping" {
            "+PONG\r\n".to_string()
        } else if parts[2] == "echo" {
            format!("{}\r\n{}\r\n", parts[3], parts[4])
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
