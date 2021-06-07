pub mod stack;

use std::cell::RefCell;
use std::rc::Rc;

use stack::Stack;
use wasm_parse::wasm::indices::{FuncIdx, LabelIdx};
use wasm_parse::wasm::instr::Instr;
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::{NumType, ValType};
use wasm_parse::wasm::values::U32;

#[derive(Copy, Clone, Debug)]
pub enum Val {
    Empty,
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
    Ref(Option<u32>),
}

pub struct Frame<'a> {
    module: &'a Module,
    locals: Vec<Val>,
    stack: Stack,
}

pub struct Executor<'a> {
    frame: Rc<RefCell<Option<Frame<'a>>>>,
    lpc: usize,
    labels: Vec<Label>,
    body: &'a Vec<Instr>,
}

struct Label {
    lpc: Option<usize>,
    stack_height: usize,
}

pub enum ExecutionResult {
    Continue,
    Return,
    BranchTo(LabelIdx),
    Trap,
}

impl<'a> Frame<'a> {
    pub fn new_frame_exec(
        module: &'a Module,
        body: &'a Vec<Instr>,
        locals: Vec<Val>,
    ) -> (Rc<RefCell<Option<Frame<'a>>>>, Executor<'a>) {
        let frame = Rc::new(RefCell::new(Some(Frame {
            module,
            locals,
            stack: Stack::default(),
        })));
        let executor = Executor::new(Rc::clone(&frame), body);
        (frame, executor)
    }

    pub fn from_index(
        module: &'a Module,
        idx: FuncIdx,
        args: Vec<Val>,
    ) -> (Rc<RefCell<Option<Frame<'a>>>>, Executor<'a>) {
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
        Frame::new_frame_exec(
            module,
            &func.body.instr,
            args.into_iter().chain(locals).collect(),
        )
    }

    pub fn get_module(&self) -> &'a Module {
        &self.module
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

    pub fn take(self) -> Vec<Val> {
        self.stack.take()
    }

    fn truncate_stack(&mut self, length: usize) {
        self.stack.truncate(length)
    }

    fn stack_len(&self) -> usize {
        self.stack.len()
    }
}

pub enum LabelPos {
    BlockStart,
    BlockEnd,
}

impl<'a> Executor<'a> {
    pub fn new(frame: Rc<RefCell<Option<Frame<'a>>>>, body: &'a Vec<Instr>) -> Executor<'a> {
        Executor {
            frame,
            body,
            lpc: 0,
            labels: Vec::new(),
        }
    }

    pub fn replicate(&self, body: &'a Vec<Instr>) -> Executor<'a> {
        Executor {
            frame: Rc::clone(&self.frame),
            body,
            lpc: 0,
            labels: Vec::new(),
        }
    }

    pub fn execute(mut self) -> ExecutionResult {
        while self.lpc < self.body.len() {
            self.lpc = match self.body[self.lpc].execute(&mut self) {
                ExecutionResult::Continue => self.lpc + 1,
                ExecutionResult::Return => return ExecutionResult::Return,
                ExecutionResult::Trap => panic!("Received trap!"),
                ExecutionResult::BranchTo(label) => {
                    let label = *label.0 as usize;
                    if label as usize >= self.labels.len() {
                        return ExecutionResult::BranchTo(LabelIdx(U32(
                            (label - self.labels.len()) as u32,
                        )));
                    }
                    let length = self.labels.len();
                    self.labels = self.labels.into_iter().take(length - label).collect();
                    let l = self.labels.last().unwrap();
                    self.frame
                        .borrow_mut()
                        .as_mut()
                        .unwrap()
                        .truncate_stack(l.stack_height);
                    match l.lpc {
                        Some(lpc) => lpc,
                        None => return ExecutionResult::Continue,
                    }
                }
            }
        }
        ExecutionResult::Continue
    }

    pub fn push_value(&mut self, value: Val) {
        self.frame.borrow_mut().as_mut().unwrap().push_value(value);
    }

    pub fn pop_value(&mut self) -> Result<Val, stack::StackError> {
        self.frame.borrow_mut().as_mut().unwrap().stack.pop()
    }

    pub fn extend_value<T: IntoIterator<Item = Val>>(&mut self, values: T) {
        self.frame
            .borrow_mut()
            .as_mut()
            .unwrap()
            .extend_value(values);
    }

    pub fn get_module(&self) -> &'a Module {
        self.frame.borrow().as_ref().unwrap().get_module()
    }

    pub fn push_label(&mut self, l: LabelPos) {
        match l {
            LabelPos::BlockStart => self.labels.push(Label {
                lpc: Some(self.lpc),
                stack_height: self.frame.borrow().as_ref().unwrap().stack_len(),
            }),
            LabelPos::BlockEnd => self.labels.push(Label {
                lpc: None,
                stack_height: self.frame.borrow().as_ref().unwrap().stack_len(),
            }),
        }
    }
}

pub trait Execute {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult;
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
