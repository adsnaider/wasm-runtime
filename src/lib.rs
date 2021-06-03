//! Runs a web assembly program.
//!
//! Some limitations:
//!   * Currently, the runtime can run a single wasm file (no imports).
//!   * Numeric instructions only, no memory, no tables. Maybe control instructions like branching,
//!   jumping.
//!   * Wasm output is returned.

use wasm_parse::wasm::func::Func;
use wasm_parse::wasm::instr::{
    FBinop, FRelop, FUnop, FloatType, IBinop, IRelop, ITestop, IUnop, Instr, IntType, NumericInstr,
};
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
            NumericInstr::I32Unary(instr) => instr.execute(stack, context),
            NumericInstr::I64Unary(instr) => instr.execute(stack, context),
            NumericInstr::F32Unary(instr) => instr.execute(stack, context),
            NumericInstr::F64Unary(instr) => instr.execute(stack, context),
            NumericInstr::I32Binary(instr) => instr.execute(stack, context),
            NumericInstr::I64Binary(instr) => instr.execute(stack, context),
            NumericInstr::F32Binary(instr) => instr.execute(stack, context),
            NumericInstr::F64Binary(instr) => instr.execute(stack, context),
            NumericInstr::I32Test(instr) => instr.execute(stack, context),
            NumericInstr::I64Test(instr) => instr.execute(stack, context),
            NumericInstr::I32Relop(instr) => instr.execute(stack, context),
            NumericInstr::I64Relop(instr) => instr.execute(stack, context),
            NumericInstr::F32Relop(instr) => instr.execute(stack, context),
            NumericInstr::F64Relop(instr) => instr.execute(stack, context),
            NumericInstr::I32Extend8S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64Extend8S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32Extend16S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64Extend16S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64Extend32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32WrapI64 => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64ExtendI32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64ExtendI32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncF32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncF32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncF32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncF32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncF64S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncF64U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncF64S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncF64U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncSatF32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncSatF32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncSatF64S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32TruncSatF64U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncSatF32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncSatF32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncSatF64S => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64TruncSatF64U => unimplemented!("Unimplemented instruction"),
            NumericInstr::F32DemoteF64 => unimplemented!("Unimplemented instruction"),
            NumericInstr::F64PromoteF32 => unimplemented!("Unimplemented instruction"),
            NumericInstr::F32ConvertI32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::F32ConvertI32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::F32ConvertI64S => unimplemented!("Unimplemented instruction"),
            NumericInstr::F32ConvertI64U => unimplemented!("Unimplemented instruction"),
            NumericInstr::F64ConvertI32S => unimplemented!("Unimplemented instruction"),
            NumericInstr::F64ConvertI32U => unimplemented!("Unimplemented instruction"),
            NumericInstr::F64ConvertI64S => unimplemented!("Unimplemented instruction"),
            NumericInstr::F64ConvertI64U => unimplemented!("Unimplemented instruction"),
            NumericInstr::I32ReinterpretF32 => unimplemented!("Unimplemented instruction"),
            NumericInstr::I64ReinterpretF64 => unimplemented!("Unimplemented instruction"),
            NumericInstr::F32ReinterpretI32 => unimplemented!("Unimplemented instruction"),
            NumericInstr::F64ReinterpretI64 => unimplemented!("Unimplemented instruction"),
        }
    }
}

impl Execute for ITestop<{ IntType::I32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        let val = match val {
            Val::I32(x) => x,
            _ => panic!("Incorrect type!"),
        };

        match self {
            ITestop::Eqz => stack.push(Val::I32((val == 0) as u32)),
        }
    }
}

impl Execute for ITestop<{ IntType::I64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        let val = match val {
            Val::I64(x) => x,
            _ => panic!("Incorrect type!"),
        };

        match self {
            ITestop::Eqz => stack.push(Val::I32((val == 0) as u32)),
        }
    }
}

impl Execute for IRelop<{ IntType::I32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing operand!");
        let lhs = stack.pop().expect("Missing operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::I32(lhs), Val::I32(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            IRelop::Equ => stack.push(Val::I32((lhs == rhs) as u32)),
            IRelop::GeS => stack.push(Val::I32((lhs as i32 >= rhs as i32) as u32)),
            IRelop::GeU => stack.push(Val::I32((lhs >= rhs) as u32)),
            IRelop::LeS => stack.push(Val::I32((lhs as i32 <= rhs as i32) as u32)),
            IRelop::LeU => stack.push(Val::I32((lhs <= rhs) as u32)),
            IRelop::GtS => stack.push(Val::I32((lhs as i32 > rhs as i32) as u32)),
            IRelop::GtU => stack.push(Val::I32((lhs > rhs) as u32)),
            IRelop::LtS => stack.push(Val::I32(((lhs as i32) < (rhs as i32)) as u32)),
            IRelop::LtU => stack.push(Val::I32((lhs < rhs) as u32)),
            IRelop::Ne => stack.push(Val::I32((lhs != rhs) as u32)),
        }
    }
}
impl Execute for IRelop<{ IntType::I64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing operand!");
        let lhs = stack.pop().expect("Missing operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::I64(lhs), Val::I64(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            IRelop::Equ => stack.push(Val::I32((lhs == rhs) as u32)),
            IRelop::GeS => stack.push(Val::I32((lhs as i64 >= rhs as i64) as u32)),
            IRelop::GeU => stack.push(Val::I32((lhs >= rhs) as u32)),
            IRelop::LeS => stack.push(Val::I32((lhs as i64 <= rhs as i64) as u32)),
            IRelop::LeU => stack.push(Val::I32((lhs <= rhs) as u32)),
            IRelop::GtS => stack.push(Val::I32((lhs as i64 > rhs as i64) as u32)),
            IRelop::GtU => stack.push(Val::I32((lhs > rhs) as u32)),
            IRelop::LtS => stack.push(Val::I32(((lhs as i64) < (rhs as i64)) as u32)),
            IRelop::LtU => stack.push(Val::I32((lhs < rhs) as u32)),
            IRelop::Ne => stack.push(Val::I32((lhs != rhs) as u32)),
        }
    }
}
impl Execute for FRelop<{ FloatType::F32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing operand!");
        let lhs = stack.pop().expect("Missing operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::F32(lhs), Val::F32(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            FRelop::Equ => stack.push(Val::I32((lhs == rhs) as u32)),
            FRelop::Ge => stack.push(Val::I32((lhs >= rhs) as u32)),
            FRelop::Gt => stack.push(Val::I32((lhs > rhs) as u32)),
            FRelop::Le => stack.push(Val::I32((lhs <= rhs) as u32)),
            FRelop::Lt => stack.push(Val::I32((lhs < rhs) as u32)),
            FRelop::Ne => stack.push(Val::I32((lhs != rhs) as u32)),
        }
    }
}
impl Execute for FRelop<{ FloatType::F64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing operand!");
        let lhs = stack.pop().expect("Missing operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::F64(lhs), Val::F64(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            FRelop::Equ => stack.push(Val::I32((lhs == rhs) as u32)),
            FRelop::Ge => stack.push(Val::I32((lhs >= rhs) as u32)),
            FRelop::Gt => stack.push(Val::I32((lhs > rhs) as u32)),
            FRelop::Le => stack.push(Val::I32((lhs <= rhs) as u32)),
            FRelop::Lt => stack.push(Val::I32((lhs < rhs) as u32)),
            FRelop::Ne => stack.push(Val::I32((lhs != rhs) as u32)),
        }
    }
}

impl Execute for FUnop<{ FloatType::F32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        let val = match val {
            Val::F32(x) => x,
            _ => panic!("Incorrect type!"),
        };

        match self {
            FUnop::Abs => stack.push(Val::F32(val.abs())),
            FUnop::Floor => stack.push(Val::F32(val.floor())),
            FUnop::Ceil => stack.push(Val::F32(val.ceil())),
            FUnop::Nearest => stack.push(Val::F32(val.round())),
            FUnop::Neg => stack.push(Val::F32(-val)),
            FUnop::Sqrt => stack.push(Val::F32(val.sqrt())),
            FUnop::Trunc => stack.push(Val::F32(val.trunc())),
        }
    }
}
impl Execute for FUnop<{ FloatType::F64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        let val = match val {
            Val::F64(x) => x,
            _ => panic!("Incorrect type!"),
        };

        match self {
            FUnop::Abs => stack.push(Val::F64(val.abs())),
            FUnop::Floor => stack.push(Val::F64(val.floor())),
            FUnop::Ceil => stack.push(Val::F64(val.ceil())),
            FUnop::Nearest => stack.push(Val::F64(val.round())),
            FUnop::Neg => stack.push(Val::F64(-val)),
            FUnop::Sqrt => stack.push(Val::F64(val.sqrt())),
            FUnop::Trunc => stack.push(Val::F64(val.trunc())),
        }
    }
}
impl Execute for FBinop<{ FloatType::F32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing operand!");
        let lhs = stack.pop().expect("Missing operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::F32(lhs), Val::F32(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            FBinop::Add => stack.push(Val::F32(lhs + rhs)),
            FBinop::CopySign => stack.push(Val::F32(lhs.copysign(rhs))),
            FBinop::Div => stack.push(Val::F32(lhs / rhs)),
            // TODO: Watchout for NaN propagation.
            FBinop::Max => stack.push(Val::F32(lhs.max(rhs))),
            FBinop::Min => stack.push(Val::F32(lhs.min(rhs))),
            FBinop::Mul => stack.push(Val::F32(lhs * rhs)),
            FBinop::Sub => stack.push(Val::F32(lhs - rhs)),
        }
    }
}
impl Execute for FBinop<{ FloatType::F64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing operand!");
        let lhs = stack.pop().expect("Missing operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::F64(lhs), Val::F64(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            FBinop::Add => stack.push(Val::F64(lhs + rhs)),
            FBinop::CopySign => stack.push(Val::F64(lhs.copysign(rhs))),
            FBinop::Div => stack.push(Val::F64(lhs / rhs)),
            // TODO: Watchout for NaN propagation.
            FBinop::Max => stack.push(Val::F64(lhs.max(rhs))),
            FBinop::Min => stack.push(Val::F64(lhs.min(rhs))),
            FBinop::Mul => stack.push(Val::F64(lhs * rhs)),
            FBinop::Sub => stack.push(Val::F64(lhs - rhs)),
        }
    }
}

impl Execute for IUnop<{ IntType::I32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        let val = match val {
            Val::I32(x) => x,
            _ => panic!("Incorrect type!"),
        };
        match self {
            IUnop::Clz => stack.push(Val::I32(val.leading_zeros())),
            IUnop::Ctz => stack.push(Val::I32(val.trailing_zeros())),
            IUnop::Popcnt => stack.push(Val::I32(val.count_ones())),
        };
    }
}

impl Execute for IUnop<{ IntType::I64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let val = stack.pop().expect("Missing operand!");
        let val = match val {
            Val::I64(x) => x,
            _ => panic!("Incorrect type!"),
        };
        match self {
            IUnop::Clz => stack.push(Val::I32(val.leading_zeros())),
            IUnop::Ctz => stack.push(Val::I32(val.trailing_zeros())),
            IUnop::Popcnt => stack.push(Val::I32(val.count_ones())),
        };
    }
}

impl Execute for IBinop<{ IntType::I32 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing second operand!");
        let lhs = stack.pop().expect("Missing first operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::I32(lhs), Val::I32(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };

        match self {
            IBinop::Add => stack.push(Val::I32(lhs.wrapping_add(rhs))),
            IBinop::And => stack.push(Val::I32(lhs & rhs)),
            IBinop::DivS => stack.push(Val::I32((lhs as i32 / rhs as i32) as u32)),
            IBinop::DivU => stack.push(Val::I32(lhs / rhs)),
            IBinop::Mul => stack.push(Val::I32(lhs.wrapping_mul(rhs))),
            IBinop::Or => stack.push(Val::I32(lhs | rhs)),
            IBinop::RemS => stack.push(Val::I32((lhs as i32 % rhs as i32) as u32)),
            IBinop::RemU => stack.push(Val::I32(lhs % rhs)),
            IBinop::Rotl => stack.push(Val::I32(lhs.rotate_left(rhs % 32))),
            IBinop::Rotr => stack.push(Val::I32(lhs.rotate_right(rhs % 32))),
            IBinop::Shl => stack.push(Val::I32(lhs.wrapping_shl(rhs % 32))),
            IBinop::ShrS => stack.push(Val::I32((lhs as i32).wrapping_shr(rhs % 32) as u32)),
            IBinop::ShrU => stack.push(Val::I32(lhs.wrapping_shr(rhs % 32))),
            IBinop::Sub => stack.push(Val::I32(lhs.wrapping_sub(rhs))),
            IBinop::Xor => stack.push(Val::I32(lhs ^ rhs)),
        }
    }
}

impl Execute for IBinop<{ IntType::I64 }> {
    fn execute(&self, stack: &mut Vec<Val>, _context: &Context) {
        let rhs = stack.pop().expect("Missing second operand!");
        let lhs = stack.pop().expect("Missing first operand!");
        let (lhs, rhs) = match (lhs, rhs) {
            (Val::I64(lhs), Val::I64(rhs)) => (lhs, rhs),
            _ => panic!("Incorrect type!"),
        };
        match self {
            IBinop::Add => stack.push(Val::I64(lhs.wrapping_add(rhs))),
            IBinop::And => stack.push(Val::I64(lhs & rhs)),
            IBinop::DivS => stack.push(Val::I64((lhs as i64 / rhs as i64) as u64)),
            IBinop::DivU => stack.push(Val::I64(lhs / rhs)),
            IBinop::Mul => stack.push(Val::I64(lhs.wrapping_mul(rhs))),
            IBinop::Or => stack.push(Val::I64(lhs | rhs)),
            IBinop::RemS => stack.push(Val::I64((lhs as i64 % rhs as i64) as u64)),
            IBinop::RemU => stack.push(Val::I64(lhs % rhs)),
            IBinop::Rotl => stack.push(Val::I64(lhs.rotate_left((rhs % 64) as u32))),
            IBinop::Rotr => stack.push(Val::I64(lhs.rotate_right((rhs % 64) as u32))),
            IBinop::Shl => stack.push(Val::I64(lhs.wrapping_shl((rhs % 64) as u32))),
            IBinop::ShrS => {
                stack.push(Val::I64((lhs as i64).wrapping_shr((rhs % 64) as u32) as u64))
            }
            IBinop::ShrU => stack.push(Val::I64(lhs.wrapping_shr((rhs % 64) as u32) as u64)),
            IBinop::Sub => stack.push(Val::I64(lhs.wrapping_sub(rhs))),
            IBinop::Xor => stack.push(Val::I64(lhs ^ rhs)),
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
