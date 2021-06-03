//! Runs a web assembly program.
//!
//! Some limitations:
//!   * Currently, the runtime can run a single wasm file (no imports).
//!   * Numeric instructions only, no memory, no tables. Maybe control instructions like branching,
//!   jumping.
//!   * Wasm output is returned.

use std::ops::Deref;

use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::instr::{Instr, NumericInstr};
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::{NumType, ValType};

pub fn execute(module: &Module, inputs: &[u32]) -> Vec<u32> {
    assert!(module.start.is_some(), "Module must have a start function");
    let mut stack = Vec::new();
    stack.extend(inputs.iter());
    execute_func(
        &module.funcs[*module.start.as_ref().unwrap().func.0.deref() as usize],
        &mut stack,
        &module,
    );
    stack
}

fn execute_func(func: &Func, stack: &mut Vec<u32>, module: &Module) {
    assert!(
        stack.len() >= func.locals.len(),
        "Mismatch between params and arguments passed."
    );
    assert!(func.locals.iter().all(|x| match x {
        ValType::Num(NumType::I32) => true,
        _ => false,
    }));
    for instr in &func.body.instr {
        execute_instr(&instr, stack);
    }
}

fn execute_instr(instr: &Instr, stack: &mut Vec<u32>) {
    match instr {
        Instr::Numeric(NumericInstr::I32Const(ref x)) => stack.push(x.0),
        instr => unimplemented!("Can't execute instruction {:?}", instr),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
