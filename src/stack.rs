use wasm_parse::wasm::instr::Instr;

use crate::module::ModuleInstance;
use crate::values::Val;

pub(crate) struct RuntimeStack {
    stack: Vec<StackElement>,
    // Position in stack of kth label and frame.
    labels: Vec<usize>,
    frames: Vec<usize>,
}

pub(crate) enum StackElement {
    Value(Val),
    Label(Label),
    Frame(Frame),
}

pub(crate) struct Label {
    arity: usize,
    instr: Vec<Instr>,
}

pub(crate) struct Frame {
    locals: Vec<Val>,
    module: ModuleInstance,
}
