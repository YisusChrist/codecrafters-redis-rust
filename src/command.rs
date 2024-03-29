use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::SystemTime;

pub type CommandCallback =
    fn(&[&str], &Arc<Mutex<HashMap<String, (String, SystemTime)>>>) -> String;

pub fn ping_command(_: &[&str], _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>) -> String {
    "+PONG\r\n".to_string()
}

pub fn echo_command(
    parts: &[&str],
    _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
) -> String {
    format!("{}\r\n{}\r\n", parts[3], parts[4])
}

pub fn set_command(
    parts: &[&str],
    storage: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
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

pub fn get_command(
    parts: &[&str],
    storage: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
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

pub fn info_command(
    parts: &[&str],
    _: &Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
) -> String {
    // Replication section of the INFO command
    // For now, only support the role key
    if parts[4] == "replication" {
        let role = "master"; // Assuming this server is always the master
        format!("$11\r\nrole:{}\r\n", role)
    } else {
        "-ERR wrong number of arguments for 'info' command\r\n".to_string()
    }
}
