use wasm_parse::wasm::table::Table;
use wasm_parse::wasm::types::TableType;

use crate::runtime_manager::Loader;
use crate::values::Ref;

pub(crate) struct TableInstance {
    tpe: TableType,
    elem: Vec<Ref>,
}

impl TableInstance {
    pub fn instantiate(table: Table, manager: &mut Loader) -> TableInstance {
        unimplemented!();
    }
}
