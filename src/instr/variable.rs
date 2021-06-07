use wasm_parse::wasm::instr::VariableInstr;

use crate::structures::{Execute, ExecutionResult, Executor};

impl Execute for VariableInstr {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult {
        match self {
            VariableInstr::GlobalGet(_gidx) => {
                unimplemented!("Global instructions not yet implemented.")
            }
            VariableInstr::GlobalSet(_gidx) => {
                unimplemented!("Global instructions not yet implemented.")
            }
            VariableInstr::LocalGet(lidx) => executor.push_value(executor.get_local(*lidx)),
            VariableInstr::LocalTee(lidx) => {
                let val = executor.pop_value().expect("No value in stack");
                executor.push_value(val);
                executor.push_value(val);
                executor.push_value(executor.get_local(*lidx));
            }
            VariableInstr::LocalSet(lidx) => {
                let value = executor.pop_value().expect("No value in stack");
                executor.set_local(*lidx, value);
            }
        }
        ExecutionResult::Continue
    }
}
