use std::collections::HashMap;

use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::values::Name;

use crate::module::ModuleInstance;
use crate::store::Store;

#[derive(Default)]
pub struct RuntimeManager {
    names: HashMap<String, usize>,
    modules: Vec<Module>,
    start: Option<usize>,
    store: Option<Store>,
}

#[derive(Default)]
pub(crate) struct Runtime {
    modules: Vec<ModuleInstance>,
    store: Store,
}

impl Runtime {
    pub fn get_store(&self) -> &Store {
        &self.store
    }

    pub fn get_module(&self, idx: usize) -> &ModuleInstance {
        &self.modules[idx]
    }
}

impl RuntimeManager {
    pub fn new() -> RuntimeManager {
        let mut manager = RuntimeManager::default();
        manager.store = Some(Store::default());
        manager
    }

    pub(crate) fn get_or_instantiate_module(
        &mut self,
        module: &Name,
    ) -> Result<ModuleInstance, String> {
        unimplemented!()
    }

    pub(crate) fn get_mut_store(&mut self) -> &mut Store {
        match &mut self.store {
            Some(s) => s,
            None => panic!("No store available"),
        }
    }

    pub fn load_modules(&mut self, modules: Vec<Module>) {
        self.modules.extend(modules.into_iter());
        let named_modules = self
            .modules
            .iter()
            .enumerate()
            .filter(|(_, module)| module.name.is_some());

        for (index, module) in named_modules {
            self.names
                .try_insert(module.name.clone().unwrap().name, index)
                .unwrap();
        }
    }

    pub fn start(&mut self) {
        let mut runtime = Runtime::default();
        for (id, module) in std::mem::take(&mut self.modules).into_iter().enumerate() {
            let instance = ModuleInstance::instantiate(module, id, self);
            self.start.or(instance.start());
            runtime.modules.push(instance);
        }

        match self.start {
            Some(s) => {
                runtime.store.get_func(s).borrow().execute(&mut runtime);
            }
            None => panic!("Can't start web assembly. No start function."),
        }
    }
}
