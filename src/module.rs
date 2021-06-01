use std::borrow::Borrow;

use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::FuncType;
use wasm_parse::wasm::values::Name;

use crate::export::ExportInstance;
use crate::external::ExternVal;
use crate::runtime_manager::RuntimeManager;
use crate::store::IntoStore;
use crate::Instantiate;

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

impl Instantiate for Module {
    type Instance = ModuleInstance;
    fn instantiate(&self, manager: &mut RuntimeManager) -> Self::Instance {
        let mut imported_funcaddrs: Vec<usize> = Vec::new();
        let mut imported_tableaddrs = Vec::new();
        let mut imported_memaddrs = Vec::new();
        let mut imported_globaladdrs = Vec::new();

        for import in &self.imports {
            let module = manager.get_or_instantiate_module(&import.module).unwrap();
            let exported = module.find_export(&import.name).unwrap();
            match exported {
                ExternVal::Func(x) => imported_funcaddrs.push(x),
                ExternVal::Table(x) => imported_tableaddrs.push(x),
                ExternVal::Mem(x) => imported_memaddrs.push(x),
                ExternVal::Global(x) => imported_globaladdrs.push(x),
            }
        }

        let funcaddrs: Vec<usize> = self
            .funcs
            .iter()
            .map(|x| x.instantiate(manager).into_store(manager.mut_store()))
            .collect();

        let mut start = None;
        if self.start.is_some() {
            let idx = self.start.borrow().as_ref().unwrap().func.0;
            start = Some(funcaddrs[*idx as usize])
        }

        let funcaddrs = imported_funcaddrs.into_iter().chain(funcaddrs).collect();
        let tableaddrs = imported_tableaddrs
            .into_iter()
            .chain(
                self.tables
                    .iter()
                    .map(|x| x.instantiate(manager).into_store(manager.mut_store())),
            )
            .collect();
        let memaddrs = imported_memaddrs
            .into_iter()
            .chain(
                self.mems
                    .iter()
                    .map(|x| x.instantiate(manager).into_store(manager.mut_store())),
            )
            .collect();
        let globaladdrs = imported_globaladdrs
            .into_iter()
            .chain(
                self.globals
                    .iter()
                    .map(|x| x.instantiate(manager).into_store(manager.mut_store())),
            )
            .collect();
        let elemaddrs = self
            .elems
            .iter()
            .map(|x| x.instantiate(manager).into_store(manager.mut_store()))
            .collect();
        let dataaddrs = self
            .datas
            .iter()
            .map(|x| x.instantiate(manager).into_store(manager.mut_store()))
            .collect();
        let exports = self
            .exports
            .iter()
            .map(|x| x.instantiate(manager))
            .collect();

        ModuleInstance {
            types: self.types.clone(),
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
