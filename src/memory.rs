use wasm_parse::wasm::types::MemType;

pub(crate) struct MemoryInstance {
    tpe: MemType,
    data: Vec<u8>,
}
