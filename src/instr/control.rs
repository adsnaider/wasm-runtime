use wasm_parse::wasm::indices::FuncIdx;
use wasm_parse::wasm::instr::ControlInstr;
use wasm_parse::wasm::types::{NumType, RefType, ValType};

use crate::structures::Val;
use crate::structures::{Execute, ExecutionResult, Frame};

impl Execute for ControlInstr {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        match self {
            ControlInstr::Nop => ExecutionResult::Continue,
            ControlInstr::Unreachable => ExecutionResult::Trap,
            ControlInstr::Block(x) => unimplemented!("Can't execute a block directly"),
            ControlInstr::Loop(_) => unimplemented!("Instruction not implemented"),
            ControlInstr::If(_) => unimplemented!("Instruction not implemented"),
            ControlInstr::Branch(_) => unimplemented!("Instruction not implemented"),
            ControlInstr::BranchIf(_) => unimplemented!("Instruction not implemented"),
            ControlInstr::BranchTable(_labels, _label) => {
                unimplemented!("Instruction not implemented")
            }
            ControlInstr::Return => {
                unreachable!("Return instruction should be caught in function execution")
            }
            ControlInstr::Call(fidx) => {
                let result = generate_function_frame(frame, *fidx).execute();
                frame.extend_value(result);
                ExecutionResult::Continue
            }
            ControlInstr::CallIndirect(_table, _tpe) => {
                unimplemented!("Instruction not implemented")
            }
        }
    }
}

fn generate_function_frame<'a>(frame: &mut Frame<'a>, idx: FuncIdx) -> Frame<'a> {
    let func = &frame.get_module().funcs[*idx.0 as usize];
    let params = &frame.get_module().types[*func.index.0 as usize]
        .params
        .types;
    let mut args = Vec::new();
    args.resize(params.len(), Val::Empty);
    for (idx, param) in params.iter().enumerate().rev() {
        let val = frame
            .pop_value()
            .expect("Validation error! Not enough arguments for function.");
        println!("Index: {}", idx);
        match (param, val) {
            (ValType::Num(NumType::I32), Val::I32(x)) => {}
            (ValType::Num(NumType::I64), Val::I64(x)) => {}
            (ValType::Num(NumType::F32), Val::F32(x)) => {}
            (ValType::Num(NumType::F64), Val::F64(x)) => {}
            (ValType::Ref(RefType::FuncRef), Val::Ref(r)) => {}
            (ValType::Ref(RefType::ExternRef), Val::Ref(r)) => {}
            _ => panic!("Validation error! Parameter and argument types don't match"),
        }
        args[idx] = val;
    }
    Frame::from_index(&frame.get_module(), idx, args)
}
