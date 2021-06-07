use wasm_parse::wasm::instr::ReferenceInstr;

use crate::safe_pop;
use crate::structures::{Execute, ExecutionResult, Executor, Val};

impl Execute for ReferenceInstr {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult {
        match self {
            ReferenceInstr::RefFunc(_fidx) => {
                unimplemented!("Function references not yet supported.")
            }
            ReferenceInstr::RefNull(_reftpe) => executor.push_value(Val::Ref(None)),
            ReferenceInstr::RefIsNull => {
                let val = safe_pop!(executor, Val::Ref);
                match val {
                    Some(_) => executor.push_value(Val::I32(0)),
                    None => executor.push_value(Val::I32(1)),
                }
            }
        }
        ExecutionResult::Continue
    }
}
