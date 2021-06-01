use wasm_parse::wasm::mem::Mem;
use wasm_parse::wasm::types::MemType;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::Instantiate;

pub(crate) struct MemoryInstance {
    tpe: MemType,
    data: Vec<u8>,
}
impl Instantiate for Mem {
    type Instance = MemoryInstance;
    fn instantiate(&self, store: &mut RuntimeManager) -> Self::Instance {
        unimplemented!();
    }
}
impl IntoStore for MemoryInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Memory(self)
    }
}
