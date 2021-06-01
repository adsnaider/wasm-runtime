use std::collections::HashMap;

use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::values::Name;

use crate::module::ModuleInstance;
use crate::store::Store;

pub(crate) struct RuntimeManager {
    wasm: HashMap<String, Module>,
    store: Store,
}

impl RuntimeManager {
    pub fn mut_store(&mut self) -> &mut Store {
        &mut self.store
    }

    pub fn get_or_instantiate_module(&mut self, module: &Name) -> Result<ModuleInstance, String> {
        panic!()
    }
}
