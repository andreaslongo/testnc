use std::error::Error;
use std::fs;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::time::Duration;

use anstream::println;
use owo_colors::OwoColorize as _;

pub struct Args {
    pub connections: Vec<String>,
    pub timeout: u64,
    pub file_path: Option<Vec<PathBuf>>,
}

pub struct Config {
    connections: Vec<String>,
    timeout: u64,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, Box<dyn Error>> {
        // Collect connections from cli args
        let mut connections = args.connections;

        // Extend with connections from file
        if let Some(file_path) = args.file_path {
            for file in file_path {
                let contents = fs::read_to_string(file)?;
                connections.extend(
                    contents
                        .lines()
                        .filter(|line| !line.is_empty() || !line.starts_with('#'))
                        .map(|line| line.to_string()),
                );
            }
        }

        Ok(Config {
            connections,
            timeout: args.timeout,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for connection in config.connections {
        test_connection(connection, config.timeout)?;
    }
    Ok(())
}

fn test_connection(connection: String, timeout: u64) -> Result<(), Box<dyn Error>> {
    let addrs = connection.to_socket_addrs()?;

    for addr in addrs {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(timeout)) {
            Ok(stream) => {
                let local = stream.local_addr()?.ip();
                let msg = format!(" OK :: {local} -> {} -> {addr}", connection);
                println!("{}", msg.green())
            }
            Err(_) => {
                let msg = format!("BAD :: {} -> {addr}", connection);
                println!("{}", msg.red())
            }
        }
    }
    Ok(())
}
