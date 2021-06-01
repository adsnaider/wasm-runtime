use std::borrow::Borrow;

use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::FuncType;
use wasm_parse::wasm::values::Name;

use crate::data::DataInstance;
use crate::element::ElementInstance;
use crate::export::ExportInstance;
use crate::external::ExternVal;
use crate::function::FunctionInstance;
use crate::global::GlobalInstance;
use crate::memory::MemoryInstance;
use crate::runtime_manager::RuntimeManager;
use crate::store::IntoStore;
use crate::table::TableInstance;

#[derive(Clone)]
pub(crate) struct ModuleInstance {
    types: Vec<FuncType>,
    funcaddrs: Vec<usize>,
    tableaddrs: Vec<usize>,
    memaddrs: Vec<usize>,
    globaladdrs: Vec<usize>,
    elemaddrs: Vec<usize>,
    dataaddrs: Vec<usize>,
    exports: Vec<ExportInstance>,
    start: Option<usize>,
}

impl ModuleInstance {
    pub fn instantiate(module: Module, id: usize, manager: &mut RuntimeManager) -> ModuleInstance {
        let mut imported_funcaddrs: Vec<usize> = Vec::new();
        let mut imported_tableaddrs = Vec::new();
        let mut imported_memaddrs = Vec::new();
        let mut imported_globaladdrs = Vec::new();

        for import in &module.imports {
            let module = manager.get_or_instantiate_module(&import.module).unwrap();
            let exported = module.find_export(&import.name).unwrap();
            match exported {
                ExternVal::Func(x) => imported_funcaddrs.push(x),
                ExternVal::Table(x) => imported_tableaddrs.push(x),
                ExternVal::Mem(x) => imported_memaddrs.push(x),
                ExternVal::Global(x) => imported_globaladdrs.push(x),
            }
        }

        let funcaddrs: Vec<usize> = module
            .funcs
            .iter()
            .map(|x| {
                FunctionInstance::instantiate(
                    x.clone(),
                    module.types[*x.index.0 as usize].clone(),
                    id,
                    manager,
                )
                .into_store(manager.get_mut_store())
            })
            .collect();

        let mut start = None;
        if module.start.is_some() {
            let idx = module.start.borrow().as_ref().unwrap().func.0;
            start = Some(funcaddrs[*idx as usize])
        }

        let funcaddrs = imported_funcaddrs.into_iter().chain(funcaddrs).collect();
        let tableaddrs = imported_tableaddrs
            .into_iter()
            .chain(module.tables.into_iter().map(|x| {
                TableInstance::instantiate(x, manager).into_store(manager.get_mut_store())
            }))
            .collect();
        let memaddrs = imported_memaddrs
            .into_iter()
            .chain(module.mems.into_iter().map(|x| {
                MemoryInstance::instantiate(x, manager).into_store(manager.get_mut_store())
            }))
            .collect();
        let globaladdrs = imported_globaladdrs
            .into_iter()
            .chain(module.globals.into_iter().map(|x| {
                GlobalInstance::instantiate(x, manager).into_store(manager.get_mut_store())
            }))
            .collect();
        let elemaddrs = module
            .elems
            .into_iter()
            .map(|x| ElementInstance::instantiate(x, manager).into_store(manager.get_mut_store()))
            .collect();
        let dataaddrs = module
            .datas
            .into_iter()
            .map(|x| DataInstance::instantiate(x, manager).into_store(manager.get_mut_store()))
            .collect();
        let exports = module
            .exports
            .into_iter()
            .map(|x| ExportInstance::instantiate(x, manager))
            .collect();

        ModuleInstance {
            types: module.types,
            funcaddrs,
            tableaddrs,
            memaddrs,
            globaladdrs,
            elemaddrs,
            dataaddrs,
            exports,
            start,
        }
    }
}

impl ModuleInstance {
    pub fn find_export(&self, name: &Name) -> Result<ExternVal, ()> {
        unimplemented!();
    }

    pub fn start(&self) -> Option<usize> {
        self.start
    }
}
