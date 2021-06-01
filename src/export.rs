use wasm_parse::wasm::export::Export;
use wasm_parse::wasm::values::Name;

use crate::external::ExternVal;
use crate::runtime_manager::RuntimeManager;
use crate::Instantiate;

pub(crate) struct ExportInstance {
    name: Name,
    value: ExternVal,
}

impl Instantiate for Export {
    type Instance = ExportInstance;
    fn instantiate(&self, store: &mut RuntimeManager) -> Self::Instance {
        panic!();
    }
}
