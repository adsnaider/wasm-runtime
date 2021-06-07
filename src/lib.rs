//! Runs a web assembly program.
//!
//! Some limitations:
//!   * Currently, the runtime can run a single wasm file (no imports).
//!   * Numeric instructions only, no memory, no tables. Maybe control instructions like branching,
//!   jumping.
//!   * Wasm output is returned.

mod instr;

pub(crate) mod structures;

use std::rc::Rc;

use wasm_parse::wasm::module::Module;

use crate::structures::{Frame, Val};

pub fn execute_module(module: &Module, inputs: Vec<Val>) -> Vec<Val> {
    let (frame, executor) = Frame::from_index(
        module,
        module
            .start
            .as_ref()
            .expect("Module must have start function!")
            .func,
        inputs,
    );
    executor.execute();
    match Rc::try_unwrap(frame) {
        Ok(frame) => frame.take().unwrap().take(),
        Err(_) => panic!("Couldn't retreive function frame"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
