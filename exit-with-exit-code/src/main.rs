/// This example main uses [`std::process::ExitCode`], which does not work as expected when
/// compiled using the `wasm32-wasip1` target.
use std::process::ExitCode;

#[no_mangle]
fn main() -> ExitCode {
    println!("Exiting with code 11`");
    ExitCode::from(11)
}
