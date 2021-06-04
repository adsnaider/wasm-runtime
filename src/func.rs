use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::types::{NumType, ValType};

use crate::{Context, Execute, Val};

impl Execute for Func {
    fn execute(&self, stack: &mut Vec<Val>, context: &Context) {
        assert!(
            stack.len() >= self.locals.len(),
            "Mismatch between params and arguments passed."
        );
        assert!(self.locals.iter().all(|x| match x {
            ValType::Num(NumType::I32) => true,
            _ => false,
        }));
        for instr in &self.body.instr {
            instr.execute(stack, context);
        }
    }
}
