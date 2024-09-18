WASM_TARGET := "wasm32-wasip1"

build-all-binary TOOLCHAIN="stable":
    @just _build-binary "exit-with-process" {{TOOLCHAIN}}
    @just _build-binary "exit-with-exit-code" {{TOOLCHAIN}}

_build-binary BIN_NAME TOOLCHAIN:
    cargo \
        +{{TOOLCHAIN}} \
        rustc \
        --manifest-path {{BIN_NAME}}/Cargo.toml \
        --release \
        -- \
        -C panic=abort \
        -C opt-level=z \
        -C codegen-units=1

build-all-wasm TOOLCHAIN="stable":
    @just _build-wasm-binary "exit-with-process" {{TOOLCHAIN}}
    @just _build-wasm-binary "exit-with-exit-code" {{TOOLCHAIN}}

_build-wasm-binary BIN_NAME TOOLCHAIN:
    cargo \
        +{{TOOLCHAIN}} \
        rustc \
        --manifest-path {{BIN_NAME}}/Cargo.toml \
        --release \
        --target {{WASM_TARGET}} \
        -- \
        -C panic=abort \
        -C opt-level=z \
        -C codegen-units=1

# Cargo test doesn't require being run under a different toolchain, as the test
# reads the wasm output from the build stage.
test TOOLCHAIN="stable": (build-all-wasm TOOLCHAIN)
    cargo test
