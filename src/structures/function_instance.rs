use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::instr::Instr;
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::{NumType, RefType, ValType};

use super::{Execute, Frame, Label};
use crate::stack::{Stack, Val};

#[derive(Clone)]
pub struct FunctionInstance<'a> {
    frame: Frame<'a>,
    body: &'a Vec<Instr>,
}

impl Execute for FunctionInstance<'_> {
    fn execute(&self, stack: &mut Stack, frame: &mut Frame) {
        self.body.first().unwrap().execute(stack, frame);
    }
}

impl<'a> FunctionInstance<'a> {
    pub fn new(func: &'a Func, module: &'a Module) -> FunctionInstance<'a> {
        // TODO: Don't forget about args as locals.
        let locals = Some(
            func.locals
                .iter()
                .map(|tpe| match tpe {
                    ValType::Num(NumType::I32) => Val::I32(0),
                    ValType::Num(NumType::I64) => Val::I64(0),
                    ValType::Num(NumType::F32) => Val::F32(0.0),
                    ValType::Num(NumType::F64) => Val::F64(0.0),
                    ValType::Ref(RefType::FuncRef) => Val::Ref(None),
                    ValType::Ref(RefType::ExternRef) => panic!("Extern ref not yet supported"),
                })
                .collect(),
        );

        let frame = Frame {
            module,
            locals,
            labels: vec![Label {
                branch: None,
                fallthrough: None,
            }],
            lpc: 0,
        };

        let body = &func.body.instr;

        FunctionInstance { frame, body }
    }
}
