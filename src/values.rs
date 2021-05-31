pub(crate) enum Num {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

pub(crate) enum Ref {
    Func(Option<usize>),
    Extern(Option<usize>),
}

pub(crate) enum Val {
    Ref(Ref),
    Num(Num),
}
