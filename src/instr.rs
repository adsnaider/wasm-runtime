mod control;
mod numeric;

use wasm_parse::wasm::instr::Instr;

use crate::structures::{Execute, ExecutionResult, Executor};

impl Execute for Instr {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult {
        match self {
            Instr::Numeric(x) => x.execute(executor),
            Instr::Control(x) => x.execute(executor),
            instr => unimplemented!("Can't execute instruction {:?}", instr),
        }
    }
}
