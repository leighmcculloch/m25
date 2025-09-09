# Extracting Contract Specs - JavaScript/TypeScript

This example demonstrates how to extract contract specifications (specs) from a
Stellar contract using JavaScript.

The example uses the contract wasm located at `docs/contract/contract.wasm`.

## Prerequisites

- [Deno](https://deno.com/)

## Usage

1. Navigate to this directory.

2. Run the code that loads the contract.wasm file and extracts its spec,
   printing out functions in the spec:
   ```bash
   deno task run
   ```

3. View the [main.js](main.js) file to understand how the spec is extracted.
