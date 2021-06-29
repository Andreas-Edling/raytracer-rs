![ci](https://github.com/Andreas-Edling/raytracer-rs/actions/workflows/ci.yaml/badge.svg)

# raytracer-rs

A raytracer written in Rust, for fun. It uses collada files as input.  
Supports native and WASM/web targets.  
You can see the wasm version running at https://andreas-edling.github.io/raytracer-rs/

## Build/Run Native

```shell
cargo run --release
```

specify input data, eg: 
```shell
cargo run --release -- -f ./data/ico2.dae
```

## Build/Run with WASM 

```shell
./build_wasm.ps1
./run_wasm.ps1
```
