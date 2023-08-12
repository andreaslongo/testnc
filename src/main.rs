use std::process;

use clap::Parser;

use testnc::Args;
use testnc::Config;

fn main() {
    let args = Args::parse();

    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        process::exit(1);
    });

    if let Err(e) = testnc::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
