use wasm_parse::wasm::export::Export;
use wasm_parse::wasm::values::Name;

use crate::external::ExternVal;
use crate::runtime_manager::Loader;

#[derive(Clone)]
pub(crate) struct ExportInstance {
    name: Name,
    value: ExternVal,
}

impl ExportInstance {
    pub fn instantiate(export: Export, store: &mut Loader) -> ExportInstance {
        unimplemented!();
        /*
        ExportInstance { name: export.name }
        */
    }
}
