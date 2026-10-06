// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! RV32I, RV64I and RV128I base integer instructions, including Zifencei.

use super::Constraint::*;
use super::{Codec, Opcode, Pseudo, fmt, pseudo};
use crate::reg;

pub(crate) static LUI: Opcode = Opcode::new("lui", Codec::U, fmt::RD_UIMM);
pub(crate) static AUIPC: Opcode = Opcode::new("auipc", Codec::U, fmt::RD_UIMM);
pub(crate) static JAL: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::J, &[RdEq(reg::ZERO)]),
        Pseudo::new(&pseudo::JAL, &[RdEq(reg::RA)]),
    ],
    ..Opcode::new("jal", Codec::Uj, fmt::RD_OFFSET)
};
pub(crate) static JALR: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::RET, &[RdEq(reg::ZERO), Rs1Eq(reg::RA), ImmEq(0)]),
        Pseudo::new(&pseudo::JR, &[RdEq(reg::ZERO), ImmEq(0)]),
        Pseudo::new(&pseudo::JR_OFFSET, &[RdEq(reg::ZERO)]),
        Pseudo::new(&pseudo::JALR, &[RdEq(reg::RA), ImmEq(0)]),
        Pseudo::new(&pseudo::JALR_OFFSET, &[RdEq(reg::RA)]),
        Pseudo::new(&pseudo::JALR_RD_RS1, &[ImmEq(0)]),
    ],
    ..Opcode::new("jalr", Codec::I, fmt::RD_OFFSET_RS1)
};
pub(crate) static BEQ: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::BEQZ, &[Rs2Eq(reg::ZERO)])],
    ..Opcode::new("beq", Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BNE: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::BNEZ, &[Rs2Eq(reg::ZERO)])],
    ..Opcode::new("bne", Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BLT: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::BLTZ, &[Rs2Eq(reg::ZERO)]),
        Pseudo::new(&pseudo::BGTZ, &[Rs1Eq(reg::ZERO)]),
    ],
    ..Opcode::new("blt", Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BGE: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::BLEZ, &[Rs1Eq(reg::ZERO)]),
        Pseudo::new(&pseudo::BGEZ, &[Rs2Eq(reg::ZERO)]),
    ],
    ..Opcode::new("bge", Codec::SB, fmt::RS1_RS2_OFFSET)
};
pub(crate) static BLTU: Opcode = Opcode::new("bltu", Codec::SB, fmt::RS1_RS2_OFFSET);
pub(crate) static BGEU: Opcode = Opcode::new("bgeu", Codec::SB, fmt::RS1_RS2_OFFSET);
pub(crate) static LB: Opcode = Opcode::new("lb", Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LH: Opcode = Opcode::new("lh", Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LW: Opcode = Opcode::new("lw", Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LBU: Opcode = Opcode::new("lbu", Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static LHU: Opcode = Opcode::new("lhu", Codec::I, fmt::RD_OFFSET_RS1);
pub(crate) static SB: Opcode = Opcode::new("sb", Codec::S, fmt::RS2_OFFSET_RS1);
pub(crate) static SH: Opcode = Opcode::new("sh", Codec::S, fmt::RS2_OFFSET_RS1);
pub(crate) static SW: Opcode = Opcode::new("sw", Codec::S, fmt::RS2_OFFSET_RS1);
pub(crate) static ADDI: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::NOP, &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), ImmEq(0)]),
        Pseudo::new(&pseudo::LI, &[Rs1Eq(reg::ZERO)]),
        Pseudo::new(&pseudo::MV, &[ImmEq(0)]),
    ],
    ..Opcode::new("addi", Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static SLTI: Opcode = Opcode::new("slti", Codec::I, fmt::RD_RS1_IMM);
pub(crate) static SLTIU: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::SEQZ, &[ImmEq(1)])],
    ..Opcode::new("sltiu", Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static XORI: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::NOT, &[ImmEq(-1)])],
    ..Opcode::new("xori", Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static ORI: Opcode = Opcode::new("ori", Codec::I, fmt::RD_RS1_IMM);
pub(crate) static ANDI: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::ZEXT_B, &[ImmEq(255)])],
    ..Opcode::new("andi", Codec::I, fmt::RD_RS1_IMM)
};
pub(crate) static SLLI: Opcode = Opcode::new("slli", Codec::ISh7, fmt::RD_RS1_SHAMT);
pub(crate) static SRLI: Opcode = Opcode::new("srli", Codec::ISh7, fmt::RD_RS1_SHAMT);
pub(crate) static SRAI: Opcode = Opcode::new("srai", Codec::ISh7, fmt::RD_RS1_SHAMT);
pub(crate) static ADD: Opcode = Opcode::new("add", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SUB: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::NEG, &[Rs1Eq(reg::ZERO)])],
    ..Opcode::new("sub", Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static SLL: Opcode = Opcode::new("sll", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SLT: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::SLTZ, &[Rs2Eq(reg::ZERO)]),
        Pseudo::new(&pseudo::SGTZ, &[Rs1Eq(reg::ZERO)]),
    ],
    ..Opcode::new("slt", Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static SLTU: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::SNEZ, &[Rs1Eq(reg::ZERO)])],
    ..Opcode::new("sltu", Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static XOR: Opcode = Opcode::new("xor", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SRL: Opcode = Opcode::new("srl", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static SRA: Opcode = Opcode::new("sra", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static OR: Opcode = Opcode::new("or", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static AND: Opcode = Opcode::new("and", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static FENCE_TSO: Opcode = Opcode::new("fence.tso", Codec::None, fmt::NONE);
pub(crate) static FENCE: Opcode = Opcode::new("fence", Codec::RF, fmt::PRED_SUCC);
pub(crate) static FENCE_I: Opcode = Opcode::new("fence.i", Codec::None, fmt::NONE);
pub(crate) static LWU: Opcode = Opcode::new("lwu", Codec::I, fmt::RD_OFFSET_RS1).rv64();
pub(crate) static LD: Opcode = Opcode::new("ld", Codec::I, fmt::RD_OFFSET_RS1).rv64();
pub(crate) static SD: Opcode = Opcode::new("sd", Codec::S, fmt::RS2_OFFSET_RS1).rv64();
pub(crate) static ADDIW: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::SEXT_W, &[ImmEq(0)])],
    ..Opcode::new("addiw", Codec::I, fmt::RD_RS1_IMM).rv64()
};
pub(crate) static SLLIW: Opcode = Opcode::new("slliw", Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static SRLIW: Opcode = Opcode::new("srliw", Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static SRAIW: Opcode = Opcode::new("sraiw", Codec::ISh5, fmt::RD_RS1_SHAMT).rv64();
pub(crate) static ADDW: Opcode = Opcode::new("addw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SUBW: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::NEGW, &[Rs1Eq(reg::ZERO)])],
    ..Opcode::new("subw", Codec::R, fmt::RD_RS1_RS2).rv64()
};
pub(crate) static SLLW: Opcode = Opcode::new("sllw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SRLW: Opcode = Opcode::new("srlw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static SRAW: Opcode = Opcode::new("sraw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static LDU: Opcode = Opcode::new("ldu", Codec::I, fmt::RD_OFFSET_RS1).rv128();
pub(crate) static LQ: Opcode = Opcode::new("lq", Codec::I, fmt::RD_OFFSET_RS1).rv128();
pub(crate) static SQ: Opcode = Opcode::new("sq", Codec::S, fmt::RS2_OFFSET_RS1).rv128();
pub(crate) static ADDID: Opcode = Opcode::new("addid", Codec::I, fmt::RD_RS1_IMM).rv128();
pub(crate) static SLLID: Opcode = Opcode::new("sllid", Codec::ISh6, fmt::RD_RS1_SHAMT).rv128();
pub(crate) static SRLID: Opcode = Opcode::new("srlid", Codec::ISh6, fmt::RD_RS1_SHAMT).rv128();
pub(crate) static SRAID: Opcode = Opcode::new("sraid", Codec::ISh6, fmt::RD_RS1_SHAMT).rv128();
pub(crate) static ADDD: Opcode = Opcode::new("addd", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SUBD: Opcode = Opcode::new("subd", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SLLD: Opcode = Opcode::new("slld", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SRLD: Opcode = Opcode::new("srld", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static SRAD: Opcode = Opcode::new("srad", Codec::R, fmt::RD_RS1_RS2).rv128();
