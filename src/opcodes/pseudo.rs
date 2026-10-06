// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Pseudoinstructions and aliases that real instructions are shown as
//! when their operands match. See [`Opcode::pseudo`](super::Opcode::pseudo).

use super::{fmt, Codec, Opcode};

pub(crate) static NOP: Opcode = Opcode::new("nop", Codec::I, fmt::NONE);
pub(crate) static MV: Opcode = Opcode::new("mv", Codec::I, fmt::RD_RS1);
pub(crate) static NOT: Opcode = Opcode::new("not", Codec::I, fmt::RD_RS1);
pub(crate) static NEG: Opcode = Opcode::new("neg", Codec::R, fmt::RD_RS2);
pub(crate) static NEGW: Opcode = Opcode::new("negw", Codec::R, fmt::RD_RS2);
pub(crate) static SEXT_W: Opcode = Opcode::new("sext.w", Codec::I, fmt::RD_RS1);
pub(crate) static SEQZ: Opcode = Opcode::new("seqz", Codec::I, fmt::RD_RS1);
pub(crate) static SNEZ: Opcode = Opcode::new("snez", Codec::R, fmt::RD_RS2);
pub(crate) static SLTZ: Opcode = Opcode::new("sltz", Codec::R, fmt::RD_RS1);
pub(crate) static SGTZ: Opcode = Opcode::new("sgtz", Codec::R, fmt::RD_RS2);
pub(crate) static FMV_S: Opcode = Opcode::new("fmv.s", Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_S: Opcode = Opcode::new("fabs.s", Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_S: Opcode = Opcode::new("fneg.s", Codec::R, fmt::FRD_FRS1);
pub(crate) static FMV_D: Opcode = Opcode::new("fmv.d", Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_D: Opcode = Opcode::new("fabs.d", Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_D: Opcode = Opcode::new("fneg.d", Codec::R, fmt::FRD_FRS1);
pub(crate) static FMV_Q: Opcode = Opcode::new("fmv.q", Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_Q: Opcode = Opcode::new("fabs.q", Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_Q: Opcode = Opcode::new("fneg.q", Codec::R, fmt::FRD_FRS1);
pub(crate) static BEQZ: Opcode = Opcode::new("beqz", Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BNEZ: Opcode = Opcode::new("bnez", Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BLEZ: Opcode = Opcode::new("blez", Codec::SB, fmt::RS2_OFFSET);
pub(crate) static BGEZ: Opcode = Opcode::new("bgez", Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BLTZ: Opcode = Opcode::new("bltz", Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BGTZ: Opcode = Opcode::new("bgtz", Codec::SB, fmt::RS2_OFFSET);
pub(crate) static JAL: Opcode = Opcode::new("jal", Codec::Uj, fmt::OFFSET);
pub(crate) static JALR: Opcode = Opcode::new("jalr", Codec::I, fmt::RS1);
pub(crate) static J: Opcode = Opcode::new("j", Codec::Uj, fmt::OFFSET);
pub(crate) static RET: Opcode = Opcode::new("ret", Codec::I, fmt::NONE);
pub(crate) static JR: Opcode = Opcode::new("jr", Codec::I, fmt::RS1);
pub(crate) static RDCYCLE: Opcode = Opcode::new("rdcycle", Codec::ICsr, fmt::RD);
pub(crate) static RDTIME: Opcode = Opcode::new("rdtime", Codec::ICsr, fmt::RD);
pub(crate) static RDINSTRET: Opcode = Opcode::new("rdinstret", Codec::ICsr, fmt::RD);
pub(crate) static RDCYCLEH: Opcode = Opcode::new("rdcycleh", Codec::ICsr, fmt::RD);
pub(crate) static RDTIMEH: Opcode = Opcode::new("rdtimeh", Codec::ICsr, fmt::RD);
pub(crate) static RDINSTRETH: Opcode = Opcode::new("rdinstreth", Codec::ICsr, fmt::RD);
pub(crate) static FRCSR: Opcode = Opcode::new("frcsr", Codec::ICsr, fmt::RD);
pub(crate) static FRRM: Opcode = Opcode::new("frrm", Codec::ICsr, fmt::RD);
pub(crate) static FRFLAGS: Opcode = Opcode::new("frflags", Codec::ICsr, fmt::RD);
pub(crate) static FSCSR: Opcode = Opcode::new("fscsr", Codec::ICsr, fmt::RD_RS1);
pub(crate) static FSRM: Opcode = Opcode::new("fsrm", Codec::ICsr, fmt::RD_RS1);
pub(crate) static FSFLAGS: Opcode = Opcode::new("fsflags", Codec::ICsr, fmt::RD_RS1);
pub(crate) static FSRMI: Opcode = Opcode::new("fsrmi", Codec::ICsr, fmt::RD_ZIMM);
pub(crate) static FSFLAGSI: Opcode = Opcode::new("fsflagsi", Codec::ICsr, fmt::RD_ZIMM);
pub(crate) static LI: Opcode = Opcode::new("li", Codec::Li, fmt::RD_IMM);
pub(crate) static ZEXT_B: Opcode = Opcode::new("zext.b", Codec::I, fmt::RD_RS1);
pub(crate) static JR_OFFSET: Opcode = Opcode::new("jr", Codec::I, fmt::OFFSET_RS1);
pub(crate) static JALR_RD_RS1: Opcode = Opcode::new("jalr", Codec::I, fmt::RD_RS1);
pub(crate) static JALR_OFFSET: Opcode = Opcode::new("jalr", Codec::I, fmt::OFFSET_RS1);
pub(crate) static UNIMP: Opcode = Opcode::new("unimp", Codec::None, fmt::NONE);
pub(crate) static SFENCE_VMA_ALL: Opcode = Opcode::new("sfence.vma", Codec::R, fmt::NONE);
pub(crate) static SFENCE_VMA_RS1: Opcode = Opcode::new("sfence.vma", Codec::R, fmt::RS1);
pub(crate) static CSRR: Opcode = Opcode::new("csrr", Codec::ICsr, fmt::RD_CSR);
pub(crate) static CSRW: Opcode = Opcode::new("csrw", Codec::ICsr, fmt::CSR_RS1);
pub(crate) static CSRS: Opcode = Opcode::new("csrs", Codec::ICsr, fmt::CSR_RS1);
pub(crate) static CSRC: Opcode = Opcode::new("csrc", Codec::ICsr, fmt::CSR_RS1);
pub(crate) static CSRWI: Opcode = Opcode::new("csrwi", Codec::ICsr, fmt::CSR_ZIMM);
pub(crate) static CSRSI: Opcode = Opcode::new("csrsi", Codec::ICsr, fmt::CSR_ZIMM);
pub(crate) static CSRCI: Opcode = Opcode::new("csrci", Codec::ICsr, fmt::CSR_ZIMM);
pub(crate) static FSCSR_RS1: Opcode = Opcode::new("fscsr", Codec::ICsr, fmt::RS1);
pub(crate) static FSRM_RS1: Opcode = Opcode::new("fsrm", Codec::ICsr, fmt::RS1);
pub(crate) static FSFLAGS_RS1: Opcode = Opcode::new("fsflags", Codec::ICsr, fmt::RS1);
