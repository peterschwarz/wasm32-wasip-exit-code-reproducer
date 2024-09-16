# `wasm32-wasip1` Exit Code Reproducer

This repository contains tests and code to reproduce an issue with WASM/WASI
(preview 1) artifacts producing unexpected exit code values when using the Rust
Standary Library's
[`std::process::ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html)
vs [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html).

There are two simple examples, `exit-with-process`, which uses
`std::process::exit`, and `exit-with-exit-code`, which uses
`std::process::ExitCode`.  Each example may be executed on its own, compiled for
the host platform.

## `exit-with-process`

This example produces the exit code of `10`.  On the host system, we see the
following.

```
$ cargo run --release --manifest-path ./exit-with-process/Cargo.toml
...
Exiting with code 10
$ echo $?
10
```

## `exit-with-exit-code`

This example produces the exit code of `11`.  On the host system, we see the
following.

```
$ cargo run --release --manifest-path ./exit-with-exit-code/Cargo.toml
...
Exiting with code 11
$ echo $?
11
```

## Running with Wasmtime Runtimee

When compiling these two examples using the `wasm32-wasip1` target, the use of
`std::process::exit` produces the same exit code as the host binary, as
expected.  However, the use of `std::process::ExitCode` results in the exit code
of 1, regardless of the value provided to `ExitCode::from`.

These expectations have been provided as a set of unit tests in `src/lib.rs`.

### With Jest

With the [just](https://github.com/casey/just) command runner, run the following

```
$ just test
cargo rustc --manifest-path exit-with-process/Cargo.toml --release --target
wasm32-wasip1 -- # -C lto=y -C panic=abort -C opt-level=z -C codegen-units=1
    Finished `release` profile [optimized] target(s) in 0.00s
cargo rustc --manifest-path exit-with-exit-code/Cargo.toml --release --target
wasm32-wasip1 -- # -C lto=y -C panic=abort -C opt-level=z -C codegen-units=1
    Finished `release` profile [optimized] target(s) in 0.00s
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs
(target/debug/deps/wasm_wasi_return_code_reproducer-aed685ebe2675c4c)

running 3 tests
hello world
test tests::test_simple_wasm_return ... ok
Exiting with code 11`
Exiting with code 10
test tests::test_exit_with_exit_code ... FAILED
test tests::test_exit_with_process ... ok

failures:

---- tests::test_exit_with_exit_code stdout ----
thread 'tests::test_exit_with_exit_code' panicked at src/lib.rs:82:9:
assertion `left == right` failed
  left: 11
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_exit_with_exit_code

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;
finished in 0.57s

error: test failed, to rerun pass `--lib`
error: Recipe `test` failed on line 19 with exit code 101
```

### Without Just

Without `just`, the following will compile the two examples to wasm with
`wasm32-wasip1`:

```
$ cargo rustc \
        --manifest-path $BIN_NAME/Cargo.toml \
        --release \
        --target wasm32-wasip1 \
        -- \
        # -C lto=y \
        -C panic=abort \
        -C opt-level=z \
        -C codegen-units=1
```

where `$BIN_NAME` is one of `[exit-with-process, exit-with-exit-code]`.

The tests can be run using the standard `cargo test`.

## System Notes

*rustc version*: `rustc 1.80.1 (3f5fd8dd4 2024-08-06)`
*host OS*:
```
$ lsb_release -a
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 22.04.4 LTS
Release:        22.04
Codename:       jammy
```
