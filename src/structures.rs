pub mod stack;

use stack::Stack;
use wasm_parse::wasm::indices::FuncIdx;
use wasm_parse::wasm::instr::Instr;
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::{NumType, RefType, ValType};

#[derive(Copy, Clone, Debug)]
pub enum Val {
    Empty,
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
    Ref(Option<u32>),
}

#[derive(Clone)]
pub struct Label {
    pub branch: usize,
}

pub struct Frame<'a> {
    module: &'a Module,
    locals: Vec<Val>,
    labels: Vec<Label>,
    lpc: usize,
    stack: Stack,
    body: &'a Vec<Instr>,
}

pub enum ExecutionResult {
    Continue,
    Trap,
    BranchTo(Label),
}

impl<'a> Frame<'a> {
    pub fn new(module: &'a Module, body: &'a Vec<Instr>, locals: Vec<Val>) -> Frame<'a> {
        Frame {
            module,
            body,
            locals,
            labels: vec![Label { branch: 0 }],
            lpc: 0,
            stack: Stack::default(),
        }
    }

    pub fn from_index(module: &'a Module, idx: FuncIdx, args: Vec<Val>) -> Frame<'a> {
        let func = &module.funcs[*idx.0 as usize];
        let mut locals = Vec::new();
        locals.reserve(func.locals.len());
        for local in &func.locals {
            locals.push(match local {
                ValType::Num(NumType::I32) => Val::I32(0),
                ValType::Num(NumType::I64) => Val::I64(0),
                ValType::Num(NumType::F32) => Val::F32(0.0),
                ValType::Num(NumType::F64) => Val::F64(0.0),
                ValType::Ref(_) => Val::Ref(None),
            })
        }
        Frame::new(
            module,
            &func.body.instr,
            args.into_iter().chain(locals).collect(),
        )
    }

    pub fn get_module(&self) -> &'a Module {
        &self.module
    }

    pub fn execute(mut self) -> Vec<Val> {
        while self.lpc < self.body.len() {
            self.lpc = match self.body[self.lpc].execute(&mut self) {
                ExecutionResult::Continue => self.lpc + 1,
                ExecutionResult::Trap => panic!("Received trap!"),
                ExecutionResult::BranchTo(label) => label.branch,
            }
        }
        self.stack.take()
    }

    pub fn push_value(&mut self, value: Val) {
        self.stack.push(value);
    }

    pub fn pop_value(&mut self) -> Result<Val, stack::StackError> {
        self.stack.pop()
    }

    pub fn extend_value<T: IntoIterator<Item = Val>>(&mut self, values: T) {
        self.stack.extend(values);
    }

    pub fn push_label(&mut self, label: Label) {
        self.labels.push(label);
    }
}

pub trait Execute {
    fn execute<'a>(&'a self, frame: &mut Frame<'a>) -> ExecutionResult;
}

#[macro_export]
macro_rules! safe_pop {
    ( $frame:ident, $type:path ) => {{
        let value = $frame.pop_value().expect("Missing value");
        match value {
            $type(x) => x,
            _ => panic!("Incorrect type!"),
        }
    }};
    ( $frame:ident, $type1:path, $type2:path ) => {{
        let value2 = $frame.pop_value().expect("Missing value");
        let value1 = $frame.pop_value().expect("Missing value");
        match (value1, value2) {
            ($type1(x), $type2(y)) => (x, y),
            _ => panic!("Incorrect type!"),
        }
    }};
}
