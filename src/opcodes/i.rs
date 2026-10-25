// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! RV32I, RV64I and RV128I base integer instructions, including Zifencei.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt, pseudo, zicbo, zimop};
use crate::reg;
use crate::{Extension, Mnemonic};

pub(crate) static LUI: Opcode = Opcode::new(Mnemonic::Lui, Codec::U, fmt::RD_UIMM);
pub(crate) static AUIPC: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&zimop::LPAD, &[RdEq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Auipc, Codec::U, fmt::RD_UIMM)
};
pub(crate) static JAL: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::J, &[RdEq(reg::ZERO)]),
            Pseudo::new(&pseudo::JAL, &[RdEq(reg::RA)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Jal, Codec::Uj, fmt::RD_OFFSET)
};
pub(crate) static JALR: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::RET, &[RdEq(reg::ZERO), Rs1Eq(reg::RA), ImmEq(0)]),
            Pseudo::new(&pseudo::JR, &[RdEq(reg::ZERO), ImmEq(0)]),
            Pseudo::new(&pseudo::JR_OFFSET, &[RdEq(reg::ZERO)]),
            Pseudo::new(&pseudo::JALR, &[RdEq(reg::RA), ImmEq(0)]),
            Pseudo::new(&pseudo::JALR_OFFSET, &[RdEq(reg::RA)]),
            Pseudo::new(&pseudo::JALR_RD_RS1, &[ImmEq(0)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Jalr, Codec::I, fmt::RD_OFFSET_RS1)
};
pub(crate) static BEQ: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::BEQZ, &[Rs2Eq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Beq, Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BNE: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::BNEZ, &[Rs2Eq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Bne, Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BLT: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::BLTZ, &[Rs2Eq(reg::ZERO)]),
            Pseudo::new(&pseudo::BGTZ, &[Rs1Eq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Blt, Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BGE: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::BLEZ, &[Rs1Eq(reg::ZERO)]),
            Pseudo::new(&pseudo::BGEZ, &[Rs2Eq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Bge, Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BLTU: Opcode = Opcode::new(Mnemonic::Bltu, Codec::SB, fmt::RS1_RS2_OFFSET);
pub(crate) static BGEU: Opcode = Opcode::new(Mnemonic::Bgeu, Codec::SB, fmt::RS1_RS2_OFFSET);
pub(crate) static LB: Opcode = Opcode::new(Mnemonic::Lb, Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LH: Opcode = Opcode::new(Mnemonic::Lh, Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LW: Opcode = Opcode::new(Mnemonic::Lw, Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LBU: Opcode = Opcode::new(Mnemonic::Lbu, Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LHU: Opcode = Opcode::new(Mnemonic::Lhu, Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static SB: Opcode = Opcode::new(Mnemonic::Sb, Codec::S, fmt::RS2_OFFSET_RS1);
pub(crate) static SH: Opcode = Opcode::new(Mnemonic::Sh, Codec::S, fmt::RS2_OFFSET_RS1);
pub(crate) static SW: Opcode = Opcode::new(Mnemonic::Sw, Codec::S, fmt::RS2_OFFSET_RS1);
pub(crate) static ADDI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::NOP, &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), ImmEq(0)]),
            Pseudo::new(&pseudo::LI, &[Rs1Eq(reg::ZERO)]),
            Pseudo::new(&pseudo::MV, &[ImmEq(0)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Addi, Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static SLTI: Opcode = Opcode::new(Mnemonic::Slti, Codec::I, fmt::RD_RS1_IMM);
pub(crate) static SLTIU: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::SEQZ, &[ImmEq(1)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Sltiu, Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static XORI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::NOT, &[ImmEq(-1)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Xori, Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static ORI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&zicbo::PREFETCH_I, &[RdEq(reg::ZERO), ImmMaskEq(0x1f, 0)]),
            Pseudo::new(&zicbo::PREFETCH_R, &[RdEq(reg::ZERO), ImmMaskEq(0x1f, 1)]),
            Pseudo::new(&zicbo::PREFETCH_W, &[RdEq(reg::ZERO), ImmMaskEq(0x1f, 3)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Ori, Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static ANDI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::ZEXT_B, &[ImmEq(255)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Andi, Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static SLLI: Opcode = Opcode::new(Mnemonic::Slli, Codec::ISh7, fmt::RD_RS1_SHAMT);
pub(crate) static SRLI: Opcode = Opcode::new(Mnemonic::Srli, Codec::ISh7, fmt::RD_RS1_SHAMT);
pub(crate) static SRAI: Opcode = Opcode::new(Mnemonic::Srai, Codec::ISh7, fmt::RD_RS1_SHAMT);
pub(crate) static ADD: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(
                &zicbo::NTL_P1,
                &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), Rs2Eq(2)],
            ),
            Pseudo::new(
                &zicbo::NTL_PALL,
                &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), Rs2Eq(3)],
            ),
            Pseudo::new(
                &zicbo::NTL_S1,
                &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), Rs2Eq(4)],
            ),
            Pseudo::new(
                &zicbo::NTL_ALL,
                &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), Rs2Eq(5)],
            ),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Add, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static SUB: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::NEG, &[Rs1Eq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Sub, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static SLL: Opcode = Opcode::new(Mnemonic::Sll, Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SLT: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::SLTZ, &[Rs2Eq(reg::ZERO)]),
            Pseudo::new(&pseudo::SGTZ, &[Rs1Eq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Slt, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static SLTU: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::SNEZ, &[Rs1Eq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Sltu, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static XOR: Opcode = Opcode::new(Mnemonic::Xor, Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SRL: Opcode = Opcode::new(Mnemonic::Srl, Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SRA: Opcode = Opcode::new(Mnemonic::Sra, Codec::R, fmt::RD_RS1_RS2);
pub(crate) static OR: Opcode = Opcode::new(Mnemonic::Or, Codec::R, fmt::RD_RS1_RS2);
pub(crate) static AND: Opcode = Opcode::new(Mnemonic::And, Codec::R, fmt::RD_RS1_RS2);
pub(crate) static FENCE_TSO: Opcode = Opcode::new(Mnemonic::FenceTso, Codec::None, fmt::NONE);
pub(crate) static FENCE: Opcode = Opcode {
    extra: &Extra {
        // pause is fence w,0 with fm, rd and rs1 zero.
        pseudo: &[Pseudo::new(
            &zicbo::PAUSE,
            &[ImmEq(0x010), RdEq(reg::ZERO), Rs1Eq(reg::ZERO)],
        )],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Fence, Codec::RF, fmt::PRED_SUCC)
};
pub(crate) static FENCE_I: Opcode =
    Opcode::new(Mnemonic::FenceI, Codec::None, fmt::NONE).requires(&[Extension::Zifencei]);
pub(crate) static LWU: Opcode = Opcode::new(Mnemonic::Lwu, Codec::I, fmt::RD_OFFSET_RS1).rv64();
pub(crate) static LD: Opcode = Opcode::new(Mnemonic::Ld, Codec::I, fmt::RD_OFFSET_RS1).rv64();
pub(crate) static SD: Opcode = Opcode::new(Mnemonic::Sd, Codec::S, fmt::RS2_OFFSET_RS1).rv64();
pub(crate) static ADDIW: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::SEXT_W, &[ImmEq(0)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Addiw, Codec::I, fmt::RD_RS1_IMM).rv64()
};
pub(crate) static SLLIW: Opcode =
    Opcode::new(Mnemonic::Slliw, Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static SRLIW: Opcode =
    Opcode::new(Mnemonic::Srliw, Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static SRAIW: Opcode =
    Opcode::new(Mnemonic::Sraiw, Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static ADDW: Opcode = Opcode::new(Mnemonic::Addw, Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SUBW: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::NEGW, &[Rs1Eq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Subw, Codec::R, fmt::RD_RS1_RS2).rv64()
};
pub(crate) static SLLW: Opcode = Opcode::new(Mnemonic::Sllw, Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SRLW: Opcode = Opcode::new(Mnemonic::Srlw, Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SRAW: Opcode = Opcode::new(Mnemonic::Sraw, Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static LDU: Opcode = Opcode::new(Mnemonic::Ldu, Codec::I, fmt::RD_OFFSET_RS1).rv128();
pub(crate) static LQ: Opcode = Opcode::new(Mnemonic::Lq, Codec::I, fmt::RD_OFFSET_RS1).rv128();
pub(crate) static SQ: Opcode = Opcode::new(Mnemonic::Sq, Codec::S, fmt::RS2_OFFSET_RS1).rv128();
pub(crate) static ADDID: Opcode = Opcode::new(Mnemonic::Addid, Codec::I, fmt::RD_RS1_IMM).rv128();
pub(crate) static SLLID: Opcode =
    Opcode::new(Mnemonic::Sllid, Codec::ISh6, fmt::RD_RS1_SHAMT).rv128();
pub(crate) static SRLID: Opcode =
    Opcode::new(Mnemonic::Srlid, Codec::ISh6, fmt::RD_RS1_SHAMT).rv128();
pub(crate) static SRAID: Opcode =
    Opcode::new(Mnemonic::Sraid, Codec::ISh6, fmt::RD_RS1_SHAMT).rv128();
pub(crate) static ADDD: Opcode = Opcode::new(Mnemonic::Addd, Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SUBD: Opcode = Opcode::new(Mnemonic::Subd, Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SLLD: Opcode = Opcode::new(Mnemonic::Slld, Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SRLD: Opcode = Opcode::new(Mnemonic::Srld, Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SRAD: Opcode = Opcode::new(Mnemonic::Srad, Codec::R, fmt::RD_RS1_RS2).rv128();
