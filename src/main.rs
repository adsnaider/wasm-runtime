use wasm_parse::wasm::module::Module;
use wasm_runtime::execute_module;

fn main() {
    let module = Module::from_binary(
        std::fs::read("../../wasm-examples/const.wasm").expect("Can't read wasm file."),
    );
    println!(
        "Execution of wasm returned: {:?}",
        execute_module(&module, &[])
    );
}
