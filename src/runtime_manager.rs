use std::collections::HashMap;

use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::values::Name;

use crate::module::ModuleInstance;
use crate::store::Store;
use crate::Instantiate;

#[derive(Default)]
pub struct RuntimeManager {
    names: HashMap<String, usize>,
    module_definitions: Vec<Module>,
    module_instances: Vec<ModuleInstance>,
    start: Option<usize>,
    store: Store,
}

impl RuntimeManager {
    pub fn new() -> RuntimeManager {
        Default::default()
    }

    pub(crate) fn mut_store(&mut self) -> &mut Store {
        &mut self.store
    }

    pub(crate) fn get_or_instantiate_module(
        &mut self,
        module: &Name,
    ) -> Result<ModuleInstance, String> {
        unimplemented!()
    }

    pub fn load_modules(&mut self, modules: Vec<Module>) {
        self.module_definitions.extend(modules.into_iter());
        let named_modules = self
            .module_definitions
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
        for module in std::mem::take(&mut self.module_definitions) {
            let instance = module.instantiate(self);
            self.start.or(instance.start());
            self.module_instances.push(instance);
        }

        match self.start {
            Some(s) => self.store.get_func(s).execute(self),
            None => panic!("Can't start web assembly. No start function."),
        }
    }
}
