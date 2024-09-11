#![no_main]
#![no_std]

use hyle_contract::{HyleInput, HyleOutput};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: HyleInput<u32> = env::read();

    let initial_state = u32::from_be_bytes(input.initial_state.clone().try_into().unwrap());

    let next_state = if initial_state == 1 {
        match input.program_inputs {
            0 => None, // Cannot reset to 0 as that would block the contract,
            a => Some(a),
        }
    } else {
        // Calculate the next number in the collatz conjecture
        let mut n = initial_state;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        Some(n)
    };
    env::commit(&HyleOutput {
        version: 1,
        initial_state: input.initial_state,
        next_state: next_state.unwrap_or(0).to_be_bytes().to_vec(),
        identity: input.identity,
        tx_hash: input.tx_hash,
        index: 0,
        payloads: input.program_inputs.to_be_bytes().to_vec(),
        success: next_state.is_some(),
        program_outputs: "Any output heehee",
    })
}
