mod command;
use command::*;

use clap::{App, Arg};

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

fn main() {
    println!("Logs from your program will appear here!");

    let matches = App::new("Redis Server")
        .arg(
            Arg::with_name("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .default_value("6379")
                .help("Sets the port number"),
        )
        .get_matches();

    let port_str = matches.value_of("port").unwrap_or("6379");
    let port: u16 = port_str.parse().expect("Invalid port number");

    let addr = format!("127.0.0.1:{}", port);
    println!("Starting Redis server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc

    let mut commands: HashMap<&str, CommandCallback> = HashMap::new();
    commands.insert("ping", ping_command);
    commands.insert("echo", echo_command);
    commands.insert("set", set_command);
    commands.insert("get", get_command);
    commands.insert("INFO", info_command);

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
