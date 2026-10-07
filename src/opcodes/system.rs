// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Environment calls, trap returns, fences for virtual memory, and Zicsr.

use super::Constraint::*;
use super::{Codec, Opcode, Pseudo, fmt, pseudo};
use crate::Extension;
use crate::reg;

pub(crate) static ECALL: Opcode = Opcode::new("ecall", Codec::None, fmt::NONE);
pub(crate) static EBREAK: Opcode = Opcode::new("ebreak", Codec::None, fmt::NONE);
pub(crate) static SRET: Opcode = Opcode::new("sret", Codec::None, fmt::NONE);
pub(crate) static MRET: Opcode = Opcode::new("mret", Codec::None, fmt::NONE);
pub(crate) static DRET: Opcode =
    Opcode::new("dret", Codec::None, fmt::NONE).requires(&[Extension::Sdext]);
pub(crate) static SFENCE_VMA: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(
            &pseudo::SFENCE_VMA_ALL,
            &[Rs1Eq(reg::ZERO), Rs2Eq(reg::ZERO)],
        ),
        Pseudo::new(&pseudo::SFENCE_VMA_RS1, &[Rs2Eq(reg::ZERO)]),
    ],
    ..Opcode::new("sfence.vma", Codec::R, fmt::RS1_RS2)
};
pub(crate) static WFI: Opcode = Opcode::new("wfi", Codec::None, fmt::NONE);
// Svinval.
pub(crate) static SINVAL_VMA: Opcode =
    Opcode::new("sinval.vma", Codec::R, fmt::RS1_RS2).requires(&[Extension::Svinval]);
pub(crate) static SFENCE_W_INVAL: Opcode =
    Opcode::new("sfence.w.inval", Codec::None, fmt::NONE).requires(&[Extension::Svinval]);
pub(crate) static SFENCE_INVAL_IR: Opcode =
    Opcode::new("sfence.inval.ir", Codec::None, fmt::NONE).requires(&[Extension::Svinval]);
// Smrnmi.
pub(crate) static MNRET: Opcode =
    Opcode::new("mnret", Codec::None, fmt::NONE).requires(&[Extension::Smrnmi]);
// Ssctr.
pub(crate) static SCTRCLR: Opcode =
    Opcode::new("sctrclr", Codec::None, fmt::NONE).requires(&[Extension::Ssctr]);
pub(crate) static CSRRW: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(
            &pseudo::UNIMP,
            &[RdEq(reg::ZERO), CsrEq(0xc00), Rs1Eq(reg::ZERO)],
        ),
        Pseudo::new(&pseudo::FSCSR_RS1, &[RdEq(reg::ZERO), CsrEq(0x003)]),
        Pseudo::new(&pseudo::FSCSR, &[CsrEq(0x003)]),
        Pseudo::new(&pseudo::FSRM_RS1, &[RdEq(reg::ZERO), CsrEq(0x002)]),
        Pseudo::new(&pseudo::FSRM, &[CsrEq(0x002)]),
        Pseudo::new(&pseudo::FSFLAGS_RS1, &[RdEq(reg::ZERO), CsrEq(0x001)]),
        Pseudo::new(&pseudo::FSFLAGS, &[CsrEq(0x001)]),
        Pseudo::new(&pseudo::CSRW, &[RdEq(reg::ZERO)]),
    ],
    ..Opcode::new("csrrw", Codec::ICsr, fmt::RD_CSR_RS1).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRS: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::RDCYCLE, &[Rs1Eq(reg::ZERO), CsrEq(0xc00)]),
        Pseudo::new(&pseudo::RDTIME, &[Rs1Eq(reg::ZERO), CsrEq(0xc01)]),
        Pseudo::new(&pseudo::RDINSTRET, &[Rs1Eq(reg::ZERO), CsrEq(0xc02)]),
        Pseudo::new(&pseudo::RDCYCLEH, &[Rs1Eq(reg::ZERO), CsrEq(0xc80)]),
        Pseudo::new(&pseudo::RDTIMEH, &[Rs1Eq(reg::ZERO), CsrEq(0xc81)]),
        Pseudo::new(&pseudo::RDINSTRETH, &[Rs1Eq(reg::ZERO), CsrEq(0xc82)]),
        Pseudo::new(&pseudo::FRCSR, &[Rs1Eq(reg::ZERO), CsrEq(0x003)]),
        Pseudo::new(&pseudo::FRRM, &[Rs1Eq(reg::ZERO), CsrEq(0x002)]),
        Pseudo::new(&pseudo::FRFLAGS, &[Rs1Eq(reg::ZERO), CsrEq(0x001)]),
        Pseudo::new(&pseudo::CSRR, &[Rs1Eq(reg::ZERO)]),
        Pseudo::new(&pseudo::CSRS, &[RdEq(reg::ZERO)]),
    ],
    ..Opcode::new("csrrs", Codec::ICsr, fmt::RD_CSR_RS1).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRC: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::CSRC, &[RdEq(reg::ZERO)])],
    ..Opcode::new("csrrc", Codec::ICsr, fmt::RD_CSR_RS1).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRWI: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&pseudo::FSRMI, &[CsrEq(0x002)]),
        Pseudo::new(&pseudo::FSFLAGSI, &[CsrEq(0x001)]),
        Pseudo::new(&pseudo::CSRWI, &[RdEq(reg::ZERO)]),
    ],
    ..Opcode::new("csrrwi", Codec::ICsr, fmt::RD_CSR_ZIMM).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRSI: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::CSRSI, &[RdEq(reg::ZERO)])],
    ..Opcode::new("csrrsi", Codec::ICsr, fmt::RD_CSR_ZIMM).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRCI: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::CSRCI, &[RdEq(reg::ZERO)])],
    ..Opcode::new("csrrci", Codec::ICsr, fmt::RD_CSR_ZIMM).requires(&[Extension::Zicsr])
};
