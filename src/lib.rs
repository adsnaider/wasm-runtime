//! Runs a web assembly program.
//!
//! Some limitations:
//!   * Currently, the runtime can run a single wasm file (no imports).
//!   * Numeric instructions only, no memory, no tables. Maybe control instructions like branching,
//!   jumping.
//!   * Wasm output is returned.

use wasm_parse::wasm::module::Module;

mod func;
mod instr;
pub(crate) mod stack;

#[derive(Copy, Clone, Debug)]
pub enum Val {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

pub struct Context<'a> {
    pub module: &'a Module,
}

trait Execute {
    fn execute(&self, stack: &mut Vec<Val>, context: &Context);
}

pub fn execute_module(module: &Module, inputs: &[Val]) -> Vec<Val> {
    assert!(module.start.is_some(), "Module must have a start function");
    let context = Context { module };
    let mut stack = Vec::new();
    stack.extend(inputs);
    module.funcs[*module.start.as_ref().unwrap().func.0 as usize].execute(&mut stack, &context);
    stack
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
