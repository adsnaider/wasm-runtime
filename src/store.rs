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

pub(crate) enum StoreElement {
    Function(FunctionInstance),
    Table(TableInstance),
    Memory(MemoryInstance),
    Global(GlobalInstance),
    Element(ElementInstance),
    Data(DataInstance),
}

pub(crate) trait IntoStore: Sized {
    fn to_element(self) -> StoreElement;
    fn into_store(self, store: &mut Store) -> usize {
        store.push(self.to_element())
    }
}

impl Store {
    pub fn push(&mut self, element: StoreElement) -> usize {
        match element {
            StoreElement::Function(f) => {
                self.funcs.push(f);
                self.funcs.len() - 1
            }
            StoreElement::Table(t) => {
                self.tables.push(t);
                self.tables.len() - 1
            }
            StoreElement::Memory(m) => {
                self.mems.push(m);
                self.mems.len() - 1
            }
            StoreElement::Global(g) => {
                self.globals.push(g);
                self.globals.len() - 1
            }
            StoreElement::Element(e) => {
                self.elems.push(e);
                self.elems.len() - 1
            }
            StoreElement::Data(d) => {
                self.datas.push(d);
                self.datas.len() - 1
            }
        }
    }
}
