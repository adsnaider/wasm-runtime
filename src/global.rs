use wasm_parse::wasm::global::Global;
use wasm_parse::wasm::types::GlobalType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::values::Val;

pub(crate) struct GlobalInstance {
    tpe: GlobalType,
    value: Val,
}

impl GlobalInstance {
    pub fn instantiate(global: Global, store: &mut RuntimeManager) -> GlobalInstance {
        unimplemented!();
    }
}
impl IntoStore for GlobalInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Global(self)
    }
}
