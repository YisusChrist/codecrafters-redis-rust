use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

type CommandCallback = fn(&[&str], &Arc<Mutex<HashMap<String, (String, SystemTime)>>>) -> String;

fn main() {
    println!("Logs from your program will appear here!");

    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).unwrap();
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc

    let mut commands: HashMap<&str, CommandCallback> = HashMap::new();
    commands.insert("ping", ping_command);
    commands.insert("echo", echo_command);
    commands.insert("set", set_command);
    commands.insert("get", get_command);

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

fn ping_command(_: &[&str], _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>) -> String {
    "+PONG\r\n".to_string()
}

fn echo_command(parts: &[&str], _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>) -> String {
    format!("{}\r\n{}\r\n", parts[3], parts[4])
}

fn set_command(
    parts: &[&str],
    storage: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
) -> String {
    // Check if we have enough parts for SET command
    if parts.len() >= 6 {
        // Assuming no extra options for simplicity
        let key = parts[4].to_string();
        let value = parts[6].to_string();
        let mut expiry: Option<u64> = None;

        print!("{:?}\n", parts);

        // Parsing expiry time if present
        for i in 7..parts.len() {
            if parts[i].to_lowercase() == "px" {
                if let Ok(exp) = parts[i + 2].parse::<u64>() {
                    expiry = Some(exp);
                    break;
                } else {
                    break;
                }
            }
        }

        print!("Expiry: {:?}\n", expiry);

        let mut storage = storage.lock().unwrap(); // Lock the Mutex before accessing the HashMap
        let expire_time: SystemTime;
        if let Some(expiry) = expiry {
            println!("Set expiry: {}", expiry);
            expire_time = SystemTime::now() + Duration::from_millis(expiry);
        } else {
            println!("Default expiry: 3600");
            expire_time = SystemTime::now() + Duration::from_secs(3600); // Default expiry time of 1 hour
        }

        storage.insert(key.clone(), (value.clone(), expire_time));
        "+OK\r\n".to_string()
    } else {
        "-ERR wrong number of arguments for 'set' command\r\n".to_string()
    }
}

fn get_command(
    parts: &[&str],
    storage: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
) -> String {
    // Check if we have enough parts for GET command
    if parts.len() >= 4 {
        let key = parts[4];
        let storage = storage.lock().unwrap(); // Lock the Mutex before accessing the HashMap
        if let Some((value, expiry)) = storage.get(key) {
            let now = SystemTime::now();
            println!("Expiry: {:?}, Current Time: {:?}", expiry, now);

            if now < *expiry {
                // Key has not expired
                format!("${}\r\n{}\r\n", value.len(), value)
            } else {
                "$-1\r\n".to_string() // Key has expired
            }
        } else {
            "$-1\r\n".to_string() // Key does not exist
        }
    } else {
        "-ERR wrong number of arguments for 'get' command\r\n".to_string()
    }
}
