use crate::{
    data::DataInstance, element::ElementInstance, function::FunctionInstance,
    global::GlobalInstance, memory::MemoryInstance, table::TableInstance,
};

pub(crate) struct Store {
    funcs: Vec<FunctionInstance>,
    tables: Vec<TableInstance>,
    mems: Vec<MemoryInstance>,
    globals: Vec<GlobalInstance>,
    elems: Vec<ElementInstance>,
    datas: Vec<DataInstance>,
}
