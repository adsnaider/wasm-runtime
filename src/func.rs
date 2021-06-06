use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::instr::{ControlInstr, Instr};
use wasm_parse::wasm::types::{NumType, RefType, ValType};

use crate::stack::Stack;
use crate::structures::function_runtime::FunctionRuntime;
