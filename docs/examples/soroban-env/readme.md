# soroban-env

The example in this directory demonstrate how to embed the soroban-env-host,
then simulate the invoke of a contract.

> [!NOTE]
> The soroban-env host requires some significant setup. This example represents
> a very basic starting point. Reach out in the
> [Stellar Developers Discord](https://discord.gg/stellardev) with questions
> specific to your use case.

## Prerequisites

- [Rust](https://rust-lang.org/)

## Usage

1. Navigate to this directory.

2. Run the code that loads the `contract.wasm` file and executes it inside the
   environment.
   ```bash
   cargo run
   ```

3. View the [src/main.rs](src/main.rs) file to understand how the environment
   was embedded.
