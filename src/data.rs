use wasm_parse::wasm::data::Data;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};

pub(crate) struct DataInstance {
    data: Vec<u8>,
}

impl DataInstance {
    pub fn instantiate(data: Data, store: &mut RuntimeManager) -> DataInstance {
        unimplemented!();
    }
}

impl IntoStore for DataInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Data(self)
    }
}
