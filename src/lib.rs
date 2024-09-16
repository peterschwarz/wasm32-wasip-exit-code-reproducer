#[cfg(test)]
mod tests {
    use std::fs;

    use wasmtime::{Engine, Linker, Module, Store};
    use wasmtime_wasi::WasiCtxBuilder;

    type BoxError = Box<dyn std::error::Error>;

    /// This test is a sanity test of using the `proc_exit` feature provided by the WASI preview 1
    /// host function.
    ///
    /// It uses a non-zero exit code and expects to receive it via the [`wasmtime_wasi::I32Exit`]
    /// error trap.
    #[test]
    fn test_simple_wasm_return() -> Result<(), BoxError> {
        let simple_wasm = r#"
            (module
                (import "wasi_snapshot_preview1" "fd_write"
                    (func $fd_write (param i32 i32 i32 i32) (result i32)))
                (import "wasi_snapshot_preview1" "proc_exit"
                    (func $__imported_wasi_snapshot_preview1_proc_exit (param i32)))

                (memory 1)
                (export "memory" (memory 0))

                (data (i32.const 8) "hello world\n")

                (func $main (export "_start")
                    (i32.store (i32.const 0) (i32.const 8))
                    (i32.store (i32.const 4) (i32.const 12))

                    (call $fd_write
                        (i32.const 1)
                        (i32.const 0)
                        (i32.const 1)
                        (i32.const 20)
                    )
                    (call $__imported_wasi_snapshot_preview1_proc_exit
                        (i32.const 90))
                    drop
                )
            )
            "#;
        let wasm_bytes = wat::parse_str(simple_wasm)?;

        let exit_code = execute_and_return_exit_code(&wasm_bytes)?;

        assert_eq!(90, exit_code);

        Ok(())
    }

    /// Load the compiled WASM produced via compiling the binary at `./exit-with-process` with the
    /// `wasm32-wasip1` target. This binary's main uses [`std::process::exit`] to exit the
    /// application with an exit code.
    ///
    /// This test validates that the [`wasmtime_wasi::I32Exit`] receives the correct value.
    #[test]
    fn test_exit_with_process() -> Result<(), BoxError> {
        let wasm_bytes =
            fs::read("./exit-with-process/target/wasm32-wasip1/release/exit-with-process.wasm")?;

        let exit_code = execute_and_return_exit_code(&wasm_bytes)?;
        assert_eq!(10, exit_code);

        Ok(())
    }

    /// Load the compiled WASM produced via compiling the binary at `./exit-with-process` with the
    /// `wasm32-wasip1` target. This binary's main returns an [`std::process::ExitCode`] instance
    /// to exit the application with an exit code.
    ///
    /// This test validates that the [`wasmtime_wasi::I32Exit`] receives the correct value.
    #[test]
    fn test_exit_with_exit_code() -> Result<(), BoxError> {
        let wasm_bytes = fs::read(
            "./exit-with-exit-code/target/wasm32-wasip1/release/exit-with-exit-code.wasm",
        )?;

        let exit_code = execute_and_return_exit_code(&wasm_bytes)?;
        assert_eq!(11, exit_code);

        Ok(())
    }

    fn execute_and_return_exit_code(wasm_bytes: &[u8]) -> Result<i32, Box<dyn std::error::Error>> {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm_bytes)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |t| t)?;
        let pre = linker.instantiate_pre(&module)?;

        let wasi = WasiCtxBuilder::new().inherit_stdio().build_p1();

        let mut store = Store::new(&engine, wasi);
        let instance = pre.instantiate(&mut store)?;

        let func = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        let result = func.call(&mut store, ());

        let exit_code: i32 = if let Err(err) = result {
            match err.downcast_ref::<wasmtime_wasi::I32Exit>() {
                Some(wasmtime_wasi::I32Exit(code)) => *code,
                None => {
                    panic!("An unexpected WASM error occurred: {}", err);
                }
            }
        } else {
            0
        };

        Ok(exit_code)
    }
}
