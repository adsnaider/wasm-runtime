use wasm_parse::wasm::values::Name;

use crate::external::ExternVal;

pub(crate) struct ExportInstance {
    name: Name,
    value: ExternVal,
}
