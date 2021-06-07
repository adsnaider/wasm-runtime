use std::cell::RefCell;
use std::rc::Rc;

use wasm_parse::wasm::indices::FuncIdx;
use wasm_parse::wasm::instr::BlockType;
use wasm_parse::wasm::instr::ControlInstr;
use wasm_parse::wasm::types::{NumType, RefType, ValType};

use crate::safe_pop;
use crate::structures::Val;
use crate::structures::{Execute, ExecutionResult, Executor, Frame, LabelPos};

impl Execute for ControlInstr {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult {
        match self {
            ControlInstr::Nop => ExecutionResult::Continue,
            ControlInstr::Unreachable => ExecutionResult::Trap,
            ControlInstr::Block(x) => {
                let mut executor = executor.replicate(&x.instr);
                let arity = match x.tpe {
                    BlockType::Val(_) => 1,
                    BlockType::Empty => 0,
                    BlockType::Type(t) => executor.get_module().types[*t.0 as usize]
                        .result
                        .types
                        .len(),
                };
                executor.push_label(LabelPos::BlockEnd, arity);
                executor.execute()
            }
            ControlInstr::Loop(x) => {
                let mut executor = executor.replicate(&x.instr);
                let arity = match x.tpe {
                    BlockType::Val(_) => 1,
                    BlockType::Empty => 0,
                    BlockType::Type(t) => executor.get_module().types[*t.0 as usize]
                        .result
                        .types
                        .len(),
                };
                executor.push_label(LabelPos::BlockStart, arity);
                executor.execute()
            }
            ControlInstr::If(x) => {
                let arity = match x.tpe {
                    BlockType::Val(_) => 1,
                    BlockType::Empty => 0,
                    BlockType::Type(t) => executor.get_module().types[*t.0 as usize]
                        .result
                        .types
                        .len(),
                };

                let cond = safe_pop!(executor, Val::I32);
                let mut executor = match cond {
                    0 => executor.replicate(&x.else_br),
                    _ => executor.replicate(&x.if_br),
                };
                executor.push_label(LabelPos::BlockEnd, arity);
                executor.execute()
            }
            ControlInstr::Branch(l) => ExecutionResult::BranchTo(*l),
            ControlInstr::BranchIf(l) => {
                let cond = safe_pop!(executor, Val::I32);
                match cond {
                    0 => ExecutionResult::Continue,
                    _ => ExecutionResult::BranchTo(*l),
                }
            }
            ControlInstr::BranchTable(_labels, _label) => {
                unimplemented!("Instruction not implemented")
            }
            ControlInstr::Return => ExecutionResult::Return,
            ControlInstr::Call(fidx) => {
                let (new_frame, new_executor) = generate_function_frame(executor, *fidx);
                new_executor.execute();
                let new_frame = match Rc::try_unwrap(new_frame) {
                    Ok(f) => f,
                    Err(_) => panic!("Couldn't retreive function frame"),
                };
                executor.extend_value(new_frame.take().unwrap().take());
                ExecutionResult::Continue
            }
            ControlInstr::CallIndirect(_table, _tpe) => {
                unimplemented!("Instruction not implemented")
            }
        }
    }
}

fn generate_function_frame<'a>(
    executor: &mut Executor<'a>,
    idx: FuncIdx,
) -> (Rc<RefCell<Option<Frame<'a>>>>, Executor<'a>) {
    let func = &executor.get_module().funcs[*idx.0 as usize];
    let params = &executor.get_module().types[*func.index.0 as usize]
        .params
        .types;
    let mut args = Vec::new();
    args.resize(params.len(), Val::Empty);
    for (idx, param) in params.iter().enumerate().rev() {
        let val = executor
            .pop_value()
            .expect("Validation error! Not enough arguments for function.");
        println!("Index: {}", idx);
        match (param, val) {
            (ValType::Num(NumType::I32), Val::I32(_)) => {}
            (ValType::Num(NumType::I64), Val::I64(_)) => {}
            (ValType::Num(NumType::F32), Val::F32(_)) => {}
            (ValType::Num(NumType::F64), Val::F64(_)) => {}
            (ValType::Ref(RefType::FuncRef), Val::Ref(_)) => {}
            (ValType::Ref(RefType::ExternRef), Val::Ref(_)) => {}
            _ => panic!("Validation error! Parameter and argument types don't match"),
        }
        args[idx] = val;
    }
    Frame::from_index(executor.get_module(), idx, args)
}
