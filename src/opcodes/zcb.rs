// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zcb: simple compressed instructions.
//!
//! The unary instructions expand directly to the aliases objdump shows them
//! as (`zext.b` rather than `andi rd,rd,255`).

use super::{Codec, Extra, Opcode, b, fmt, i, m, pseudo};
use crate::{Extension, Mnemonic};

pub(crate) static C_LBU: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LBU), Some(&i::LBU), Some(&i::LBU)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLbu, Codec::ClB, fmt::RD_OFFSET_RS1).requires(&[Extension::Zcb])
};
pub(crate) static C_LHU: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LHU), Some(&i::LHU), Some(&i::LHU)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLhu, Codec::ClH, fmt::RD_OFFSET_RS1).requires(&[Extension::Zcb])
};
pub(crate) static C_LH: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LH), Some(&i::LH), Some(&i::LH)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLh, Codec::ClH, fmt::RD_OFFSET_RS1).requires(&[Extension::Zcb])
};
pub(crate) static C_SB: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SB), Some(&i::SB), Some(&i::SB)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSb, Codec::CsB, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zcb])
};
pub(crate) static C_SH: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SH), Some(&i::SH), Some(&i::SH)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSh, Codec::CsH, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zcb])
};
pub(crate) static C_ZEXT_B: Opcode = Opcode {
    extra: &Extra {
        decompress: [
            Some(&pseudo::ZEXT_B),
            Some(&pseudo::ZEXT_B),
            Some(&pseudo::ZEXT_B),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CZextB, Codec::CuRd, fmt::RD).requires(&[Extension::Zcb])
};
pub(crate) static C_SEXT_B: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&b::SEXT_B), Some(&b::SEXT_B), Some(&b::SEXT_B)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSextB, Codec::CuRd, fmt::RD).requires(&[Extension::Zcb])
};
pub(crate) static C_ZEXT_H: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&b::ZEXT_H), Some(&b::ZEXT_H), Some(&b::ZEXT_H)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CZextH, Codec::CuRd, fmt::RD).requires(&[Extension::Zcb])
};
pub(crate) static C_SEXT_H: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&b::SEXT_H), Some(&b::SEXT_H), Some(&b::SEXT_H)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSextH, Codec::CuRd, fmt::RD).requires(&[Extension::Zcb])
};
pub(crate) static C_ZEXT_W: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&b::ZEXT_W), Some(&b::ZEXT_W)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CZextW, Codec::CuRd, fmt::RD).requires(&[Extension::Zcb])
};
pub(crate) static C_NOT: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&pseudo::NOT), Some(&pseudo::NOT), Some(&pseudo::NOT)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CNot, Codec::CuRd, fmt::RD).requires(&[Extension::Zcb])
};
pub(crate) static C_MUL: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&m::MUL), Some(&m::MUL), Some(&m::MUL)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CMul, Codec::Cs, fmt::RD_RS2).requires(&[Extension::Zcb])
};
