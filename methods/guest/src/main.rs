#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use hyle_contract::{HyleInput, HyleOutput};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: HyleInput<u32> = env::read();

    env::commit(&HyleOutput {
        block_number: input.block_number,
        block_time: input.block_time,
        caller: input.caller,
        tx_hash: input.tx_hash,
        program_outputs: Some("Any output heehee"),
        initial_state: u32::to_be_bytes(input.initial_state).to_vec(),
        next_state: u32::to_be_bytes(
            if input.initial_state == 1 {
                match input.program_inputs.unwrap() {
                    0 => panic!("Cannot reset to 0 as that would block the contract."),
                    a => a
                }
            } else {
                // Calculate the next number in the collatz conjecture
                let mut n = input.initial_state;
                if n % 2 == 0 {
                    n = n / 2;
                } else {
                    n = 3 * n + 1;
                }
                n
            }
        ).to_vec()
    })
}
