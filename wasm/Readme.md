# WebAssembly bindings

[pack]: https://rustwasm.github.io/wasm-pack/book/

Uses [wasmbindgen][bindgen].

[bindgen]: https://rustwasm.github.io/docs/wasm-bindgen/introduction.html

## Building

Building optimized wasm:

```
wasm-pack build --release --target nodejs
wasm-opt -O3 -o pkg/starkcrypto_wasm_bg.wasm pkg/starkcrypto_wasm_bg.wasm
```

## Testing

```
wasm-pack test --release --node
```

Or in a browser:

```
wasm-pack test --release --firefox
```

Test in NodeJS:

```
node ./tests/node.js
```

## Size profiling:

```
twiggy top -n 20 pkg/starkcrypto_wasm_bg.wasm
```

## Performance profiling:

```
wasm-pack build --profiling --target nodejs
node --prof ./tests/node.js
node --prof-process isolate-*-v8.log > processed.txt
```
