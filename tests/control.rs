use wasm_parse::wasm::module::Module;
use wasm_runtime::{execute_function, Val};

static CONTROL: &[u8] = include_bytes!("wasm/control.wasm");

#[test]
fn min() {
    let module = Module::from_binary(CONTROL.to_vec());
    let result = execute_function(&module, "min", vec![Val::I32(7), Val::I32(23)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(7));
    let result = execute_function(&module, "min", vec![Val::I32(25), Val::I32(23)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(23));
}

#[test]
fn fibr() {
    let module = Module::from_binary(CONTROL.to_vec());
    let result = execute_function(&module, "fibr", vec![Val::I32(0)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(0));

    let result = execute_function(&module, "fibr", vec![Val::I32(1)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(1));

    let result = execute_function(&module, "fibr", vec![Val::I32(5)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(5));
}

#[test]
fn fibi() {
    let module = Module::from_binary(CONTROL.to_vec());
    let result = execute_function(&module, "fibi", vec![Val::I32(0)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(0));

    /*
    let result = execute_function(&module, "fibi", vec![Val::I32(1)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(1));

    let result = execute_function(&module, "fibi", vec![Val::I32(5)]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Val::I32(5));
    */
}
