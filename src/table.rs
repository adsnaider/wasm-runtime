use wasm_parse::wasm::table::Table;
use wasm_parse::wasm::types::TableType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::values::Ref;

pub(crate) struct TableInstance {
    tpe: TableType,
    elem: Vec<Ref>,
}

impl TableInstance {
    pub fn instantiate(table: Table, manager: &mut RuntimeManager) -> TableInstance {
        unimplemented!();
    }
}

impl IntoStore for TableInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Table(self)
    }
}
