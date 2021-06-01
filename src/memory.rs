use wasm_parse::wasm::mem::Mem;
use wasm_parse::wasm::types::MemType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};

pub(crate) struct MemoryInstance {
    tpe: MemType,
    data: Vec<u8>,
}
impl MemoryInstance {
    pub fn instantiate(mem: Mem, store: &mut RuntimeManager) -> MemoryInstance {
        unimplemented!();
    }
}
impl IntoStore for MemoryInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Memory(self)
    }
}
