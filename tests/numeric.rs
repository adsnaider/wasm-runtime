use wasm_parse::wasm::module::Module;
use wasm_runtime::{execute_function, Val};

static NUMBERS: &[u8] = include_bytes!("wasm/numbers.wasm");

#[test]
fn add() {
    let module = Module::from_binary(NUMBERS.to_vec());
    let result = execute_function(&module, "add", vec![Val::I32(3), Val::I32(2)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(5));
}

#[test]
fn sub() {
    let module = Module::from_binary(NUMBERS.to_vec());
    let result = execute_function(&module, "sub", vec![Val::I32(3), Val::I32(2)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(1));
}

#[test]
fn mul() {
    let module = Module::from_binary(NUMBERS.to_vec());
    let result = execute_function(&module, "mul", vec![Val::I32(3), Val::I32(2)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(6));
}

#[test]
fn fadd() {
    let module = Module::from_binary(NUMBERS.to_vec());
    let result = execute_function(&module, "fadd", vec![Val::F32(3.3), Val::F32(2.1)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::F32(3.3 + 2.1));
}

#[test]
fn fsub() {
    let module = Module::from_binary(NUMBERS.to_vec());
    let result = execute_function(&module, "fsub", vec![Val::F32(3.3), Val::F32(2.1)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::F32(3.3 - 2.1));
}

#[test]
fn fmul() {
    let module = Module::from_binary(NUMBERS.to_vec());
    let result = execute_function(&module, "fmul", vec![Val::F32(3.3), Val::F32(2.1)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::F32(3.3 * 2.1));
}
