use std::fs;

use wasm_parse::wasm::module::Module;
use wasm_runtime::runtime_manager::{Loader, Runtime};

fn main() {
    let data = fs::read("../../wasm-examples/add.wasm").unwrap();
    let module = Module::from_binary(data);
    println!("Got module {:#?}", module);
    let mut loader = Loader::new(module);
    let runtime = loader.instantiate();
}
