use clap::ArgMatches;

pub enum ServerRole {
    Master,
    Replica {
        master_host: String,
        master_port: u16,
    },
}

pub fn get_role(matches: ArgMatches) -> ServerRole {
    match matches.value_of("replicaof") {
        Some(master) => {
            let parts: Vec<&str> = master.split(':').collect();
            if parts.len() != 2 {
                panic!("Invalid replicaof argument. Use format: host:port");
            }
            let master_host = parts[0].to_string();
            let master_port: u16 = parts[1].parse().expect("Invalid master port number");
            ServerRole::Replica {
                master_host,
                master_port,
            }
        }
        None => ServerRole::Master,
    }
}
