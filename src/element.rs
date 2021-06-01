use wasm_parse::wasm::elem::Elem;
use wasm_parse::wasm::types::RefType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::values::Ref;

pub(crate) struct ElementInstance {
    tpe: RefType,
    elem: Vec<Ref>,
}
impl ElementInstance {
    pub fn instantiate(elem: Elem, store: &mut RuntimeManager) -> ElementInstance {
        unimplemented!();
    }
}
impl IntoStore for ElementInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Element(self)
    }
}
