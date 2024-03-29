use clap::ArgMatches;

pub enum ServerRole {
    Master,
    Replica {
        master_host: String,
        master_port: u16,
    },
}

pub fn get_role(matches: ArgMatches) -> ServerRole {
    if let Some(replicaof) = matches.values_of("replicaof") {
        let values: Vec<&str> = replicaof.collect();
        if let [master_host, master_port] = values.as_slice() {
            let master_port: u16 = master_port.parse().expect("Invalid master port number");

            return ServerRole::Replica {
                master_host: master_host.to_string(),
                master_port,
            };
        }
    }
    ServerRole::Master
}
