WASM_TARGET := "wasm32-wasip1"

build-all:
    @just _build-wasm-binary "exit-with-process"
    @just _build-wasm-binary "exit-with-exit-code"

_build-wasm-binary BIN_NAME:
    cargo rustc \
        --manifest-path {{BIN_NAME}}/Cargo.toml \
        --release \
        --target {{WASM_TARGET}} \
        -- \
        # -C lto=y \
        -C panic=abort \
        -C opt-level=z \
        -C codegen-units=1

test: build-all
    cargo test
