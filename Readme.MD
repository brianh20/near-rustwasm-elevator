# Elevator excercise for NEAR blockchain.

This extends from the Elevator example in Rust. There is no frontend, execution is purely done through the CLI.
The complete excercise is in repo: `https://github.com/brianh20/rust-elevator`

# Commands
- Build `cargo build --target wasm32-unknown-unknown --release`
- Get a testnet wallet and login with cli `near login`
- Deploy `near deploy --wasmFile target/wasm32-unknown-unknown/release/near_elevator.wasm --accountId {accountId}`
- Run new to instantiate Elevator `near call {accountId} new --accountId {accountId}`

- Run `hop_on` a number of times to add people `near call {accountId} hop_on --accountId {accountId}`
- Run `flush` to get stops and empty `near call {accountId} flush --accountId {accountId}`. Watch the logs for the result

# Quick Notes
- This was done as a first approach to see how data is stored and interacting with the blockchain so...
-- Elevator is always the same
-- Person is always the same weight and floor destinatino
-- Tests are missing
