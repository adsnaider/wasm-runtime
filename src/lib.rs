//! Runs a web assembly program.
//!
//! Some limitations:
//!   * Currently, the runtime can run a single wasm file (no imports).
//!   * Numeric instructions only, no memory, no tables. Maybe control instructions like branching,
//!   jumping.
//!   * Wasm output is returned.

use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::instr::{FBinop, FUnop, IBinop, IUnop, Instr, NumericInstr, Sign};
use wasm_parse::wasm::module::Module;
use wasm_parse::wasm::types::{NumType, ValType};

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

impl Execute for Instr {
    fn execute(&self, stack: &mut Vec<Val>, context: &Context) {
        match self {
            Instr::Numeric(x) => x.execute(stack, context),
            instr => unimplemented!("Can't execute instruction {:?}", instr),
        }
    }
}

impl Execute for NumericInstr {
    fn execute(&self, stack: &mut Vec<Val>, context: &Context) {
        match self {
            NumericInstr::I32Const(x) => stack.push(Val::I32(x.0)),
            NumericInstr::I64Const(x) => stack.push(Val::I64(x.0)),
            NumericInstr::F32Const(x) => stack.push(Val::F32(x.0)),
            NumericInstr::F64Const(x) => stack.push(Val::F64(x.0)),
            NumericInstr::IUnary(tpe, instr) => instr.execute(stack, context),
            //NumericInstr::FUnary(tpe, instr) => instr.execute(stack, context),
            NumericInstr::IBinary(tpe, instr) => instr.execute(stack, context),
            //NumericInstr::FBinary(tpe, instr) => instr.execute(stack, context),
            instr => unimplemented!("Unimplemented instruction: {:?}", instr),
        }
    }
}

impl Execute for IUnop {
    fn execute(&self, stack: &mut Vec<Val>, context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        match val {
            Val::I32(ref x) => match self {
                IUnop::Clz => stack.push(Val::I32(x.leading_zeros())),
                IUnop::Ctz => stack.push(Val::I32(x.trailing_zeros())),
                IUnop::Popcnt => stack.push(Val::I32(x.count_ones())),
            },
            Val::I64(ref x) => match self {
                IUnop::Clz => stack.push(Val::I32(x.leading_zeros())),
                IUnop::Ctz => stack.push(Val::I32(x.trailing_zeros())),
                IUnop::Popcnt => stack.push(Val::I32(x.count_ones())),
            },
            _ => panic!("Incorrect type for instruction!"),
        };
    }
}

impl Execute for IBinop {
    fn execute(&self, stack: &mut Vec<Val>, context: &Context) {
        let rhs = stack.pop().expect("Missing second operand!");
        let lhs = stack.pop().expect("Missing first operand!");
        match (lhs, rhs) {
            (Val::I32(lhs), Val::I32(rhs)) => match self {
                IBinop::Add => stack.push(Val::I32(lhs.wrapping_add(rhs))),
                IBinop::And => stack.push(Val::I32(lhs & rhs)),
                IBinop::Div(Sign::Signed) => stack.push(Val::I32((lhs as i32 / rhs as i32) as u32)),
                IBinop::Div(Sign::Unsigned) => stack.push(Val::I32(lhs / rhs)),
                IBinop::Mul => stack.push(Val::I32(lhs.wrapping_mul(rhs))),
                IBinop::Or => stack.push(Val::I32(lhs | rhs)),
                IBinop::Rem(Sign::Signed) => stack.push(Val::I32((lhs as i32 % rhs as i32) as u32)),
                IBinop::Rem(Sign::Unsigned) => stack.push(Val::I32(lhs % rhs)),
                IBinop::Rotl => stack.push(Val::I32(lhs.rotate_left(rhs % 32))),
                IBinop::Rotr => stack.push(Val::I32(lhs.rotate_right(rhs % 32))),
                IBinop::Shl => stack.push(Val::I32(lhs.wrapping_shl(rhs % 32))),
                IBinop::Shr(Sign::Signed) => {
                    stack.push(Val::I32((lhs as i32).wrapping_shr(rhs % 32) as u32))
                }
                IBinop::Shr(Sign::Unsigned) => stack.push(Val::I32(lhs.wrapping_shr(rhs % 32))),
                IBinop::Sub => stack.push(Val::I32(lhs.wrapping_sub(rhs))),
                IBinop::Xor => stack.push(Val::I32(lhs ^ rhs)),
            },
            (Val::I64(lhs), Val::I64(rhs)) => match self {
                IBinop::Add => stack.push(Val::I64(lhs.wrapping_add(rhs))),
                IBinop::And => stack.push(Val::I64(lhs & rhs)),
                IBinop::Div(Sign::Signed) => stack.push(Val::I64((lhs as i64 / rhs as i64) as u64)),
                IBinop::Div(Sign::Unsigned) => stack.push(Val::I64(lhs / rhs)),
                IBinop::Mul => stack.push(Val::I64(lhs.wrapping_mul(rhs))),
                IBinop::Or => stack.push(Val::I64(lhs | rhs)),
                IBinop::Rem(Sign::Signed) => stack.push(Val::I64((lhs as i64 % rhs as i64) as u64)),
                IBinop::Rem(Sign::Unsigned) => stack.push(Val::I64(lhs % rhs)),
                IBinop::Rotl => stack.push(Val::I64(lhs.rotate_left((rhs % 64) as u32))),
                IBinop::Rotr => stack.push(Val::I64(lhs.rotate_right((rhs % 64) as u32))),
                IBinop::Shl => stack.push(Val::I64(lhs.wrapping_shl((rhs % 64) as u32))),
                IBinop::Shr(Sign::Signed) => {
                    stack.push(Val::I64((lhs as i64).wrapping_shr((rhs % 64) as u32) as u64))
                }
                IBinop::Shr(Sign::Unsigned) => {
                    stack.push(Val::I64(lhs.wrapping_shr((rhs % 64) as u32) as u64))
                }
                IBinop::Sub => stack.push(Val::I64(lhs.wrapping_sub(rhs))),
                IBinop::Xor => stack.push(Val::I64(lhs ^ rhs)),
            },
            _ => panic!("Incorrect type for instruction!"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
