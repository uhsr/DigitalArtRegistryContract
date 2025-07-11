// src/main.rs
/*
 * Main executable for DigitalArtRegistryContract
 */

use clap::Parser;
use digitalartregistrycontract::{Result, run};

#[derive(Parser)]
#[command(version, about = "DigitalArtRegistryContract - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
