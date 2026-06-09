use anstream::println;
use anyhow::{Context, Result};
use owo_colors::OwoColorize as _;
use std::{
    fs,
    net::{TcpStream, ToSocketAddrs},
    path::PathBuf,
    time::Duration,
};

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
    pub fn build(args: Args) -> Result<Config> {
        // Collect connections from cli args
        let mut connections = args.connections;

        // Extend with connections from files
        if let Some(file_path) = args.file_path {
            for file in file_path {
                let contents = fs::read_to_string(&file)
                    .with_context(|| format!("Failed to read file: {}", file.display()))?;
                extend_connections_from_contents(&mut connections, &contents);
            }
        }

        Ok(Config {
            connections,
            timeout: args.timeout,
        })
    }
}

fn extend_connections_from_contents(connections: &mut Vec<String>, contents: &str) {
    connections.extend(
        contents
            .lines()
            .filter(|line| !line.is_empty())
            .map(ToString::to_string),
    );
}

pub fn run(config: Config) -> Result<()> {
    for connection in config.connections {
        if connection.starts_with('#') {
            println!("{}", connection.blue());
        } else {
            test_connection(&connection, config.timeout);
        }
    }
    Ok(())
}

enum TestResult {
    Ok {
        local: String,
        connection: String,
        addr: String,
    },
    Err {
        connection: String,
        error: String,
    },
}
impl TestResult {
    pub fn display(&self) {
        match self {
            TestResult::Ok {
                local,
                connection,
                addr,
            } => {
                println!(
                    "{} {} {} {} {} {} {} {}",
                    " OK".green(),
                    ":: (src)".yellow(),
                    local.green(),
                    ">>".yellow(),
                    "(dst)".yellow(),
                    connection.green(),
                    "==".yellow(),
                    addr.green()
                );
            }
            TestResult::Err { connection, error } => {
                println!(
                    "{} {} {} {} {}",
                    "ERR".red(),
                    ":: (dst)".yellow(),
                    connection.red(),
                    "==".yellow(),
                    error.red()
                );
            }
        }
    }
}

fn test_connection(connection: &str, timeout: u64) {
    // Resolve network address
    // Performs network address resolution
    // Yields 0 or more addresses
    let addrs = match connection.to_socket_addrs() {
        Ok(v) => v,
        Err(e) => {
            TestResult::Err {
                connection: connection.to_string(),
                error: e.to_string(),
            }
            .display();
            return;
        }
    };

    for addr in addrs {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(timeout)) {
            Ok(stream) => {
                let local = stream.local_addr().unwrap().ip();
                TestResult::Ok {
                    local: local.to_string(),
                    connection: connection.to_string(),
                    addr: addr.to_string(),
                }
                .display();
            }
            Err(_) => {
                TestResult::Err {
                    connection: connection.to_string(),
                    error: addr.to_string(),
                }
                .display();
            }
        }
    }
}

#[cfg(test)]
mod extend_connections_from_file_contents {
    use super::*;

    #[test]
    fn single_line() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("single");
        extend_connections_from_contents(&mut connections, &contents);
        assert_eq!(connections, vec!["init", "single"]);
    }

    #[test]
    fn two_lines() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("first\nsecond");
        extend_connections_from_contents(&mut connections, &contents);
        assert_eq!(connections, vec!["init", "first", "second"]);
    }

    #[test]
    fn empty_line() {
        let mut connections = vec![String::from("init")];
        let contents = String::new();
        extend_connections_from_contents(&mut connections, &contents);
        assert_eq!(connections, vec!["init"]);
    }

    #[test]
    fn empty_line_in_between() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("first\n\nsecond");
        extend_connections_from_contents(&mut connections, &contents);
        assert_eq!(connections, vec!["init", "first", "second"]);
    }

    #[test]
    fn comment() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("# comment");
        extend_connections_from_contents(&mut connections, &contents);
        assert_eq!(connections, vec!["init", "# comment"]);
    }

    #[test]
    fn comment_in_between() {
        let mut connections = vec![String::from("init")];
        let contents = String::from("first\n# comment\nsecond");
        extend_connections_from_contents(&mut connections, &contents);
        assert_eq!(connections, vec!["init", "first", "# comment", "second"]);
    }
}
