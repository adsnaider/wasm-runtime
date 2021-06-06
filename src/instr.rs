mod control;
mod numeric;

use wasm_parse::wasm::instr::Instr;

use crate::structures::{Execute, ExecutionResult, Frame};

impl Execute for Instr {
    fn execute<'a>(&'a self, frame: &mut Frame<'a>) -> ExecutionResult {
        match self {
            Instr::Numeric(x) => x.execute(frame),
            Instr::Control(x) => x.execute(frame),
            instr => unimplemented!("Can't execute instruction {:?}", instr),
        }
    }
}
