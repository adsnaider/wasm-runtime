use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::types::FuncType;

use crate::module::ModuleInstance;

pub(crate) enum FunctionInstance {
    ModuleFunction(ModuleFunction),
    HostFunction(HostFunction),
}

pub(crate) struct ModuleFunction {
    tpe: FuncType,
    module: ModuleInstance,
    code: Func,
}
pub(crate) struct HostFunction {
    tpe: FuncType,
}
