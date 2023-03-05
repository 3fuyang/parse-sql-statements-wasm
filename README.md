# Parse SQL Statements WASM

This is a `wasm` wrapper of [sqlparser](https://crates.io/crates/sqlparser), exposing the `parse_statements` method only.

## Build

[wasm-pack](https://github.com/rustwasm/wasm-pack) required.

```shell
wasm-pack build --target bundler
```
