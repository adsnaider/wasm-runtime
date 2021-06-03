use wasm_parse::wasm::global::Global;
use wasm_parse::wasm::types::GlobalType;

use crate::runtime_manager::Loader;
use crate::values::Val;

pub(crate) struct GlobalInstance {
    tpe: GlobalType,
    value: Val,
}

impl GlobalInstance {
    pub fn instantiate(global: Global, store: &mut Loader) -> GlobalInstance {
        unimplemented!();
    }
}
