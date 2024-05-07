use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json;

use base64::prelude::*;

use collatz_core::{HyleOutput, Input};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Next { input: u32 },
    Reset { input: u32 },
}

fn main() {
    let cli = Cli::parse();

    println!("Running with method ID: {} (hex)", METHOD_ID.iter().map(|&id| format!("{:08x}", id)).collect::<Vec<String>>().join(""));

    let receipt = match &cli.command {
        Commands::Next { input } => prove(*input, 0),
        Commands::Reset { input } => prove(1, *input)
    };
    let receipt_json = serde_json::to_string(&receipt).unwrap();
    std::fs::write("proof.json", receipt_json).unwrap();

    let journal = receipt.journal.decode::<HyleOutput>().unwrap();
    let initial_state_b64 = BASE64_STANDARD.encode(&journal.initial_state);
    let next_state_b64 = BASE64_STANDARD.encode(&journal.next_state);
    let initial_state_u32: u32 = u32::from_be_bytes(journal.initial_state.try_into().unwrap());
    let next_state_u32: u32 = u32::from_be_bytes(journal.next_state.try_into().unwrap());

    println!("proof.json written, transition from {} ({}) to {} ({})", initial_state_b64, initial_state_u32, next_state_b64, next_state_u32);
}

fn prove(initial_state: u32, suggested_number: u32) -> risc0_zkvm::Receipt {
    let env = ExecutorEnv::builder()
        .write(&Input {
            initial_state,
            suggested_number,
        })
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    return prover.prove(env, METHOD_ELF).unwrap();
}
