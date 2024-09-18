/// This example main uses [`std::process::exit`], which works as expected when compiled against the
/// `wasm32-wasip1` target.
fn main() {
    println!("Exiting with code 10");
    std::process::exit(10);
}
