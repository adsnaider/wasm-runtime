use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::FuncType;

use crate::module::ModuleInstance;
use crate::runtime_manager::{Runtime, RuntimeManager};
use crate::store::{IntoStore, StoreElement};

#[derive(Clone)]
pub(crate) struct FunctionInstance {
    tpe: FuncType,
    func: Function,
}

#[derive(Clone)]
pub(crate) enum Function {
    ModuleFunction(ModuleFunction),
    HostFunction,
}

#[derive(Clone)]
pub(crate) struct ModuleFunction {
    module: usize,
    code: Func,
}

impl FunctionInstance {
    pub fn instantiate(
        func: Func,
        tpe: FuncType,
        module_id: usize,
        _manager: &mut RuntimeManager,
    ) -> FunctionInstance {
        FunctionInstance {
            tpe,
            func: Function::ModuleFunction(ModuleFunction {
                module: module_id,
                code: func,
            }),
        }
    }
}

impl IntoStore for FunctionInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Function(self)
    }
}

impl FunctionInstance {
    pub fn execute(&self, runtime: &mut Runtime) {
        unimplemented!();
    }
}
