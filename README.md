# Hylé example RISC Zero smart contract

This repository provides an example smart contract for Hylé, implementing the Collatz Conjecture.
The Image ID of this smart contract is `0x85dd8705789d7e8eba2f2698bfafae3dbe8d124a059f08b8032e4a1b182dbf57`

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
# The verifier currently expects no `0x` prefix
cargo run 85dd8705789d7e8eba2f2698bfafae3dbe8d124a059f08b8032e4a1b182dbf57 [path_to_proof] [initial_state] [final_state]
```
If the proof is malformed, or doesn't respect the rules of the smart contract, the verifier will return an error.

## Verifying on Hylé

TODO
