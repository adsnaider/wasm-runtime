use wasm_parse::wasm::types::FuncType;

use crate::export::ExportInstance;

pub(crate) struct ModuleInstance {
    types: Vec<FuncType>,
    funcaddrs: usize,
    tableaddrs: usize,
    memaddrs: usize,
    globaladdrs: usize,
    elemaddrs: usize,
    dataaddrs: usize,
    exports: Vec<ExportInstance>,
}
