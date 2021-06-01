use wasm_parse::wasm::export::Export;
use wasm_parse::wasm::values::Name;

use crate::external::ExternVal;
use crate::runtime_manager::RuntimeManager;

#[derive(Clone)]
pub(crate) struct ExportInstance {
    name: Name,
    value: ExternVal,
}

impl ExportInstance {
    pub fn instantiate(export: Export, store: &mut RuntimeManager) -> ExportInstance {
        ExportInstance {
            name: export.name,

        }
    }
}
