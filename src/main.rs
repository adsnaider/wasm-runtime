use wasm_parse::wasm::module::Module;
use wasm_runtime::execute_function;
use wasm_runtime::Val;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).take(2).collect();
    println!("Func to execute: {}", args[1]);
    let module = Module::from_binary(std::fs::read(&args[0]).expect("Can't read wasm file."));
    println!(
        "Execution of wasm returned: {:?}",
        execute_function(&module, &args[1], vec![Val::I32(15)])
    );
}
