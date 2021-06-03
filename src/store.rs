use crate::{
    data::DataInstance, element::ElementInstance, function::FunctionInstance,
    global::GlobalInstance, memory::MemoryInstance, table::TableInstance,
};

#[derive(Default)]
pub(crate) struct Store {
    // Since functions require access to the store, we need to expose some interior mutability to
    // make the borrow checker happy.
    funcs: Vec<FunctionInstance>,
    tables: Vec<TableInstance>,
    mems: Vec<MemoryInstance>,
    globals: Vec<GlobalInstance>,
    elems: Vec<ElementInstance>,
    datas: Vec<DataInstance>,
}

pub(crate) trait StoreElement {}
impl StoreElement for FunctionInstance {}
impl StoreElement for TableInstance {}
impl StoreElement for MemoryInstance {}
impl StoreElement for GlobalInstance {}
impl StoreElement for ElementInstance {}
impl StoreElement for DataInstance {}

#[derive(Copy, Clone)]
pub(crate) struct StoreIndex<T: Idx> {
    idx: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Copy, Clone)]
pub(crate) struct FunctionIdx;
#[derive(Copy, Clone)]
pub(crate) struct TableIdx;
#[derive(Copy, Clone)]
pub(crate) struct MemoryIdx;
#[derive(Copy, Clone)]
pub(crate) struct GlobalIdx;
#[derive(Copy, Clone)]
pub(crate) struct ElementIdx;
#[derive(Copy, Clone)]
pub(crate) struct DataIdx;

pub(crate) trait Idx: Copy + Clone + Sized {}
impl Idx for FunctionIdx {}
impl Idx for TableIdx {}
impl Idx for MemoryIdx {}
impl Idx for GlobalIdx {}
impl Idx for ElementIdx {}
impl Idx for DataIdx {}

pub(crate) trait StoreGet<T: Idx + Copy + Clone> {
    type Item;
    fn get(&self, idx: StoreIndex<T>) -> &Self::Item;
    fn mut_get(&mut self, idx: StoreIndex<T>) -> &mut Self::Item;
}

pub(crate) trait StorePush<T: StoreElement> {
    type Index: Idx;
    fn push(&mut self, element: T) -> StoreIndex<Self::Index>;
}

impl StorePush<FunctionInstance> for Store {
    type Index = FunctionIdx;
    fn push(&mut self, element: FunctionInstance) -> StoreIndex<Self::Index> {
        self.funcs.push(element);
        StoreIndex {
            idx: self.funcs.len() - 1,
            _phantom: Default::default(),
        }
    }
}
impl StorePush<TableInstance> for Store {
    type Index = TableIdx;
    fn push(&mut self, element: TableInstance) -> StoreIndex<Self::Index> {
        self.tables.push(element);
        StoreIndex {
            idx: self.tables.len() - 1,
            _phantom: Default::default(),
        }
    }
}
impl StorePush<MemoryInstance> for Store {
    type Index = MemoryIdx;
    fn push(&mut self, element: MemoryInstance) -> StoreIndex<Self::Index> {
        self.mems.push(element);
        StoreIndex {
            idx: self.mems.len() - 1,
            _phantom: Default::default(),
        }
    }
}
impl StorePush<GlobalInstance> for Store {
    type Index = GlobalIdx;
    fn push(&mut self, element: GlobalInstance) -> StoreIndex<Self::Index> {
        self.globals.push(element);
        StoreIndex {
            idx: self.globals.len() - 1,
            _phantom: Default::default(),
        }
    }
}
impl StorePush<ElementInstance> for Store {
    type Index = ElementIdx;
    fn push(&mut self, element: ElementInstance) -> StoreIndex<Self::Index> {
        self.elems.push(element);
        StoreIndex {
            idx: self.elems.len() - 1,
            _phantom: Default::default(),
        }
    }
}
impl StorePush<DataInstance> for Store {
    type Index = DataIdx;
    fn push(&mut self, element: DataInstance) -> StoreIndex<Self::Index> {
        self.datas.push(element);
        StoreIndex {
            idx: self.datas.len() - 1,
            _phantom: Default::default(),
        }
    }
}

impl StoreGet<FunctionIdx> for Store {
    type Item = FunctionInstance;
    fn get(&self, idx: StoreIndex<FunctionIdx>) -> &Self::Item {
        &self.funcs[idx.idx]
    }
    fn mut_get(&mut self, idx: StoreIndex<FunctionIdx>) -> &mut Self::Item {
        &mut self.funcs[idx.idx]
    }
}

impl StoreGet<TableIdx> for Store {
    type Item = TableInstance;
    fn get(&self, idx: StoreIndex<TableIdx>) -> &Self::Item {
        &self.tables[idx.idx]
    }
    fn mut_get(&mut self, idx: StoreIndex<TableIdx>) -> &mut Self::Item {
        &mut self.tables[idx.idx]
    }
}

impl StoreGet<MemoryIdx> for Store {
    type Item = MemoryInstance;
    fn get(&self, idx: StoreIndex<MemoryIdx>) -> &Self::Item {
        &self.mems[idx.idx]
    }
    fn mut_get(&mut self, idx: StoreIndex<MemoryIdx>) -> &mut Self::Item {
        &mut self.mems[idx.idx]
    }
}

impl StoreGet<GlobalIdx> for Store {
    type Item = GlobalInstance;
    fn get(&self, idx: StoreIndex<GlobalIdx>) -> &Self::Item {
        &self.globals[idx.idx]
    }
    fn mut_get(&mut self, idx: StoreIndex<GlobalIdx>) -> &mut Self::Item {
        &mut self.globals[idx.idx]
    }
}

impl StoreGet<ElementIdx> for Store {
    type Item = ElementInstance;
    fn get(&self, idx: StoreIndex<ElementIdx>) -> &Self::Item {
        &self.elems[idx.idx]
    }
    fn mut_get(&mut self, idx: StoreIndex<ElementIdx>) -> &mut Self::Item {
        &mut self.elems[idx.idx]
    }
}

impl StoreGet<DataIdx> for Store {
    type Item = DataInstance;
    fn get(&self, idx: StoreIndex<DataIdx>) -> &Self::Item {
        &self.datas[idx.idx]
    }
    fn mut_get(&mut self, idx: StoreIndex<DataIdx>) -> &mut Self::Item {
        &mut self.datas[idx.idx]
    }
}
