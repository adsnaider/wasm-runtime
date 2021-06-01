use wasm_parse::wasm::global::Global;
use wasm_parse::wasm::types::GlobalType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::values::Val;
use crate::Instantiate;

pub(crate) struct GlobalInstance {
    tpe: GlobalType,
    value: Val,
}
impl Instantiate for Global {
    type Instance = GlobalInstance;
    fn instantiate(&self, store: &mut RuntimeManager) -> Self::Instance {
        panic!();
    }
}
impl IntoStore for GlobalInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Global(self)
    }
}
