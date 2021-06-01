use wasm_parse::wasm::data::Data;

use crate::runtime_manager::RuntimeManager;
use crate::store::{IntoStore, StoreElement};
use crate::Instantiate;

pub(crate) struct DataInstance {
    data: Vec<u8>,
}

impl Instantiate for Data {
    type Instance = DataInstance;
    fn instantiate(&self, store: &mut RuntimeManager) -> Self::Instance {
        panic!();
    }
}

impl IntoStore for DataInstance {
    fn to_element(self) -> StoreElement {
        StoreElement::Data(self)
    }
}
