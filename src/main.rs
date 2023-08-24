use std::path::PathBuf;
use std::process;

use clap::Parser;

use testnc::Args;
use testnc::Config;

fn main() {
    let cli = Cli::parse();

    let args = Args {
        connections: cli.connections,
        timeout: cli.timeout,
        file_path: cli.file,
    };

    let config = Config::build(args).unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}\nTry 'testnc --help' for more information.");
        process::exit(1);
    });

    if let Err(e) = testnc::run(config) {
        eprintln!("Application error: {e}\nTry 'testnc --help' for more information.");
        process::exit(1);
    }
}

/// A simple program to test TCP network connectivity
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
#[command(
    help_template = "{about-section}\n{usage-heading} {usage}\n\n{all-args}\n\nWritten by {author}\nhttps://github.com/andreaslongo/testnc"
)]
pub struct Cli {
    /// One or more connection strings in the form 'host:port'.
    connections: Vec<String>,

    /// Timeout for each connection in seconds.
    ///
    /// '-t 0' does only address resolution without testing connections.
    #[arg(short, long, default_value_t = 1)]
    timeout: u64,

    /// File with connection strings in the form 'host:port' separated by newlines.
    #[arg(short, long)]
    file: Option<Vec<PathBuf>>,
}
