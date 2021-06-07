use wasm_parse::wasm::instr::{
    FBinop, FRelop, FUnop, FloatType, IBinop, IRelop, ITestop, IUnop, IntType, NumericInstr,
};

use crate::safe_pop;
use crate::structures::{Execute, ExecutionResult, Executor, Val};

impl Execute for NumericInstr {
    fn execute<'a>(&'a self, executor: &mut Executor<'a>) -> ExecutionResult {
        match self {
            NumericInstr::I32Const(x) => {
                executor.push_value(Val::I32(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::I64Const(x) => {
                executor.push_value(Val::I64(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::F32Const(x) => {
                executor.push_value(Val::F32(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::F64Const(x) => {
                executor.push_value(Val::F64(x.0));
                ExecutionResult::Continue
            }
            NumericInstr::I32Unary(instr) => instr.execute(executor),
            NumericInstr::I64Unary(instr) => instr.execute(executor),
            NumericInstr::F32Unary(instr) => instr.execute(executor),
            NumericInstr::F64Unary(instr) => instr.execute(executor),
            NumericInstr::I32Binary(instr) => instr.execute(executor),
            NumericInstr::I64Binary(instr) => instr.execute(executor),
            NumericInstr::F32Binary(instr) => instr.execute(executor),
            NumericInstr::F64Binary(instr) => instr.execute(executor),
            NumericInstr::I32Test(instr) => instr.execute(executor),
            NumericInstr::I64Test(instr) => instr.execute(executor),
            NumericInstr::I32Relop(instr) => instr.execute(executor),
            NumericInstr::I64Relop(instr) => instr.execute(executor),
            NumericInstr::F32Relop(instr) => instr.execute(executor),
            NumericInstr::F64Relop(instr) => instr.execute(executor),
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
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let val = safe_pop!(executor, Val::I32);
        match self {
            ITestop::Eqz => executor.push_value(Val::I32((val == 0) as u32)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for ITestop<{ IntType::I64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let val = safe_pop!(executor, Val::I64);
        match self {
            ITestop::Eqz => executor.push_value(Val::I32((val == 0) as u32)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for IRelop<{ IntType::I32 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::I32, Val::I32);

        match self {
            IRelop::Equ => executor.push_value(Val::I32((lhs == rhs) as u32)),
            IRelop::GeS => executor.push_value(Val::I32((lhs as i32 >= rhs as i32) as u32)),
            IRelop::GeU => executor.push_value(Val::I32((lhs >= rhs) as u32)),
            IRelop::LeS => executor.push_value(Val::I32((lhs as i32 <= rhs as i32) as u32)),
            IRelop::LeU => executor.push_value(Val::I32((lhs <= rhs) as u32)),
            IRelop::GtS => executor.push_value(Val::I32((lhs as i32 > rhs as i32) as u32)),
            IRelop::GtU => executor.push_value(Val::I32((lhs > rhs) as u32)),
            IRelop::LtS => executor.push_value(Val::I32(((lhs as i32) < (rhs as i32)) as u32)),
            IRelop::LtU => executor.push_value(Val::I32((lhs < rhs) as u32)),
            IRelop::Ne => executor.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for IRelop<{ IntType::I64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::I64, Val::I64);

        match self {
            IRelop::Equ => executor.push_value(Val::I32((lhs == rhs) as u32)),
            IRelop::GeS => executor.push_value(Val::I32((lhs as i64 >= rhs as i64) as u32)),
            IRelop::GeU => executor.push_value(Val::I32((lhs >= rhs) as u32)),
            IRelop::LeS => executor.push_value(Val::I32((lhs as i64 <= rhs as i64) as u32)),
            IRelop::LeU => executor.push_value(Val::I32((lhs <= rhs) as u32)),
            IRelop::GtS => executor.push_value(Val::I32((lhs as i64 > rhs as i64) as u32)),
            IRelop::GtU => executor.push_value(Val::I32((lhs > rhs) as u32)),
            IRelop::LtS => executor.push_value(Val::I32(((lhs as i64) < (rhs as i64)) as u32)),
            IRelop::LtU => executor.push_value(Val::I32((lhs < rhs) as u32)),
            IRelop::Ne => executor.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FRelop<{ FloatType::F32 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::F32, Val::F32);

        match self {
            FRelop::Equ => executor.push_value(Val::I32((lhs == rhs) as u32)),
            FRelop::Ge => executor.push_value(Val::I32((lhs >= rhs) as u32)),
            FRelop::Gt => executor.push_value(Val::I32((lhs > rhs) as u32)),
            FRelop::Le => executor.push_value(Val::I32((lhs <= rhs) as u32)),
            FRelop::Lt => executor.push_value(Val::I32((lhs < rhs) as u32)),
            FRelop::Ne => executor.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FRelop<{ FloatType::F64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::F64, Val::F64);

        match self {
            FRelop::Equ => executor.push_value(Val::I32((lhs == rhs) as u32)),
            FRelop::Ge => executor.push_value(Val::I32((lhs >= rhs) as u32)),
            FRelop::Gt => executor.push_value(Val::I32((lhs > rhs) as u32)),
            FRelop::Le => executor.push_value(Val::I32((lhs <= rhs) as u32)),
            FRelop::Lt => executor.push_value(Val::I32((lhs < rhs) as u32)),
            FRelop::Ne => executor.push_value(Val::I32((lhs != rhs) as u32)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for FUnop<{ FloatType::F32 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let val = safe_pop!(executor, Val::F32);

        match self {
            FUnop::Abs => executor.push_value(Val::F32(val.abs())),
            FUnop::Floor => executor.push_value(Val::F32(val.floor())),
            FUnop::Ceil => executor.push_value(Val::F32(val.ceil())),
            FUnop::Nearest => executor.push_value(Val::F32(val.round())),
            FUnop::Neg => executor.push_value(Val::F32(-val)),
            FUnop::Sqrt => executor.push_value(Val::F32(val.sqrt())),
            FUnop::Trunc => executor.push_value(Val::F32(val.trunc())),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FUnop<{ FloatType::F64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let val = safe_pop!(executor, Val::F64);

        match self {
            FUnop::Abs => executor.push_value(Val::F64(val.abs())),
            FUnop::Floor => executor.push_value(Val::F64(val.floor())),
            FUnop::Ceil => executor.push_value(Val::F64(val.ceil())),
            FUnop::Nearest => executor.push_value(Val::F64(val.round())),
            FUnop::Neg => executor.push_value(Val::F64(-val)),
            FUnop::Sqrt => executor.push_value(Val::F64(val.sqrt())),
            FUnop::Trunc => executor.push_value(Val::F64(val.trunc())),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FBinop<{ FloatType::F32 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::F32, Val::F32);

        match self {
            FBinop::Add => executor.push_value(Val::F32(lhs + rhs)),
            FBinop::CopySign => executor.push_value(Val::F32(lhs.copysign(rhs))),
            FBinop::Div => executor.push_value(Val::F32(lhs / rhs)),
            // TODO: Watchout for NaN propagation.
            FBinop::Max => executor.push_value(Val::F32(lhs.max(rhs))),
            FBinop::Min => executor.push_value(Val::F32(lhs.min(rhs))),
            FBinop::Mul => executor.push_value(Val::F32(lhs * rhs)),
            FBinop::Sub => executor.push_value(Val::F32(lhs - rhs)),
        }
        ExecutionResult::Continue
    }
}
impl Execute for FBinop<{ FloatType::F64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::F64, Val::F64);

        match self {
            FBinop::Add => executor.push_value(Val::F64(lhs + rhs)),
            FBinop::CopySign => executor.push_value(Val::F64(lhs.copysign(rhs))),
            FBinop::Div => executor.push_value(Val::F64(lhs / rhs)),
            // TODO: Watchout for NaN propagation.
            FBinop::Max => executor.push_value(Val::F64(lhs.max(rhs))),
            FBinop::Min => executor.push_value(Val::F64(lhs.min(rhs))),
            FBinop::Mul => executor.push_value(Val::F64(lhs * rhs)),
            FBinop::Sub => executor.push_value(Val::F64(lhs - rhs)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for IUnop<{ IntType::I32 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let val = safe_pop!(executor, Val::I32);

        match self {
            IUnop::Clz => executor.push_value(Val::I32(val.leading_zeros())),
            IUnop::Ctz => executor.push_value(Val::I32(val.trailing_zeros())),
            IUnop::Popcnt => executor.push_value(Val::I32(val.count_ones())),
        };
        ExecutionResult::Continue
    }
}

impl Execute for IUnop<{ IntType::I64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let val = safe_pop!(executor, Val::I64);
        match self {
            IUnop::Clz => executor.push_value(Val::I32(val.leading_zeros())),
            IUnop::Ctz => executor.push_value(Val::I32(val.trailing_zeros())),
            IUnop::Popcnt => executor.push_value(Val::I32(val.count_ones())),
        };
        ExecutionResult::Continue
    }
}

impl Execute for IBinop<{ IntType::I32 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::I32, Val::I32);

        match self {
            IBinop::Add => executor.push_value(Val::I32(lhs.wrapping_add(rhs))),
            IBinop::And => executor.push_value(Val::I32(lhs & rhs)),
            IBinop::DivS => executor.push_value(Val::I32((lhs as i32 / rhs as i32) as u32)),
            IBinop::DivU => executor.push_value(Val::I32(lhs / rhs)),
            IBinop::Mul => executor.push_value(Val::I32(lhs.wrapping_mul(rhs))),
            IBinop::Or => executor.push_value(Val::I32(lhs | rhs)),
            IBinop::RemS => executor.push_value(Val::I32((lhs as i32 % rhs as i32) as u32)),
            IBinop::RemU => executor.push_value(Val::I32(lhs % rhs)),
            IBinop::Rotl => executor.push_value(Val::I32(lhs.rotate_left(rhs % 32))),
            IBinop::Rotr => executor.push_value(Val::I32(lhs.rotate_right(rhs % 32))),
            IBinop::Shl => executor.push_value(Val::I32(lhs.wrapping_shl(rhs % 32))),
            IBinop::ShrS => {
                executor.push_value(Val::I32((lhs as i32).wrapping_shr(rhs % 32) as u32))
            }
            IBinop::ShrU => executor.push_value(Val::I32(lhs.wrapping_shr(rhs % 32))),
            IBinop::Sub => executor.push_value(Val::I32(lhs.wrapping_sub(rhs))),
            IBinop::Xor => executor.push_value(Val::I32(lhs ^ rhs)),
        }
        ExecutionResult::Continue
    }
}

impl Execute for IBinop<{ IntType::I64 }> {
    fn execute(&self, executor: &mut Executor) -> ExecutionResult {
        let (lhs, rhs) = safe_pop!(executor, Val::I64, Val::I64);
        match self {
            IBinop::Add => executor.push_value(Val::I64(lhs.wrapping_add(rhs))),
            IBinop::And => executor.push_value(Val::I64(lhs & rhs)),
            IBinop::DivS => executor.push_value(Val::I64((lhs as i64 / rhs as i64) as u64)),
            IBinop::DivU => executor.push_value(Val::I64(lhs / rhs)),
            IBinop::Mul => executor.push_value(Val::I64(lhs.wrapping_mul(rhs))),
            IBinop::Or => executor.push_value(Val::I64(lhs | rhs)),
            IBinop::RemS => executor.push_value(Val::I64((lhs as i64 % rhs as i64) as u64)),
            IBinop::RemU => executor.push_value(Val::I64(lhs % rhs)),
            IBinop::Rotl => executor.push_value(Val::I64(lhs.rotate_left((rhs % 64) as u32))),
            IBinop::Rotr => executor.push_value(Val::I64(lhs.rotate_right((rhs % 64) as u32))),
            IBinop::Shl => executor.push_value(Val::I64(lhs.wrapping_shl((rhs % 64) as u32))),
            IBinop::ShrS => {
                executor.push_value(Val::I64((lhs as i64).wrapping_shr((rhs % 64) as u32) as u64))
            }
            IBinop::ShrU => {
                executor.push_value(Val::I64(lhs.wrapping_shr((rhs % 64) as u32) as u64))
            }
            IBinop::Sub => executor.push_value(Val::I64(lhs.wrapping_sub(rhs))),
            IBinop::Xor => executor.push_value(Val::I64(lhs ^ rhs)),
        }
        ExecutionResult::Continue
    }
}
