## 1

```
cargo build --target wasm32-unknown-unknown  --release
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/key_value_storage.wasm
```

## 2

```
Transaction Id ABgudVPazfuVyuzMYCiuLfjFEv2i1Kq62GWmtVKzp1PT
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/ABgudVPazfuVyuzMYCiuLfjFEv2i1Kq62GWmtVKzp1PT
Done deploying to dev-1653298028614-29625999758418
```

## 3

```
near call dev-1653298028614-29625999758418 create_update '{"k": "key1", "v": "value1"}' --accountId dev-1653298028614-29625999758418
near view dev-1653298028614-29625999758418 read '{"k": "key1"}'
near view dev-1653298028614-29625999758418 read_keys
near call dev-1653298028614-29625999758418 delete '{"k": "key1"}' --accountId dev-1653298028614-29625999758418

```
