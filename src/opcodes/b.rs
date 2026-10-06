//! Bit-manipulation extensions: Zba, Zbb, Zbc and Zbs.

use super::Constraint::*;
use super::{fmt, Codec, Opcode, Pseudo};
use crate::reg;

pub(crate) static ANDN: Opcode = Opcode::new("andn", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static ADD_UW: Opcode = Opcode {
    pseudo: &[Pseudo::new(&ZEXT_W, &[Rs2Eq(reg::ZERO)])],
    ..Opcode::new("add.uw", Codec::R, fmt::RD_RS1_RS2).rv64()
};
pub(crate) static BSET: Opcode = Opcode::new("bset", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static BSETI: Opcode = Opcode::new("bseti", Codec::ISh5, fmt::RD_RS1_SHAMT);
pub(crate) static BSET_64: Opcode = Opcode::new("bseti", Codec::ISh6, fmt::RD_RS1_SHAMT);
pub(crate) static BCLR: Opcode = Opcode::new("bclr", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static BCLRI: Opcode = Opcode::new("bclri", Codec::ISh5, fmt::RD_RS1_SHAMT);
pub(crate) static BCLRI_64: Opcode = Opcode::new("bclri", Codec::ISh6, fmt::RD_RS1_SHAMT);
pub(crate) static BEXT: Opcode = Opcode::new("bext", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static BEXTI: Opcode = Opcode::new("bexti", Codec::ISh5, fmt::RD_RS1_SHAMT);
pub(crate) static BEXTI_64: Opcode = Opcode::new("bexti", Codec::ISh6, fmt::RD_RS1_SHAMT);
pub(crate) static BINV: Opcode = Opcode::new("binv", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static BINVI: Opcode = Opcode::new("binvi", Codec::ISh5, fmt::RD_RS1_SHAMT);
pub(crate) static BINVI_64: Opcode = Opcode::new("binvi", Codec::ISh6, fmt::RD_RS1_SHAMT);
pub(crate) static CLMUL: Opcode = Opcode::new("clmul", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static CLMULH: Opcode = Opcode::new("clmulh", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static CLMULR: Opcode = Opcode::new("clmulr", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static CLZ: Opcode = Opcode::new("clz", Codec::R, fmt::RD_RS1);
pub(crate) static CLZW: Opcode = Opcode::new("clzw", Codec::R, fmt::RD_RS1).rv64();
pub(crate) static CPOP: Opcode = Opcode::new("cpop", Codec::R, fmt::RD_RS1);
pub(crate) static CPOPW: Opcode = Opcode::new("cpopw", Codec::R, fmt::RD_RS1).rv64();
pub(crate) static CTZ: Opcode = Opcode::new("ctz", Codec::R, fmt::RD_RS1);
pub(crate) static CTZW: Opcode = Opcode::new("ctzw", Codec::R, fmt::RD_RS1).rv64();
pub(crate) static MAX: Opcode = Opcode::new("max", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MAXU: Opcode = Opcode::new("maxu", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MIN: Opcode = Opcode::new("min", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MINU: Opcode = Opcode::new("minu", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static ORC_B: Opcode = Opcode::new("orc.b", Codec::R, fmt::RD_RS1);
pub(crate) static ORN: Opcode = Opcode::new("orn", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static REV8: Opcode = Opcode::new("rev8", Codec::R, fmt::RD_RS1);
pub(crate) static ROL: Opcode = Opcode::new("rol", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static ROLW: Opcode = Opcode::new("rolw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static ROR: Opcode = Opcode::new("ror", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static RORI: Opcode = Opcode::new("rori", Codec::ISh5, fmt::RD_RS1_SHAMT);
pub(crate) static RORI_64: Opcode = Opcode::new("rori", Codec::ISh6, fmt::RD_RS1_SHAMT);
pub(crate) static RORIW: Opcode = Opcode::new("roriw", Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static RORW: Opcode = Opcode::new("rorw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SEXT_B: Opcode = Opcode::new("sext.b", Codec::R, fmt::RD_RS1);
pub(crate) static SEXT_H: Opcode = Opcode::new("sext.h", Codec::R, fmt::RD_RS1);
pub(crate) static SH1ADD: Opcode = Opcode::new("sh1add", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SH1ADD_UW: Opcode = Opcode::new("sh1add.uw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SH2ADD: Opcode = Opcode::new("sh2add", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SH2ADD_UW: Opcode = Opcode::new("sh2add.uw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SH3ADD: Opcode = Opcode::new("sh3add", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SH3ADD_UW: Opcode = Opcode::new("sh3add.uw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SLLI_UW: Opcode = Opcode::new("slli.uw", Codec::ISh6, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static XNOR: Opcode = Opcode::new("xnor", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static ZEXT_H: Opcode = Opcode::new("zext.h", Codec::R, fmt::RD_RS1);
pub(crate) static ZEXT_W: Opcode = Opcode::new("zext.w", Codec::R, fmt::RD_RS1);
