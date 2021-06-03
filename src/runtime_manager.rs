use wasm_parse::wasm::module::Module;

use crate::module::ModuleInstance;
use crate::store::Store;

pub struct Loading;
pub struct Running;

pub struct Loader {
    module: Option<Module>,
    store: Store,
}

pub struct Runtime {
    module: ModuleInstance,
    store: Store,
}

#[derive(Copy, Clone)]
pub struct ModuleIdx {
    idx: usize,
}

impl Loader {
    pub fn new(module: Module) -> Loader {
        Loader {
            module: Some(module),
            store: Default::default(),
        }
    }

    pub fn instantiate(mut self) -> Runtime {
        Runtime {
            module: ModuleInstance::instantiate(
                self.module.take().unwrap(),
                ModuleIdx { idx: 0 },
                &mut self.store,
            ),
            store: self.store,
        }
    }
}
