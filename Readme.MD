Elevator excercise for NEAR blockchain.

... To be continued


- Build (cargo build --target wasm32-unknown-unknown --release)
- Get a testnet wallet and login with cli (near login)
- Deploy (near deploy --wasmFile target/wasm32-unknown-unknown/release/near.wasm --accountId {accountId})

- Run new to instantiate Elevator  (near call {accountId} new --accountId {accountId})

- Run hop_on a number of times to add people (near call {accountId} hop_on --accountId {accountId})
- Run flush to get stops and empty (near call {accountId} flush --accountId {accountId})

Quick::

- Elevator is always the same
- Person is always the same
- Tests are missing
