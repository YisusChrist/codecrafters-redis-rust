mod command;
mod redis_server;
mod role;

use redis_server::{start_master_server, start_replica_server};
use role::*;

use clap::{App, Arg};

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
        .arg(
            Arg::with_name("replicaof")
                .long("replicaof")
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["host", "port"])
                .help("Sets the master server's host and port"),
        )
        .get_matches();

    let port_str = matches.value_of("port").unwrap_or("6379");
    let port: u16 = port_str.parse().expect("Invalid port number");
    let role = get_role(matches);

    // Start listening for incoming connections and handle them based on the server role
    match role {
        ServerRole::Master => start_master_server(port),
        ServerRole::Replica {
            master_host,
            master_port,
        } => start_replica_server(port, master_host, master_port),
    };
}
