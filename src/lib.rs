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

pub use structures::Val;
use wasm_parse::wasm::export::ExportDesc;
use wasm_parse::wasm::module::Module;

use crate::structures::{ExecutionResult, Frame};

#[derive(Debug)]
pub enum RuntimeError {
    Trap,
    UnknownRuntimeError,
}

pub fn execute_module(module: &Module, inputs: Vec<Val>) -> Result<Vec<Val>, RuntimeError> {
    let (frame, executor) = Frame::from_index(
        module,
        module
            .start
            .as_ref()
            .expect("Module must have start function!")
            .func,
        inputs,
    );
    match executor.execute() {
        ExecutionResult::Continue | ExecutionResult::Return => {}
        ExecutionResult::Trap => return Err(RuntimeError::Trap),
        ExecutionResult::BranchTo(_) => return Err(RuntimeError::UnknownRuntimeError),
    }
    match Rc::try_unwrap(frame) {
        Ok(frame) => Ok(frame.take().unwrap().take()),
        Err(_) => panic!("Couldn't retreive function frame"),
    }
}

pub fn execute_function(
    module: &Module,
    name: &str,
    inputs: Vec<Val>,
) -> Result<Vec<Val>, RuntimeError> {
    let mut func = None;
    for export in &module.exports {
        if export.name.name == name {
            match export.desc {
                ExportDesc::Func(idx) => func = Some(idx),
                _ => panic!("Function export should have function type."),
            }
        }
    }
    let func = func.expect(&format!("No function with name: {}", name));
    let (frame, executor) = Frame::from_index(module, func, inputs);
    match executor.execute() {
        ExecutionResult::Continue | ExecutionResult::Return => {}
        ExecutionResult::Trap => return Err(RuntimeError::Trap),
        ExecutionResult::BranchTo(_) => return Err(RuntimeError::UnknownRuntimeError),
    }
    match Rc::try_unwrap(frame) {
        Ok(frame) => Ok(frame.take().unwrap().take()),
        Err(_) => panic!("Couldn't retreive function frame"),
    }
}
