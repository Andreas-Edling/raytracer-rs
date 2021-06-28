# raytracer-rs

A raytracer written in Rust, for fun. It uses collada files as input.  
Supports native and WASM/web targets.

## Build/Run Native

```shell
> cargo run --release
```

specify input data, eg: 
```shell
cargo run --release -- -f ./data/ico2.dae
```

## Build for WASM 

```shell
> ./build_wasm.ps1
> ./run_wasm.ps1
```
