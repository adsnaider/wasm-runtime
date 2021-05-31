use wasm_parse::wasm::types::TableType;

use crate::values::Ref;

pub(crate) struct TableInstance {
    tpe: TableType,
    elem: Vec<Ref>,
}
