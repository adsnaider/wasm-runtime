use wasm_parse::wasm::types::GlobalType;

use crate::values::Val;

pub(crate) struct GlobalInstance {
    tpe: GlobalType,
    value: Val,
}
