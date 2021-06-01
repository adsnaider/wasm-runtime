use wasm_parse::wasm::elem::Elem;
use wasm_parse::wasm::types::RefType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::values::Ref;
use crate::Instantiate;

pub(crate) struct ElementInstance {
    tpe: RefType,
    elem: Vec<Ref>,
}
impl Instantiate for Elem {
    type Instance = ElementInstance;
    fn instantiate(&self, store: &mut RuntimeManager) -> Self::Instance {
        panic!();
    }
}
impl IntoStore for ElementInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Element(self)
    }
}
