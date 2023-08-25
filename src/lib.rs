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

        // Extend with connections from files
        if let Some(file_path) = args.file_path {
            for file in file_path {
                let contents = fs::read_to_string(file)?;
                extend_connections_from_contents(&mut connections, contents);
            }
        }

        Ok(Config {
            connections,
            timeout: args.timeout,
        })
    }
}

fn extend_connections_from_contents(connections: &mut Vec<String>, contents: String) {
    connections.extend(
        contents
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string()),
    );
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for connection in config.connections {
        if connection.starts_with('#') {
            let msg = connection;
            println!("{}", msg.blue())
        } else {
            test_connection(connection, config.timeout)?;
        }
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

#[cfg(test)]
mod extend_connections_from_file_contents {
    use super::*;

    #[test]
    fn single_line() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("single");
        extend_connections_from_contents(&mut connections, contents);
        assert_eq!(connections, vec!["init", "single"]);
    }

    #[test]
    fn two_lines() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("first\nsecond");
        extend_connections_from_contents(&mut connections, contents);
        assert_eq!(connections, vec!["init", "first", "second"]);
    }

    #[test]
    fn empty_line() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("");
        extend_connections_from_contents(&mut connections, contents);
        assert_eq!(connections, vec!["init"]);
    }

    #[test]
    fn empty_line_in_between() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("first\n\nsecond");
        extend_connections_from_contents(&mut connections, contents);
        assert_eq!(connections, vec!["init", "first", "second"]);
    }

    #[test]
    fn comment() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("# comment");
        extend_connections_from_contents(&mut connections, contents);
        assert_eq!(connections, vec!["init", "# comment"]);
    }

    #[test]
    fn comment_in_between() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("first\n# comment\nsecond");
        extend_connections_from_contents(&mut connections, contents);
        assert_eq!(connections, vec!["init", "first", "# comment", "second"]);
    }
}
