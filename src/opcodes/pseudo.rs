// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Pseudoinstructions and aliases that real instructions are shown as
//! when their operands match. See [`Extra::pseudo`](super::Extra::pseudo).

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static NOP: Opcode = Opcode::new(Mnemonic::Nop, Codec::I, fmt::NONE);
pub(crate) static MV: Opcode = Opcode::new(Mnemonic::Mv, Codec::I, fmt::RD_RS1);
pub(crate) static NOT: Opcode = Opcode::new(Mnemonic::Not, Codec::I, fmt::RD_RS1);
pub(crate) static NEG: Opcode = Opcode::new(Mnemonic::Neg, Codec::R, fmt::RD_RS2);
pub(crate) static NEGW: Opcode = Opcode::new(Mnemonic::Negw, Codec::R, fmt::RD_RS2);
pub(crate) static SEXT_W: Opcode = Opcode::new(Mnemonic::SextW, Codec::I, fmt::RD_RS1);
pub(crate) static SEQZ: Opcode = Opcode::new(Mnemonic::Seqz, Codec::I, fmt::RD_RS1);
pub(crate) static SNEZ: Opcode = Opcode::new(Mnemonic::Snez, Codec::R, fmt::RD_RS2);
pub(crate) static SLTZ: Opcode = Opcode::new(Mnemonic::Sltz, Codec::R, fmt::RD_RS1);
pub(crate) static SGTZ: Opcode = Opcode::new(Mnemonic::Sgtz, Codec::R, fmt::RD_RS2);
pub(crate) static FMV_S: Opcode = Opcode::new(Mnemonic::FmvS, Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_S: Opcode = Opcode::new(Mnemonic::FabsS, Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_S: Opcode = Opcode::new(Mnemonic::FnegS, Codec::R, fmt::FRD_FRS1);
pub(crate) static FMV_H: Opcode = Opcode::new(Mnemonic::FmvH, Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_H: Opcode = Opcode::new(Mnemonic::FabsH, Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_H: Opcode = Opcode::new(Mnemonic::FnegH, Codec::R, fmt::FRD_FRS1);
pub(crate) static FMV_D: Opcode = Opcode::new(Mnemonic::FmvD, Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_D: Opcode = Opcode::new(Mnemonic::FabsD, Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_D: Opcode = Opcode::new(Mnemonic::FnegD, Codec::R, fmt::FRD_FRS1);
pub(crate) static FMV_Q: Opcode = Opcode::new(Mnemonic::FmvQ, Codec::R, fmt::FRD_FRS1);
pub(crate) static FABS_Q: Opcode = Opcode::new(Mnemonic::FabsQ, Codec::R, fmt::FRD_FRS1);
pub(crate) static FNEG_Q: Opcode = Opcode::new(Mnemonic::FnegQ, Codec::R, fmt::FRD_FRS1);
pub(crate) static BEQZ: Opcode = Opcode::new(Mnemonic::Beqz, Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BNEZ: Opcode = Opcode::new(Mnemonic::Bnez, Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BLEZ: Opcode = Opcode::new(Mnemonic::Blez, Codec::SB, fmt::RS2_OFFSET);
pub(crate) static BGEZ: Opcode = Opcode::new(Mnemonic::Bgez, Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BLTZ: Opcode = Opcode::new(Mnemonic::Bltz, Codec::SB, fmt::RS1_OFFSET);
pub(crate) static BGTZ: Opcode = Opcode::new(Mnemonic::Bgtz, Codec::SB, fmt::RS2_OFFSET);
pub(crate) static JAL: Opcode = Opcode::new(Mnemonic::Jal, Codec::Uj, fmt::OFFSET);
pub(crate) static JALR: Opcode = Opcode::new(Mnemonic::Jalr, Codec::I, fmt::RS1);
pub(crate) static J: Opcode = Opcode::new(Mnemonic::J, Codec::Uj, fmt::OFFSET);
pub(crate) static RET: Opcode = Opcode::new(Mnemonic::Ret, Codec::I, fmt::NONE);
pub(crate) static JR: Opcode = Opcode::new(Mnemonic::Jr, Codec::I, fmt::RS1);
pub(crate) static RDCYCLE: Opcode = Opcode::new(Mnemonic::Rdcycle, Codec::ICsr, fmt::RD);
pub(crate) static RDTIME: Opcode = Opcode::new(Mnemonic::Rdtime, Codec::ICsr, fmt::RD);
pub(crate) static RDINSTRET: Opcode = Opcode::new(Mnemonic::Rdinstret, Codec::ICsr, fmt::RD);
pub(crate) static RDCYCLEH: Opcode = Opcode::new(Mnemonic::Rdcycleh, Codec::ICsr, fmt::RD);
pub(crate) static RDTIMEH: Opcode = Opcode::new(Mnemonic::Rdtimeh, Codec::ICsr, fmt::RD);
pub(crate) static RDINSTRETH: Opcode = Opcode::new(Mnemonic::Rdinstreth, Codec::ICsr, fmt::RD);
pub(crate) static FRCSR: Opcode =
    Opcode::new(Mnemonic::Frcsr, Codec::ICsr, fmt::RD).requires(&[Extension::F]);
pub(crate) static FRRM: Opcode =
    Opcode::new(Mnemonic::Frrm, Codec::ICsr, fmt::RD).requires(&[Extension::F]);
pub(crate) static FRFLAGS: Opcode =
    Opcode::new(Mnemonic::Frflags, Codec::ICsr, fmt::RD).requires(&[Extension::F]);
pub(crate) static FSCSR: Opcode =
    Opcode::new(Mnemonic::Fscsr, Codec::ICsr, fmt::RD_RS1).requires(&[Extension::F]);
pub(crate) static FSRM: Opcode =
    Opcode::new(Mnemonic::Fsrm, Codec::ICsr, fmt::RD_RS1).requires(&[Extension::F]);
pub(crate) static FSFLAGS: Opcode =
    Opcode::new(Mnemonic::Fsflags, Codec::ICsr, fmt::RD_RS1).requires(&[Extension::F]);
pub(crate) static FSRMI: Opcode =
    Opcode::new(Mnemonic::Fsrmi, Codec::ICsr, fmt::RD_ZIMM).requires(&[Extension::F]);
pub(crate) static FSFLAGSI: Opcode =
    Opcode::new(Mnemonic::Fsflagsi, Codec::ICsr, fmt::RD_ZIMM).requires(&[Extension::F]);
pub(crate) static LI: Opcode = Opcode::new(Mnemonic::Li, Codec::Li, fmt::RD_IMM);
pub(crate) static ZEXT_B: Opcode = Opcode::new(Mnemonic::ZextB, Codec::I, fmt::RD_RS1);
pub(crate) static JR_OFFSET: Opcode = Opcode::new(Mnemonic::Jr, Codec::I, fmt::OFFSET_RS1);
pub(crate) static JALR_RD_RS1: Opcode = Opcode::new(Mnemonic::Jalr, Codec::I, fmt::RD_RS1);
pub(crate) static JALR_OFFSET: Opcode = Opcode::new(Mnemonic::Jalr, Codec::I, fmt::OFFSET_RS1);
pub(crate) static UNIMP: Opcode = Opcode::new(Mnemonic::Unimp, Codec::None, fmt::NONE);
pub(crate) static SFENCE_VMA_ALL: Opcode = Opcode::new(Mnemonic::SfenceVma, Codec::R, fmt::NONE);
pub(crate) static SFENCE_VMA_RS1: Opcode = Opcode::new(Mnemonic::SfenceVma, Codec::R, fmt::RS1);
pub(crate) static CSRR: Opcode = Opcode::new(Mnemonic::Csrr, Codec::ICsr, fmt::RD_CSR);
pub(crate) static CSRW: Opcode = Opcode::new(Mnemonic::Csrw, Codec::ICsr, fmt::CSR_RS1);
pub(crate) static CSRS: Opcode = Opcode::new(Mnemonic::Csrs, Codec::ICsr, fmt::CSR_RS1);
pub(crate) static CSRC: Opcode = Opcode::new(Mnemonic::Csrc, Codec::ICsr, fmt::CSR_RS1);
pub(crate) static CSRWI: Opcode = Opcode::new(Mnemonic::Csrwi, Codec::ICsr, fmt::CSR_ZIMM);
pub(crate) static CSRSI: Opcode = Opcode::new(Mnemonic::Csrsi, Codec::ICsr, fmt::CSR_ZIMM);
pub(crate) static CSRCI: Opcode = Opcode::new(Mnemonic::Csrci, Codec::ICsr, fmt::CSR_ZIMM);
pub(crate) static FSCSR_RS1: Opcode =
    Opcode::new(Mnemonic::Fscsr, Codec::ICsr, fmt::RS1).requires(&[Extension::F]);
pub(crate) static FSRM_RS1: Opcode =
    Opcode::new(Mnemonic::Fsrm, Codec::ICsr, fmt::RS1).requires(&[Extension::F]);
pub(crate) static FSFLAGS_RS1: Opcode =
    Opcode::new(Mnemonic::Fsflags, Codec::ICsr, fmt::RS1).requires(&[Extension::F]);
