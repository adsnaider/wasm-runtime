use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::types::FuncType;

use crate::module::ModuleInstance;
use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::Instantiate;

pub(crate) struct FunctionInstance {
    tpe: FuncType,
    func: Function,
}

pub(crate) enum Function {
    ModuleFunction(ModuleFunction),
    HostFunction,
}

pub(crate) struct ModuleFunction {
    module: ModuleInstance,
    code: Func,
}

impl Instantiate for Func {
    type Instance = FunctionInstance;
    fn instantiate(&self, manager: &mut RuntimeManager) -> Self::Instance {}
}

impl IntoStore for FunctionInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Function(self)
    }
}

impl FunctionInstance {
    pub fn execute(&self, manager: &RuntimeManager) {
        unimplemented!();
    }
}
