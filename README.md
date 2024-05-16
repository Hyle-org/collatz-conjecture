# Hylé example RISC Zero smart contract

This repository provides an example smart contract for Hylé, implementing the Collatz Conjecture.

## Installing RISC Zero

Please refer to [RiscZero's installation guide](https://dev.risczero.com/api/zkvm/install)

## Reproducible builds

RISC Zero provides using a docker setup. Simply run to build the smart contract ; 
```bash
cargo risczero build --manifest-path methods/guest/Cargo.toml
```


The reproducible Image ID of this smart contract is currently `0x14ecbb1bc768586be869bb32be7a337fc3c0c67d51ebfe9a4baf6570446b9e12`


## Running the smart contract

```bash
cargo run next X # Generate a proof of the transition from X to the next number in the collatz conjecture

cargo run -- -r next X # For reproducible builds
```

```bash
cargo run reset X # Reset to X, assuming the current number is a 1

cargo run -- -r reset X # For reproducible builds:
```

### Verifying locally

Install the [Hylé RISC Zero verifier](https://github.com/Hyle-org/hyle-risc-zero-verifier).
You can then verify proofs using:
```sh
# The verifier currently expects no `0x` prefix. Pass data as base64 values.

cargo run -p risc0-verifier 14ecbb1bc768586be869bb32be7a337fc3c0c67d51ebfe9a4baf6570446b9e12 [path_to_proof] [initial_state] [final_state]
```
If the proof is malformed, or doesn't respect the rules of the smart contract, the verifier will return an error.

## Verifying on Hylé

Once you [installed the CLI](https://docs.hyle.eu/getting-started/hyled-install-instructions/) and got [connected to devnet](https://docs.hyle.eu/getting-started/connect-to-devnet/), you should be able to [_register_ and _execute_ for your contract](https://docs.hyle.eu/getting-started/your-first-smart-contract/).
