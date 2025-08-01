// src/main.rs
/*
 * Main executable for MetaNexus
 */

use clap::Parser;
use metanexus::{Result, run};

#[derive(Parser)]
#[command(version, about = "MetaNexus - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
