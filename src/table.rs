use wasm_parse::wasm::table::Table;
use wasm_parse::wasm::types::TableType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::values::Ref;
use crate::Instantiate;

pub(crate) struct TableInstance {
    tpe: TableType,
    elem: Vec<Ref>,
}

impl Instantiate for Table {
    type Instance = TableInstance;
    fn instantiate(&self, manager: &mut RuntimeManager) -> Self::Instance {
        unimplemented!();
    }
}

impl IntoStore for TableInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Table(self)
    }
}
