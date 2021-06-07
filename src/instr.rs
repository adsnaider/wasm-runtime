mod control;
mod memory;
mod numeric;
mod reference;
mod table;
mod variable;

use wasm_parse::wasm::instr::Instr;

use crate::safe_pop;
use crate::structures::{Execute, ExecutionResult, Executor, Val};

impl Execute for Instr {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult {
        match self {
            Instr::Numeric(x) => x.execute(executor),
            Instr::Control(x) => x.execute(executor),
            Instr::Dropp => {
                executor.pop_value().expect("Can't pop value from stack");
                ExecutionResult::Continue
            }
            Instr::Memory(_mem_instr) => unimplemented!("Memory instructions not implemented"),
            Instr::Reference(ref_instr) => ref_instr.execute(executor),
            Instr::Select(_types) => {
                let c = safe_pop!(executor, Val::I32);
                let v2 = executor.pop_value().expect("Can't pop value from stack");
                let v1 = executor.pop_value().expect("Can't pop value from stack");
                assert_eq!(
                    std::mem::discriminant(&v1),
                    std::mem::discriminant(&v2),
                    "Select instruction must use values of the same type."
                );
                match c {
                    0 => executor.push_value(v2),
                    _ => executor.push_value(v1),
                }
                ExecutionResult::Continue
            }
            Instr::Table(_table_instr) => unimplemented!("Table instructions not implemented."),
            Instr::Variable(var_instr) => var_instr.execute(executor),
        }
    }
}
