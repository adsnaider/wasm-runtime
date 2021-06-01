pub(crate) mod data;
pub(crate) mod element;
pub(crate) mod export;
pub(crate) mod external;
pub(crate) mod function;
pub(crate) mod global;
pub(crate) mod memory;
pub(crate) mod module;
pub(crate) mod runtime_manager;
pub(crate) mod stack;
pub(crate) mod store;
pub(crate) mod table;
pub(crate) mod values;

pub(crate) trait Instantiate {
    type Instance: Sized;
    fn instantiate(&self, manager: &mut runtime_manager::RuntimeManager) -> Self::Instance;
}
