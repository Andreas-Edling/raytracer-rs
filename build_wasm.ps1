#prereqs: "cargo install -f wasm-bindgen-cli"

cargo build --release --lib --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir ./www target/wasm32-unknown-unknown/release/raytracer_wasm.wasm