// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zcb: simple compressed instructions.
//!
//! The unary instructions expand directly to the aliases objdump shows them
//! as (`zext.b` rather than `andi rd,rd,255`).

use super::{Codec, Opcode, b, fmt, i, m, pseudo};

pub(crate) static C_LBU: Opcode = Opcode {
    decompress: [Some(&i::LBU), Some(&i::LBU), Some(&i::LBU)],
    ..Opcode::new("c.lbu", Codec::ClB, fmt::RD_OFFSET_RS1)
};
pub(crate) static C_LHU: Opcode = Opcode {
    decompress: [Some(&i::LHU), Some(&i::LHU), Some(&i::LHU)],
    ..Opcode::new("c.lhu", Codec::ClH, fmt::RD_OFFSET_RS1)
};
pub(crate) static C_LH: Opcode = Opcode {
    decompress: [Some(&i::LH), Some(&i::LH), Some(&i::LH)],
    ..Opcode::new("c.lh", Codec::ClH, fmt::RD_OFFSET_RS1)
};
pub(crate) static C_SB: Opcode = Opcode {
    decompress: [Some(&i::SB), Some(&i::SB), Some(&i::SB)],
    ..Opcode::new("c.sb", Codec::CsB, fmt::RS2_OFFSET_RS1)
};
pub(crate) static C_SH: Opcode = Opcode {
    decompress: [Some(&i::SH), Some(&i::SH), Some(&i::SH)],
    ..Opcode::new("c.sh", Codec::CsH, fmt::RS2_OFFSET_RS1)
};
pub(crate) static C_ZEXT_B: Opcode = Opcode {
    decompress: [
        Some(&pseudo::ZEXT_B),
        Some(&pseudo::ZEXT_B),
        Some(&pseudo::ZEXT_B),
    ],
    ..Opcode::new("c.zext.b", Codec::CuRd, fmt::RD)
};
pub(crate) static C_SEXT_B: Opcode = Opcode {
    decompress: [Some(&b::SEXT_B), Some(&b::SEXT_B), Some(&b::SEXT_B)],
    ..Opcode::new("c.sext.b", Codec::CuRd, fmt::RD)
};
pub(crate) static C_ZEXT_H: Opcode = Opcode {
    decompress: [Some(&b::ZEXT_H), Some(&b::ZEXT_H), Some(&b::ZEXT_H)],
    ..Opcode::new("c.zext.h", Codec::CuRd, fmt::RD)
};
pub(crate) static C_SEXT_H: Opcode = Opcode {
    decompress: [Some(&b::SEXT_H), Some(&b::SEXT_H), Some(&b::SEXT_H)],
    ..Opcode::new("c.sext.h", Codec::CuRd, fmt::RD)
};
pub(crate) static C_ZEXT_W: Opcode = Opcode {
    decompress: [None, Some(&b::ZEXT_W), Some(&b::ZEXT_W)],
    ..Opcode::new("c.zext.w", Codec::CuRd, fmt::RD)
};
pub(crate) static C_NOT: Opcode = Opcode {
    decompress: [Some(&pseudo::NOT), Some(&pseudo::NOT), Some(&pseudo::NOT)],
    ..Opcode::new("c.not", Codec::CuRd, fmt::RD)
};
pub(crate) static C_MUL: Opcode = Opcode {
    decompress: [Some(&m::MUL), Some(&m::MUL), Some(&m::MUL)],
    ..Opcode::new("c.mul", Codec::Cs, fmt::RD_RS2)
};
