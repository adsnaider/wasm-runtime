use wasm_parse::wasm::types::RefType;

use crate::values::Ref;

pub(crate) struct ElementInstance {
    tpe: RefType,
    elem: Vec<Ref>,
}
