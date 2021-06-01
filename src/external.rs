#[derive(Clone)]
pub(crate) enum ExternVal {
    Func(usize),
    Table(usize),
    Mem(usize),
    Global(usize),
}
