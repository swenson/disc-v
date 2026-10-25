// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "C" extension: compressed instructions.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, d, f, fmt, i, pseudo, system, zicbo};
use crate::reg;
use crate::{Extension, Mnemonic};

pub(crate) static C_ADDI4SPN: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        illegal_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAddi4spn, Codec::Ciw4spn, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_FLD: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FLD), Some(&d::FLD), None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFld, Codec::ClLd, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_LW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LW), Some(&i::LW), Some(&i::LW)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLw, Codec::ClLw, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FLW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FLW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFlw, Codec::ClLw, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcf])
};
pub(crate) static C_FSD: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FSD), Some(&d::FSD), None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFsd, Codec::CsSd, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_SW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SW), Some(&i::SW), Some(&i::SW)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSw, Codec::CsSw, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FSW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FSW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFsw, Codec::CsSw, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zcf])
};
/// `c.nop` with a nonzero immediate, which is a HINT.
pub(crate) static C_NOP_HINT: Opcode =
    Opcode::new(Mnemonic::CNop, Codec::Ci, fmt::IMM).requires(&[Extension::Zca]);
pub(crate) static C_UNIMP: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::UNIMP, &[])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CUnimp, Codec::None, fmt::NONE).requires(&[Extension::Zca])
};
pub(crate) static C_NOP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CNop, Codec::CiNone, fmt::NONE).requires(&[Extension::Zca])
};
pub(crate) static C_ADDI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAddi, Codec::Ci, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_JAL: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JAL), None, None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CJal, Codec::CjJal, fmt::RD_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_LI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        hint_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLi, Codec::CiLi, fmt::RD_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_ADDI16SP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        illegal_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAddi16sp, Codec::Ci16sp, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_LUI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LUI), Some(&i::LUI), Some(&i::LUI)],
        illegal_if: &[ImmEq(0)],
        hint_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLui, Codec::CiLui, fmt::RD_UIMM).requires(&[Extension::Zca])
};
pub(crate) static C_SRLI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SRLI), Some(&i::SRLI), Some(&i::SRLI)],
        hint_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSrli, Codec::CbSh6, fmt::RD_SHAMT).requires(&[Extension::Zca])
};
pub(crate) static C_SRAI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SRAI), Some(&i::SRAI), Some(&i::SRAI)],
        hint_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSrai, Codec::CbSh6, fmt::RD_SHAMT).requires(&[Extension::Zca])
};
pub(crate) static C_ANDI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ANDI), Some(&i::ANDI), Some(&i::ANDI)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAndi, Codec::CbImm, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_SUB: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SUB), Some(&i::SUB), Some(&i::SUB)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSub, Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_XOR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::XOR), Some(&i::XOR), Some(&i::XOR)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CXor, Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_OR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::OR), Some(&i::OR), Some(&i::OR)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::COr, Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_AND: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::AND), Some(&i::AND), Some(&i::AND)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAnd, Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_SUBW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SUBW), Some(&i::SUBW), Some(&i::SUBW)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSubw, Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_ADDW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDW), Some(&i::ADDW), Some(&i::ADDW)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAddw, Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_J: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JAL), Some(&i::JAL), Some(&i::JAL)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CJ, Codec::Cj, fmt::RD_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_BEQZ: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::BEQ), Some(&i::BEQ), Some(&i::BEQ)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CBeqz, Codec::Cb, fmt::RS1_RS2_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_BNEZ: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::BNE), Some(&i::BNE), Some(&i::BNE)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CBnez, Codec::Cb, fmt::RS1_RS2_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_SLLI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SLLI), Some(&i::SLLI), Some(&i::SLLI)],
        hint_if: &[RdEq(reg::ZERO), ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSlli, Codec::CiSh6, fmt::RD_SHAMT).requires(&[Extension::Zca])
};
pub(crate) static C_FLDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FLD), Some(&d::FLD), Some(&d::FLD)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFldsp, Codec::CiLdsp, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_LWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LW), Some(&i::LW), Some(&i::LW)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLwsp, Codec::CiLwsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FLWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FLW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFlwsp, Codec::CiLwsp, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcf])
};
pub(crate) static C_JR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JALR), Some(&i::JALR), Some(&i::JALR)],
        illegal_if: &[Rs1Eq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CJr, Codec::CrJr, fmt::RS1).requires(&[Extension::Zca])
};
pub(crate) static C_MV: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        hint_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CMv, Codec::CrMv, fmt::RD_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_EBREAK: Opcode = Opcode {
    extra: &Extra {
        decompress: [
            Some(&system::EBREAK),
            Some(&system::EBREAK),
            Some(&system::EBREAK),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CEbreak, Codec::CiNone, fmt::NONE).requires(&[Extension::Zca])
};
pub(crate) static C_JALR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JALR), Some(&i::JALR), Some(&i::JALR)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CJalr, Codec::CrJalr, fmt::RS1).requires(&[Extension::Zca])
};
pub(crate) static C_ADD: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADD), Some(&i::ADD), Some(&i::ADD)],
        hint_if: &[RdEq(reg::ZERO)],
        // Aliases of the HINT, which has rd=zero.
        pseudo: &[
            Pseudo::new(&zicbo::NTL_P1, &[Rs2Eq(2)]),
            Pseudo::new(&zicbo::NTL_PALL, &[Rs2Eq(3)]),
            Pseudo::new(&zicbo::NTL_S1, &[Rs2Eq(4)]),
            Pseudo::new(&zicbo::NTL_ALL, &[Rs2Eq(5)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAdd, Codec::Cr, fmt::RD_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_FSDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FSD), Some(&d::FSD), Some(&d::FSD)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFsdsp, Codec::CssSdsp, fmt::FRS2_OFFSET_RS1)
        .requires(&[Extension::Zcd])
};
pub(crate) static C_SWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SW), Some(&i::SW), Some(&i::SW)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSwsp, Codec::CssSwsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FSWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FSW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CFswsp, Codec::CssSwsp, fmt::FRS2_OFFSET_RS1)
        .requires(&[Extension::Zcf])
};
pub(crate) static C_LD: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::LD), Some(&i::LD)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLd, Codec::ClLd, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SD: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::SD), Some(&i::SD)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSd, Codec::CsSd, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_ADDIW: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::ADDIW), Some(&i::ADDIW)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CAddiw, Codec::Ci, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_LDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::LD), Some(&i::LD)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLdsp, Codec::CiLdsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::SD), Some(&i::SD)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSdsp, Codec::CssSdsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_LQ: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::LQ)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLq, Codec::ClLq, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SQ: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::SQ)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSq, Codec::CsSq, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_LQSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::LQ)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CLqsp, Codec::CiLqsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SQSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::SQ)],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CSqsp, Codec::CssSqsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
