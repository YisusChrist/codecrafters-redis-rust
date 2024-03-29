use crate::command::{get_commands, CommandCallback};

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub fn start_master_server(port: u16) {
    // Initialize storage, commands, etc.
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc
    let commands = get_commands();

    // Start TCP listener
    let addr = format!("127.0.0.1:{}", port);
    println!("Starting master Redis server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    // Accept and handle incoming connections
    accept_connections(listener, storage, commands);
}

pub fn start_replica_server(port: u16, master_host: String, master_port: u16) {
    // Initialize storage, commands, etc.
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc
    let commands = get_commands();

    // Connect to master server
    let master_addr = format!("{}:{}", master_host, master_port);
    match TcpStream::connect(master_addr) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to master server: {}", e);
            return;
        }
    };

    // Send REPLCONF commands to the master
    // (Optional: implement Redis replication protocol)

    // Start TCP listener
    let addr = format!("127.0.0.1:{}", port);
    println!("Starting replica Redis server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    // Accept and handle incoming connections
    accept_connections(listener, storage, commands);
}

fn accept_connections(
    listener: TcpListener,
    storage: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    commands: HashMap<&'static str, CommandCallback>,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let storage_clone = Arc::clone(&storage); // Clone the Arc for each thread
                let commands = commands.clone();
                thread::spawn(move || {
                    handle_incoming_connection(stream, commands, storage_clone);
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
    commands: HashMap<&str, CommandCallback>,
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

        print!("Received: {:?}", parts);

        if let Some(callback) = commands.get(parts[2]) {
            let response = callback(&parts, &storage);
            if let Err(_) = stream.write(response.as_bytes()) {
                println!("Error writing to stream");
                break;
            }
        } else {
            let response = "Invalid command".to_string();
            if let Err(_) = stream.write(response.as_bytes()) {
                println!("Error writing to stream");
                break;
            }
        }
    }
}
