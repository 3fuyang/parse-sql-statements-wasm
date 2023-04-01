# Parse SQL Statements WASM

This is a `wasm` wrapper of [sqlparser](https://crates.io/crates/sqlparser), exposing the `parse_statements` method only.

## Method

```ts
interface ParseResult {
  /** Indicates whether or not the given sql is valid. */
  success: boolean
  /** You may deserialize this using `JSON.parse()`. */
  serialzed_result: string
}

export function parse_statements(text: string): ParseResult
```

## Build

[wasm-pack](https://github.com/rustwasm/wasm-pack) required.

```shell
# native in browser
wasm-pack build --target web

# esm integration
wasm-pack build --target bundler
```
