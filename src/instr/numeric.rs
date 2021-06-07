use wasm_parse::wasm::instr::{
    FBinop, FRelop, FUnop, FloatType, IBinop, IRelop, ITestop, IUnop, IntType, NumericInstr,
};

use crate::safe_pop;
use crate::structures::{Execute, ExecutionResult, Frame, Val};

impl Execute for NumericInstr {
    fn execute<'a>(&'a self, frame: &mut Frame<'a>) -> ExecutionResult {
        match self {
            NumericInstr::I32Const(x) => {
                frame.push_value(Val::I32(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::I64Const(x) => {
                frame.push_value(Val::I64(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::F32Const(x) => {
                frame.push_value(Val::F32(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::F64Const(x) => {
                frame.push_value(Val::F64(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::I32Unary(instr) => instr.execute(frame),
            NumericInstr::I64Unary(instr) => instr.execute(frame),
            NumericInstr::F32Unary(instr) => instr.execute(frame),
            NumericInstr::F64Unary(instr) => instr.execute(frame),
            NumericInstr::I32Binary(instr) => instr.execute(frame),
            NumericInstr::I64Binary(instr) => instr.execute(frame),
            NumericInstr::F32Binary(instr) => instr.execute(frame),
            NumericInstr::F64Binary(instr) => instr.execute(frame),
            NumericInstr::I32Test(instr) => instr.execute(frame),
            NumericInstr::I64Test(instr) => instr.execute(frame),
            NumericInstr::I32Relop(instr) => instr.execute(frame),
            NumericInstr::I64Relop(instr) => instr.execute(frame),
            NumericInstr::F32Relop(instr) => instr.execute(frame),
            NumericInstr::F64Relop(instr) => instr.execute(frame),
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
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let val = safe_pop!(frame, Val::I32);
        match self {
            ITestop::Eqz => frame.push_value(Val::I32((val == 0) as u32)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for ITestop<{ IntType::I64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let val = safe_pop!(frame, Val::I64);
        match self {
            ITestop::Eqz => frame.push_value(Val::I32((val == 0) as u32)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for IRelop<{ IntType::I32 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::I32, Val::I32);

        match self {
            IRelop::Equ => frame.push_value(Val::I32((lhs == rhs) as u32)),
            IRelop::GeS => frame.push_value(Val::I32((lhs as i32 >= rhs as i32) as u32)),
            IRelop::GeU => frame.push_value(Val::I32((lhs >= rhs) as u32)),
            IRelop::LeS => frame.push_value(Val::I32((lhs as i32 <= rhs as i32) as u32)),
            IRelop::LeU => frame.push_value(Val::I32((lhs <= rhs) as u32)),
            IRelop::GtS => frame.push_value(Val::I32((lhs as i32 > rhs as i32) as u32)),
            IRelop::GtU => frame.push_value(Val::I32((lhs > rhs) as u32)),
            IRelop::LtS => frame.push_value(Val::I32(((lhs as i32) < (rhs as i32)) as u32)),
            IRelop::LtU => frame.push_value(Val::I32((lhs < rhs) as u32)),
            IRelop::Ne => frame.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for IRelop<{ IntType::I64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::I64, Val::I64);

        match self {
            IRelop::Equ => frame.push_value(Val::I32((lhs == rhs) as u32)),
            IRelop::GeS => frame.push_value(Val::I32((lhs as i64 >= rhs as i64) as u32)),
            IRelop::GeU => frame.push_value(Val::I32((lhs >= rhs) as u32)),
            IRelop::LeS => frame.push_value(Val::I32((lhs as i64 <= rhs as i64) as u32)),
            IRelop::LeU => frame.push_value(Val::I32((lhs <= rhs) as u32)),
            IRelop::GtS => frame.push_value(Val::I32((lhs as i64 > rhs as i64) as u32)),
            IRelop::GtU => frame.push_value(Val::I32((lhs > rhs) as u32)),
            IRelop::LtS => frame.push_value(Val::I32(((lhs as i64) < (rhs as i64)) as u32)),
            IRelop::LtU => frame.push_value(Val::I32((lhs < rhs) as u32)),
            IRelop::Ne => frame.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FRelop<{ FloatType::F32 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::F32, Val::F32);

        match self {
            FRelop::Equ => frame.push_value(Val::I32((lhs == rhs) as u32)),
            FRelop::Ge => frame.push_value(Val::I32((lhs >= rhs) as u32)),
            FRelop::Gt => frame.push_value(Val::I32((lhs > rhs) as u32)),
            FRelop::Le => frame.push_value(Val::I32((lhs <= rhs) as u32)),
            FRelop::Lt => frame.push_value(Val::I32((lhs < rhs) as u32)),
            FRelop::Ne => frame.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FRelop<{ FloatType::F64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::F64, Val::F64);

        match self {
            FRelop::Equ => frame.push_value(Val::I32((lhs == rhs) as u32)),
            FRelop::Ge => frame.push_value(Val::I32((lhs >= rhs) as u32)),
            FRelop::Gt => frame.push_value(Val::I32((lhs > rhs) as u32)),
            FRelop::Le => frame.push_value(Val::I32((lhs <= rhs) as u32)),
            FRelop::Lt => frame.push_value(Val::I32((lhs < rhs) as u32)),
            FRelop::Ne => frame.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for FUnop<{ FloatType::F32 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let val = safe_pop!(frame, Val::F32);

        match self {
            FUnop::Abs => frame.push_value(Val::F32(val.abs())),
            FUnop::Floor => frame.push_value(Val::F32(val.floor())),
            FUnop::Ceil => frame.push_value(Val::F32(val.ceil())),
            FUnop::Nearest => frame.push_value(Val::F32(val.round())),
            FUnop::Neg => frame.push_value(Val::F32(-val)),
            FUnop::Sqrt => frame.push_value(Val::F32(val.sqrt())),
            FUnop::Trunc => frame.push_value(Val::F32(val.trunc())),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FUnop<{ FloatType::F64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let val = safe_pop!(frame, Val::F64);

        match self {
            FUnop::Abs => frame.push_value(Val::F64(val.abs())),
            FUnop::Floor => frame.push_value(Val::F64(val.floor())),
            FUnop::Ceil => frame.push_value(Val::F64(val.ceil())),
            FUnop::Nearest => frame.push_value(Val::F64(val.round())),
            FUnop::Neg => frame.push_value(Val::F64(-val)),
            FUnop::Sqrt => frame.push_value(Val::F64(val.sqrt())),
            FUnop::Trunc => frame.push_value(Val::F64(val.trunc())),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FBinop<{ FloatType::F32 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::F32, Val::F32);

        match self {
            FBinop::Add => frame.push_value(Val::F32(lhs + rhs)),
            FBinop::CopySign => frame.push_value(Val::F32(lhs.copysign(rhs))),
            FBinop::Div => frame.push_value(Val::F32(lhs / rhs)),
            // TODO: Watchout for NaN propagation.
            FBinop::Max => frame.push_value(Val::F32(lhs.max(rhs))),
            FBinop::Min => frame.push_value(Val::F32(lhs.min(rhs))),
            FBinop::Mul => frame.push_value(Val::F32(lhs * rhs)),
            FBinop::Sub => frame.push_value(Val::F32(lhs - rhs)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FBinop<{ FloatType::F64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::F64, Val::F64);

        match self {
            FBinop::Add => frame.push_value(Val::F64(lhs + rhs)),
            FBinop::CopySign => frame.push_value(Val::F64(lhs.copysign(rhs))),
            FBinop::Div => frame.push_value(Val::F64(lhs / rhs)),
            // TODO: Watchout for NaN propagation.
            FBinop::Max => frame.push_value(Val::F64(lhs.max(rhs))),
            FBinop::Min => frame.push_value(Val::F64(lhs.min(rhs))),
            FBinop::Mul => frame.push_value(Val::F64(lhs * rhs)),
            FBinop::Sub => frame.push_value(Val::F64(lhs - rhs)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for IUnop<{ IntType::I32 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let val = safe_pop!(frame, Val::I32);

        match self {
            IUnop::Clz => frame.push_value(Val::I32(val.leading_zeros())),
            IUnop::Ctz => frame.push_value(Val::I32(val.trailing_zeros())),
            IUnop::Popcnt => frame.push_value(Val::I32(val.count_ones())),
        };
        ExecutionResult::Continue
    }
}

impl Execute for IUnop<{ IntType::I64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let val = safe_pop!(frame, Val::I64);
        match self {
            IUnop::Clz => frame.push_value(Val::I32(val.leading_zeros())),
            IUnop::Ctz => frame.push_value(Val::I32(val.trailing_zeros())),
            IUnop::Popcnt => frame.push_value(Val::I32(val.count_ones())),
        };
        ExecutionResult::Continue
    }
}

impl Execute for IBinop<{ IntType::I32 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::I32, Val::I32);

        match self {
            IBinop::Add => frame.push_value(Val::I32(lhs.wrapping_add(rhs))),
            IBinop::And => frame.push_value(Val::I32(lhs & rhs)),
            IBinop::DivS => frame.push_value(Val::I32((lhs as i32 / rhs as i32) as u32)),
            IBinop::DivU => frame.push_value(Val::I32(lhs / rhs)),
            IBinop::Mul => frame.push_value(Val::I32(lhs.wrapping_mul(rhs))),
            IBinop::Or => frame.push_value(Val::I32(lhs | rhs)),
            IBinop::RemS => frame.push_value(Val::I32((lhs as i32 % rhs as i32) as u32)),
            IBinop::RemU => frame.push_value(Val::I32(lhs % rhs)),
            IBinop::Rotl => frame.push_value(Val::I32(lhs.rotate_left(rhs % 32))),
            IBinop::Rotr => frame.push_value(Val::I32(lhs.rotate_right(rhs % 32))),
            IBinop::Shl => frame.push_value(Val::I32(lhs.wrapping_shl(rhs % 32))),
            IBinop::ShrS => frame.push_value(Val::I32((lhs as i32).wrapping_shr(rhs % 32) as u32)),
            IBinop::ShrU => frame.push_value(Val::I32(lhs.wrapping_shr(rhs % 32))),
            IBinop::Sub => frame.push_value(Val::I32(lhs.wrapping_sub(rhs))),
            IBinop::Xor => frame.push_value(Val::I32(lhs ^ rhs)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for IBinop<{ IntType::I64 }> {
    fn execute(&self, frame: &mut Frame) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(frame, Val::I64, Val::I64);
        match self {
            IBinop::Add => frame.push_value(Val::I64(lhs.wrapping_add(rhs))),
            IBinop::And => frame.push_value(Val::I64(lhs & rhs)),
            IBinop::DivS => frame.push_value(Val::I64((lhs as i64 / rhs as i64) as u64)),
            IBinop::DivU => frame.push_value(Val::I64(lhs / rhs)),
            IBinop::Mul => frame.push_value(Val::I64(lhs.wrapping_mul(rhs))),
            IBinop::Or => frame.push_value(Val::I64(lhs | rhs)),
            IBinop::RemS => frame.push_value(Val::I64((lhs as i64 % rhs as i64) as u64)),
            IBinop::RemU => frame.push_value(Val::I64(lhs % rhs)),
            IBinop::Rotl => frame.push_value(Val::I64(lhs.rotate_left((rhs % 64) as u32))),
            IBinop::Rotr => frame.push_value(Val::I64(lhs.rotate_right((rhs % 64) as u32))),
            IBinop::Shl => frame.push_value(Val::I64(lhs.wrapping_shl((rhs % 64) as u32))),
            IBinop::ShrS => {
                frame.push_value(Val::I64((lhs as i64).wrapping_shr((rhs % 64) as u32) as u64))
            }
            IBinop::ShrU => frame.push_value(Val::I64(lhs.wrapping_shr((rhs % 64) as u32) as u64)),
            IBinop::Sub => frame.push_value(Val::I64(lhs.wrapping_sub(rhs))),
            IBinop::Xor => frame.push_value(Val::I64(lhs ^ rhs)),
        }
        ExecutionResult::Continue
    }
}
