# `wasm32-wasip1` Exit Code Reproducer

*FIXED* This has been fixed in
[130554](https://github.com/rust-lang/rust/pull/130554) and is scheduled to be
included in the Rust 1.83 release (it is currently available in the Rust
1.83-nightly build).

This repository contains tests and code to reproduce an issue with WASM/WASI
(preview 1) artifacts producing unexpected exit code values when using the Rust
Standary Library's
[`std::process::ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html)
vs [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html).

There are two simple examples, `exit-with-process`, which uses
`std::process::exit`, and `exit-with-exit-code`, which uses
`std::process::ExitCode`.  Each example may be executed on its own, compiled for
the host platform.

When compiled as a standard binary on the host system, the exit codes are
returned as expected.  When compiled with `wasm32-wasip1`, the use of `ExitCode`
results in the exit code of `1`, regardless of the value provided to
`ExitCode::from`.

Note that the tool chain may be passed as an additional parameter to the `just`
commands listed below.

## Building

Build the two applications using either

```
$ just build-all-binary
```

or directly with

```
$ cargo rustc \
    --manifest-path $BIN_NAME/Cargo.toml \
    --release \
    -- \
    -C panic=abort \
    -C opt-level=z \
    -C codegen-units=1
```

where `$BIN_NAME` is one of `[exit-with-process, exit-with-exit-code]`.

These `rustc` configuration options match what is being used when compiling the
code to `wasm32-wasi`.

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
        -C panic=abort \
        -C opt-level=z \
        -C codegen-units=1
```

where `$BIN_NAME` is one of `[exit-with-process, exit-with-exit-code]`.

The tests can be run using the standard `cargo test`.

## Generated WASM

Looking at the WASM and decompiling it to WAT, we can see that in the ExitCode
case, the `main` (marked as `no_mangle`) function is returning a `i32.const 1`:

```wat
(func $main (type 0) (result i32)
  (local i32)
  global.get $__stack_pointer
  ;; ...
  global.set $__stack_pointer
  i32.const 1)
```


## System Notes

*rustc versions*: `rustc 1.80.1 (3f5fd8dd4 2024-08-06)`, `rustc 1.81.0
(eeb90cda1 2024-09-04)`, `rustc 1.82.0-beta.3 (4976ae480 2024-09-09)`, and
`rustc 1.83.0-nightly (28e8f01c2 2024-09-17)`
*host OS*:
```
$ lsb_release -a
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 22.04.4 LTS
Release:        22.04
Codename:       jammy
```
