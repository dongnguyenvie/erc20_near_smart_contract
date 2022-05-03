###

> Build and deploy token and run constructor

```bash
cargo build --target wasm32-unknown-unknown --release
near deploy  --wasmFile target/wasm32-unknown-unknown/release/impl_erc20.wasm --accountId dev-1650612027853-89805364150390
near call dev-1650612027853-89805364150390  new '{"name": "nolan-coin", "symbol": "NLC", "total_supply": "10000000"}' --accountId dev-1650612027853-89805364150390
```

> Check total sunpply of token

```bash
near view dev-1650612027853-89805364150390 total_supply
```

> Mint token by owner

```bash
near call dev-1650612027853-89805364150390  mint_by_owner '{"recipient": "dev-1650612027853-89805364150390", "amount": "200"}' --accountId dev-1650612027853-89805364150390
```

> Check balance of address

```bash
near view dev-1650612027853-89805364150390  balance_of '{"account": "dev-1650612027853-89805364150390"}'
```

daft:

```
cargo test -- --nocapture
cargo build --target wasm32-unknown-unknown --release
near deploy --wasmFile target/wasm32-unknown-unknown/release/impl_erc20.wasm --accountId nolannguyen.testnet
near deploy --wasm-file target/wasm32-unknown-unknown/release/impl_erc20.wasm --account-id counter.nolannguyen.testnet --master-account nolannguyen.testnet
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/impl_erc20.wasm

near view dev-1650612027853-89805364150390 balance_of dev-1650612027853-89805364150390 --accountId dev-1650612027853-89805364150390

near call dev-1650612027853-89805364150390 new '{"name": "nolan-coin", "symbol": "NLC", "total_supply": "10000000"}' --accountId dev-1650612027853-89805364150390

--initArgs '{"name": "nolan-coin", "symbol": "NLC", "total_supply": "10000000"}'


near deploy  --wasmFile target/wasm32-unknown-unknown/release/impl_erc20.wasm --accountId dev-1650612027853-89805364150390
near call dev-1650612027853-89805364150390  new '{"name": "nolan-coin", "symbol": "NLC", "total_supply": "10000000"}' --accountId dev-1650612027853-89805364150390
near call dev-1650612027853-89805364150390  mint_by_owner '{"recipient": "dev-1650612027853-89805364150390", "amount": "200"}' --accountId dev-1650612027853-89805364150390

near view dev-1650612027853-89805364150390 balance_of '{"account": "dev-1650612027853-89805364150390"}' --accountId dev-1650612027853-89805364150390
near view dev-1650612027853-89805364150390 total_supply
```
