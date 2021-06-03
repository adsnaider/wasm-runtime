use wasm_parse::wasm::module::Module;
use wasm_runtime::execute;

fn main() {
    let module = Module::from_binary(
        std::fs::read("../../wasm-examples/const.wasm").expect("Can't read wasm file."),
    );
    println!("Execution of wasm returned: {:?}", execute(&module, &[]));
}
