use wasm_parse::wasm::mem::Mem;
use wasm_parse::wasm::types::MemType;

use crate::runtime_manager::Loader;

pub(crate) struct MemoryInstance {
    tpe: MemType,
    data: Vec<u8>,
}
impl MemoryInstance {
    pub fn instantiate(mem: Mem, store: &mut Loader) -> MemoryInstance {
        unimplemented!();
    }
}
