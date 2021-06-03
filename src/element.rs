use wasm_parse::wasm::elem::Elem;
use wasm_parse::wasm::types::RefType;

use crate::runtime_manager::Loader;
use crate::values::Ref;

pub(crate) struct ElementInstance {
    tpe: RefType,
    elem: Vec<Ref>,
}
impl ElementInstance {
    pub fn instantiate(elem: Elem, store: &mut Loader) -> ElementInstance {
        unimplemented!();
    }
}
