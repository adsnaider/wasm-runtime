use wasm_parse::wasm::module::Module;
use wasm_runtime::execute_module;

fn main() {
    let filename: String = std::env::args()
        .skip(1)
        .next()
        .expect("Missing wasm filename to execute.");
    let module = Module::from_binary(std::fs::read(filename).expect("Can't read wasm file."));
    println!(
        "Execution of wasm returned: {:?}",
        execute_module(&module, Vec::new())
    );
}
