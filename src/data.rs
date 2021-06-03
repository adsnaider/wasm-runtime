use wasm_parse::wasm::data::Data;

use crate::runtime_manager::Loader;

pub(crate) struct DataInstance {
    data: Vec<u8>,
}

impl DataInstance {
    pub fn instantiate(data: Data, store: &mut Loader) -> DataInstance {
        unimplemented!();
    }
}
