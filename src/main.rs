use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Logs from your program will appear here!");

    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).unwrap();
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let storage_clone = Arc::clone(&storage); // Clone the Arc for each thread
                thread::spawn(move || {
                    handle_incoming_connection(stream, storage_clone);
                });
            }
            Err(e) => {
                println!("Error accepting stream: {}", e);
                break;
            }
        }
    }
}

fn handle_incoming_connection(
    mut stream: TcpStream,
    storage: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
) {
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
        } else if parts[2] == "set" {
            // Check if we have enough parts for SET command
            if parts.len() >= 6 {
                // Assuming no extra options for simplicity
                let key = parts[4].to_string();
                let value = parts[6].to_string();
                let mut expiry: Option<u64> = None;

                // Parsing expiry time if present
                for i in 7..parts.len() {
                    if parts[i].to_lowercase() == "px" {
                        if let Ok(exp) = parts[i + 1].parse::<u64>() {
                            expiry = Some(exp);
                            break;
                        } else {
                            stream
                                .write(b"-ERR invalid expiry time\r\n")
                                .expect("Error writing to stream");
                        }
                    }
                }

                let mut storage = storage.lock().unwrap(); // Lock the Mutex before accessing the HashMap
                if let Some(expiry) = expiry {
                    let now = SystemTime::now();
                    storage.insert(
                        key.clone(),
                        (value.clone(), now + Duration::from_millis(expiry)),
                    );
                    "+OK\r\n".to_string()
                } else {
                    storage.insert(key.clone(), (value.clone(), SystemTime::now()));
                    "+OK\r\n".to_string()
                }
            } else {
                "-ERR wrong number of arguments for 'set' command\r\n".to_string()
            }
        } else if parts[2] == "get" {
            // Check if we have enough parts for GET command
            if parts.len() >= 4 {
                let key = parts[4];
                let storage = storage.lock().unwrap(); // Lock the Mutex before accessing the HashMap
                match storage.get(key) {
                    Some((value, expiry)) => {
                        if expiry.elapsed().unwrap_or(Duration::from_secs(0))
                            > Duration::from_secs(0)
                        {
                            format!("${}\r\n{}\r\n", value.len(), value)
                        } else {
                            "$-1\r\n".to_string() // Key has expired
                        }
                    }
                    None => "$-1\r\n".to_string(), // Key does not exist
                }
            } else {
                "-ERR wrong number of arguments for 'get' command\r\n".to_string()
            }
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
