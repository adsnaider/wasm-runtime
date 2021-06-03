use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::FuncType;

use crate::module::ModuleInstance;
use crate::runtime_manager::{Loader, ModuleIdx};
use crate::store::Store;

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
    pub fn instantiate(func: Func, module: ModuleIdx, loader: &mut Store, ) -> FunctionInstance {
        FunctionInstance {
            tpe,
            func: Function::ModuleFunction(ModuleFunction {
                module: module_id,
                code: func,
            }),
        }
        unimplemented!();
    }
}
