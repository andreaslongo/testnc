use std::error::Error;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

use anstream::println;
use clap::Parser;
use clap::ValueEnum;
use owo_colors::OwoColorize as _;

/// A simple program to test TCP network connectivity
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(
    help_template = "{about-section}\n{usage-heading} {usage}\n\n{all-args}\n\nWritten by {author}\nhttps://github.com/andreaslongo/testnc"
)]
pub struct Args {
    /// DNS name or IP address
    host: String,

    /// TCP port number
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,

    /// Protocol
    #[arg(short, long, conflicts_with("port"))]
    protocol: Option<Protocol>,

    /// Timeout limit in seconds for each connection
    #[arg(short, long, default_value_t = 1)]
    timeout: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Protocol {
    Dns,
    Http,
    Https,
    Mssql,
    Smb,
}

pub struct Config {
    host: String,
    port: u16,
    timeout_in_seconds: Duration,
}

impl Config {
    pub fn build(args: &Args) -> Result<Config, &'static str> {
        let host = args.host.clone();
        let port: u16 = match args.protocol {
            Some(Protocol::Dns) => 53,
            Some(Protocol::Http) => 80,
            Some(Protocol::Https) => 443,
            Some(Protocol::Mssql) => 1433,
            Some(Protocol::Smb) => 445,
            None => args.port.unwrap_or(443),
        };
        let timeout_in_seconds = Duration::new(args.timeout.into(), 0);
        Ok(Config {
            host,
            port,
            timeout_in_seconds,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let connection = format!("{}:{}", config.host, config.port);

    let addrs = connection.to_socket_addrs()?;

    for addr in addrs {
        match TcpStream::connect_timeout(&addr, config.timeout_in_seconds) {
            Ok(stream) => {
                let local = stream.local_addr()?.ip();
                let msg = format!("OK :: {local} :: {connection} :: {addr}");
                println!("{}", msg.green())
            }
            Err(_) => {
                let msg = format!("BAD :: {connection} :: {addr}");
                println!("{}", msg.red())
            }
        }
    }
    Ok(())
}
