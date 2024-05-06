use methods::METHOD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json;

use collatz_core::Input;

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

    match &cli.command {
        Commands::Next { input } => {
            let receipt = prove(*input, 0);
            let receipt_json = serde_json::to_string(&receipt).unwrap();
            std::fs::write("proof.json", receipt_json).unwrap();
        }
        Commands::Reset { input } => {
            let receipt = prove(1, *input);
            let receipt_json = serde_json::to_string(&receipt).unwrap();
            std::fs::write("proof.json", receipt_json).unwrap();
        }
    }
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
