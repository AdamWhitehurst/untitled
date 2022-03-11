host:
    cargo build --target wasm32-unknown-unknown
    wasm-bindgen --out-dir public/ --target web target/wasm32-unknown-unknown/debug/runyx.wasm
    basic-http-server
run:
    just host