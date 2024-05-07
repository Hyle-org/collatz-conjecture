# Hylé example RISC Zero smart contract

This repository provides an example smart contract for Hylé, implementing the Collatz Conjecture.
The Image ID of this smart contract is `0x81895070f3026a5b56341be45cf6497329851ed6a847f223d8776f225ad2489c`

## Installing RISC Zero

TODO

## Running the smart contract

```bash
cargo run next X # Generate a proof of the transition from X to the next number in the collatz conjecture
```

```bash
cargo run reset X # Reset to X, assuming the current number is a 1
```

### Verifying locally

Install the [Hylé RISC Zero verifier](https://github.com/Hyle-org/hyle-risc-zero-verifier).
You can then verify proofs using:
```sh
# The verifier currently expects no `0x` prefix. Pass data as base64 values.
cargo run 81895070f3026a5b56341be45cf6497329851ed6a847f223d8776f225ad2489c [path_to_proof] [initial_state] [final_state]
```
If the proof is malformed, or doesn't respect the rules of the smart contract, the verifier will return an error.

## Verifying on Hylé

TODO
