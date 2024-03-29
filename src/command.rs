use crate::role::ServerRole;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::SystemTime;

pub type CommandCallback =
    fn(&[&str], &Arc<Mutex<HashMap<String, (String, SystemTime)>>>, &ServerRole) -> String;

fn ping_command(
    _: &[&str],
    _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    _: &ServerRole,
) -> String {
    "+PONG\r\n".to_string()
}

fn echo_command(
    parts: &[&str],
    _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    _: &ServerRole,
) -> String {
    format!("{}\r\n{}\r\n", parts[3], parts[4])
}

fn set_command(
    parts: &[&str],
    storage: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    _: &ServerRole,
) -> String {
    // Check if we have enough parts for SET command
    if parts.len() >= 6 {
        // Assuming no extra options for simplicity
        let key = parts[4].to_string();
        let value = parts[6].to_string();
        let mut expiry: Option<u64> = None;

        // Parsing expiry time if present
        if let Some(px_index) = parts.iter().position(|&x| x.to_lowercase() == "px") {
            if let Some(exp_str) = parts.get(px_index + 2) {
                if let Ok(exp) = exp_str.parse::<u64>() {
                    expiry = Some(exp);
                }
            }
        }

        let mut storage = storage.lock().unwrap(); // Lock the Mutex before accessing the HashMap
        let expire_time: SystemTime;
        if let Some(expiry) = expiry {
            expire_time = SystemTime::now() + Duration::from_millis(expiry);
        } else {
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
    _: &ServerRole,
) -> String {
    // Check if we have enough parts for GET command
    if parts.len() >= 4 {
        let key = parts[4];
        let storage = storage.lock().unwrap(); // Lock the Mutex before accessing the HashMap
        if let Some((value, expiry)) = storage.get(key) {
            let now = SystemTime::now();
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

fn info_command(
    parts: &[&str],
    _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    server_role: &ServerRole,
) -> String {
    // Replication section of the INFO command
    // For now, only support the role key
    if parts[4] == "replication" {
        let role = match server_role {
            ServerRole::Master => "master",
            ServerRole::Replica { .. } => "slave",
        };
        let message = ;

        // Hardcoded replication ID
        let master_replid = "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb";
        let master_repl_offset = 0;
        let total_len = message.len() + master_replid.len() + 14 + 19;

        let data = [
            format!("role:{}", role),
            format!("master_replid:{}", master_replid),
            format!("master_repl_offset:{}", master_repl_offset),
        ];

        let message = data.join("\r\n");
        let total_len = loop {
            let len = message.len() + master_replid.len() + 14 + 19;
            if len == total_len {
                break len;
            }
        };
        println!("total_len: {}", total_len);
        println!("message: {}", message);

        format!("${}\r\n{}\r\n", total_len, message)
    } else {
        "-ERR wrong number of arguments for 'info' command\r\n".to_string()
    }
}

pub fn get_commands() -> HashMap<&'static str, CommandCallback> {
    let mut commands: HashMap<&str, CommandCallback> = HashMap::new();
    commands.insert("ping", ping_command);
    commands.insert("echo", echo_command);
    commands.insert("set", set_command);
    commands.insert("get", get_command);
    commands.insert("INFO", info_command);
    commands
}
