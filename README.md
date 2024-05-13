# Hylé example RISC Zero smart contract

This repository provides an example smart contract for Hylé, implementing the Collatz Conjecture.

## Installing RISC Zero

TODO

## Reproducible builds

RISC Zero provides using a docker setup. Simply run
```bash
cargo risczero build --manifest-path methods/guest/Cargo.toml
```
to build the smart contract.

The reproducible Image ID of this smart contract is currently `0xb48e70c79688b41fc8f0daf8370d1ddb3f44ada934c10c6e0b0f5915102a363b`


## Running the smart contract

```bash
cargo run next X # Generate a proof of the transition from X to the next number in the collatz conjecture
# Or reproducibly
cargo run -- -r next X
```

```bash
cargo run reset X # Reset to X, assuming the current number is a 1
# Or reproducibly
cargo run -- -r reset X
```

### Verifying locally

Install the [Hylé RISC Zero verifier](https://github.com/Hyle-org/hyle-risc-zero-verifier).
You can then verify proofs using:
```sh
# The verifier currently expects no `0x` prefix. Pass data as base64 values.
cargo run b48e70c79688b41fc8f0daf8370d1ddb3f44ada934c10c6e0b0f5915102a363b [path_to_proof] [initial_state] [final_state]
```
If the proof is malformed, or doesn't respect the rules of the smart contract, the verifier will return an error.

## Verifying on Hylé

TODO
