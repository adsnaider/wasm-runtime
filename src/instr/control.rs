use wasm_parse::wasm::instr::ControlInstr;

use crate::structures::{Execute, ExecutionResult, Frame};

impl Execute for ControlInstr {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        match self {
            ControlInstr::Nop => {}
            ControlInstr::Unreachable => panic!("Reached unrechable instruction!"),
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
            ControlInstr::Call(fidx) => unimplemented!("Method call unimplemented"), /* frame.module.funcs[*fidx.0 as usize].execute(stack, frame), */
            ControlInstr::CallIndirect(_table, _tpe) => {
                unimplemented!("Instruction not implemented")
            }
        }
        ExecutionResult::Continue
    }
}
