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
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(n) => {
            let received = String::from_utf8_lossy(&buf[..n]);
            let mut response = String::new();
            let mut parts = received.trim().split("\r\n");

            if let Some(command) = parts.next() {
                if command.starts_with("*") {
                    // Assuming a valid multi-bulk command
                    if let Ok(count) = command[1..].parse::<usize>() {
                        let mut data_parts = parts.collect::<Vec<_>>();
                        if data_parts.len() >= count {
                            for _ in 0..count {
                                let command_type = data_parts.remove(0);
                                match command_type {
                                    "$4" => {
                                        if let Some(data) = data_parts.pop() {
                                            if command_type == "echo" {
                                                response.push_str(&format!("+{}\r\n", data));
                                            } else {
                                                response.push_str("-Unknown command\r\n");
                                            }
                                        } else {
                                            response.push_str("-Invalid data\r\n");
                                        }
                                    }
                                    _ => {
                                        response.push_str("-Unknown command\r\n");
                                    }
                                }
                            }
                        } else {
                            response.push_str("-Incomplete command\r\n");
                        }
                    } else {
                        response.push_str("-Invalid command count\r\n");
                    }
                } else {
                    response.push_str("-Invalid protocol\r\n");
                }
            } else {
                response.push_str("-Empty input\r\n");
            }

            if let Err(_) = stream.write_all(response.as_bytes()) {
                println!("Error writing to stream");
            }
        }
        Err(_) => {
            println!("Error reading from stream");
        }
    }
}
