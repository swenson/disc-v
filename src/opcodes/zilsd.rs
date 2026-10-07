// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Load and store register pairs on RV32 (Zilsd), and their compressed forms
//! (Zclsd), which use the encodings of Zcf's `c.flw` family.
//!
//! The registers are even-odd pairs, so odd registers are reserved.

use super::Constraint::*;
use super::{Codec, Opcode, fmt};
use crate::{Extension, reg};

pub(crate) static LD: Opcode = Opcode {
    illegal_if: &[RdOdd],
    ..Opcode::new("ld", Codec::I, fmt::RD_OFFSET_RS1)
        .requires(&[Extension::Zilsd])
        .rv32_only()
};
pub(crate) static SD: Opcode = Opcode {
    illegal_if: &[Rs2Odd],
    ..Opcode::new("sd", Codec::S, fmt::RS2_OFFSET_RS1)
        .requires(&[Extension::Zilsd])
        .rv32_only()
};
pub(crate) static C_LD: Opcode = Opcode {
    decompress: [Some(&LD), None, None],
    illegal_if: &[RdOdd],
    ..Opcode::new("c.ld", Codec::ClLd, fmt::RD_OFFSET_RS1).requires(&[Extension::Zclsd])
};
pub(crate) static C_SD: Opcode = Opcode {
    decompress: [Some(&SD), None, None],
    illegal_if: &[Rs2Odd],
    ..Opcode::new("c.sd", Codec::CsSd, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zclsd])
};
pub(crate) static C_LDSP: Opcode = Opcode {
    decompress: [Some(&LD), None, None],
    illegal_if: &[RdOdd, RdEq(reg::ZERO)],
    ..Opcode::new("c.ldsp", Codec::CiLdsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zclsd])
};
pub(crate) static C_SDSP: Opcode = Opcode {
    decompress: [Some(&SD), None, None],
    illegal_if: &[Rs2Odd],
    ..Opcode::new("c.sdsp", Codec::CssSdsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zclsd])
};
