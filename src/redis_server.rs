use crate::command::{get_commands, CommandCallback};
use crate::role::ServerRole;

use hex;

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

// Define a struct to hold replica connections
#[derive(Debug)]
struct ReplicaConnection {
    stream: TcpStream,
}

pub fn start_master_server(port: u16) {
    // Initialize storage, commands, etc.
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc
    let commands = get_commands();
    let replicas = Arc::new(Mutex::new(Vec::<ReplicaConnection>::new())); // Vector to hold connected replicas

    // Start TCP listener
    let addr = format!("127.0.0.1:{}", port);
    println!("Starting master Redis server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    let role = Arc::new(ServerRole::Master);
    // Accept and handle incoming connections
    accept_connections(listener, storage, commands, role, replicas);
}

pub fn start_replica_server(port: u16, master_host: String, master_port: u16) {
    // Initialize storage, commands, etc.
    let storage = Arc::new(Mutex::new(HashMap::new())); // Shared storage wrapped in Mutex and Arc
    let commands = get_commands();

    // Connect to master server
    let master_addr = format!("{}:{}", master_host, master_port);
    match TcpStream::connect(master_addr) {
        Ok(mut stream) => {
            // Send REPLCONF commands to master
            handshake(&mut stream);
            stream
        }
        Err(e) => {
            println!("Failed to connect to master server: {}", e);
            return;
        }
    };

    // Start TCP listener
    let addr = format!("127.0.0.1:{}", port);
    println!("Starting replica Redis server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    let role = Arc::new(ServerRole::Replica {
        master_host,
        master_port,
    });

    let replicas = Arc::new(Mutex::new(Vec::<ReplicaConnection>::new())); // Vector to hold connected replicas

    // Accept and handle incoming connections
    accept_connections(listener, storage, commands, role, replicas);
}

fn accept_connections(
    listener: TcpListener,
    storage: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    commands: HashMap<String, CommandCallback>,
    role: Arc<ServerRole>,
    replicas: Arc<Mutex<Vec<ReplicaConnection>>>,
) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let storage_clone = Arc::clone(&storage); // Clone the Arc for each thread
                let commands = commands.clone();
                let role_clone = Arc::clone(&role);
                let replicas_clone = Arc::clone(&replicas);
                thread::spawn(move || {
                    handle_incoming_connection(
                        stream,
                        commands,
                        storage_clone,
                        role_clone,
                        replicas_clone,
                    );
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
    commands: HashMap<String, CommandCallback>,
    storage: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    role: Arc<ServerRole>,
    replicas: Arc<Mutex<Vec<ReplicaConnection>>>,
) {
    println!("accepted new connection");

    let mut buf = [0; 1024];
    loop {
        let n = match stream.read(&mut buf) {
            Ok(n) => n,
            Err(_) => {
                //println!("Error reading from stream");
                //break;
                continue;
            }
        };
        let received = String::from_utf8_lossy(&buf[..n]);
        let parts = received.split("\r\n").collect::<Vec<&str>>();
        if parts.len() < 2 || !(parts[0].starts_with("*")) {
            continue;
        }

        println!("Received: {}", received);
        println!("Role is {:?}", role);

        // Convert the command to lowercase
        let command = parts[2].to_lowercase();
        if let Some(callback) = commands.get(&command) {
            let response = callback(&parts, &storage, &role);
            // Check if the current server is a replica and the command comes from the master
            /*
            if let ServerRole::Replica { .. } = *role {
                if stream.peer_addr().unwrap().port() == 6379 {
                    println!("Ignoring command from master");
                }
            } else {
            }
            */
            if let Err(_) = stream.write(response.as_bytes()) {
                println!("Error writing to stream");
                break;
            }
            if response.starts_with("+FULLRESYNC") {
                send_empty_rdb_file(&mut stream);
                // Add the newly connected replica to the list of replicas
                let mut replicas = replicas.lock().unwrap();
                replicas.push(ReplicaConnection {
                    stream: stream.try_clone().unwrap(),
                });
                println!("Replica connected");
                println!("Replicas: {:?}", replicas);
            }
            if is_write_command(&command) {
                propagate_command_to_replica(received, &replicas);
            }
        } else {
            let response = "Invalid command".to_string();
            if let Err(_) = stream.write(response.as_bytes()) {
                println!("Error writing to stream");
                break;
            }
        }
    }
    println!("Connection closed from {:?}", stream.peer_addr());
}

fn handshake(stream: &mut TcpStream) {
    // Send PING command to master server
    let ping = "*1\r\n$4\r\nping\r\n".to_string();
    if let Err(_) = stream.write(ping.as_bytes()) {
        println!("Error writing to stream");
    }
    // Await for PONG response
    read_from_stream(stream);

    // Send REPLCONF listening-port command
    let listening_port_cmd = "*3\r\n$8\r\nREPLCONF\r\n$14\r\nlistening-port\r\n$4\r\n6380\r\n";
    if let Err(_) = stream.write(listening_port_cmd.as_bytes()) {
        println!("Error writing REPLCONF listening-port command to stream");
    }
    // Await for OK response
    read_from_stream(stream);

    // Send REPLCONF capa psync2 command
    let capa_cmd = "*3\r\n$8\r\nREPLCONF\r\n$4\r\ncapa\r\n$6\r\npsync2\r\n";
    if let Err(_) = stream.write(capa_cmd.as_bytes()) {
        println!("Error writing REPLCONF capa psync2 command to stream");
    }
    // Await for OK response
    read_from_stream(stream);

    // Send PSYNC command
    let psync_cmd = "*3\r\n$5\r\nPSYNC\r\n$1\r\n?\r\n$2\r\n-1\r\n";
    if let Err(_) = stream.write(psync_cmd.as_bytes()) {
        println!("Error writing PSYNC command to stream");
    }
    // Await for FULLRESYNC response
    read_from_stream(stream);
    // Await for RDB file
    read_from_stream(stream);
}

fn read_from_stream(stream: &mut TcpStream) -> String {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    let received = String::from_utf8_lossy(&buf[..n]).to_string();
    println!("Handshake Received: {}", received);
    received
}

fn send_empty_rdb_file(stream: &mut TcpStream) {
    // Hardcoded empty RDB file in hex format
    let rdb_hex =
        "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2".to_string();
    let rdb_binary = hex::decode(rdb_hex).unwrap();
    let rdb_length = rdb_binary.len();

    // Send RDB file to the replica
    let rdb_command = format!("${}\r\n", rdb_length);
    if let Err(_) = stream.write(rdb_command.as_bytes()) {
        println!("Error writing RDB file length to stream");
        return;
    }
    if let Err(_) = stream.write(&rdb_binary) {
        println!("Error writing RDB file to stream");
    }
}

fn is_write_command(command: &str) -> bool {
    // Define write commands here
    let write_commands = vec!["set", "del"]; // Add more write commands if needed
    write_commands.contains(&command)
}

fn propagate_command_to_replica(
    received: Cow<'_, str>,
    replicas: &Arc<Mutex<Vec<ReplicaConnection>>>,
) {
    // Send command to each replica
    let replicas = replicas.lock().unwrap();
    for replica in replicas.iter() {
        let mut stream = &replica.stream;
        println!("Propagating command to replica: {:?}", stream.peer_addr());
        println!("Command: {}", received);
        match stream.write(received.as_bytes()) {
            Ok(_) => {
                println!("Command propagated successfully");
            }
            Err(_) => {
                println!("Error propagating command to replica");
            }
        }
    }
}
