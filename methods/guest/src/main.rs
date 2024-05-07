#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

use collatz_core::{Input, HyleOutput};

pub fn main() {
    let input: Input = env::read();

    env::commit(&HyleOutput {
        initial_state: u32::to_be_bytes(input.initial_state).to_vec(),
        next_state: u32::to_be_bytes(
            if input.initial_state == 1 {
                match input.suggested_number {
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
