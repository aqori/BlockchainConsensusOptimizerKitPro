// src/main.rs
/*
 * Main executable for BlockchainConsensusOptimizerKitPro
 */

use clap::Parser;
use blockchainconsensusoptimizerkitpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainConsensusOptimizerKitPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
