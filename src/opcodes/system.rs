// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Environment calls, trap returns, fences for virtual memory, and Zicsr.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt, pseudo};
use crate::reg;
use crate::{Extension, Mnemonic};

pub(crate) static ECALL: Opcode = Opcode::new(Mnemonic::Ecall, Codec::None, fmt::NONE);
pub(crate) static EBREAK: Opcode = Opcode::new(Mnemonic::Ebreak, Codec::None, fmt::NONE);
pub(crate) static SRET: Opcode = Opcode::new(Mnemonic::Sret, Codec::None, fmt::NONE);
pub(crate) static MRET: Opcode = Opcode::new(Mnemonic::Mret, Codec::None, fmt::NONE);
pub(crate) static DRET: Opcode =
    Opcode::new(Mnemonic::Dret, Codec::None, fmt::NONE).requires(&[Extension::Sdext]);
pub(crate) static SFENCE_VMA: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(
                &pseudo::SFENCE_VMA_ALL,
                &[Rs1Eq(reg::ZERO), Rs2Eq(reg::ZERO)],
            ),
            Pseudo::new(&pseudo::SFENCE_VMA_RS1, &[Rs2Eq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::SfenceVma, Codec::R, fmt::RS1_RS2)
};
pub(crate) static WFI: Opcode = Opcode::new(Mnemonic::Wfi, Codec::None, fmt::NONE);
// Svinval.
pub(crate) static SINVAL_VMA: Opcode =
    Opcode::new(Mnemonic::SinvalVma, Codec::R, fmt::RS1_RS2).requires(&[Extension::Svinval]);
pub(crate) static SFENCE_W_INVAL: Opcode =
    Opcode::new(Mnemonic::SfenceWInval, Codec::None, fmt::NONE).requires(&[Extension::Svinval]);
pub(crate) static SFENCE_INVAL_IR: Opcode =
    Opcode::new(Mnemonic::SfenceInvalIr, Codec::None, fmt::NONE).requires(&[Extension::Svinval]);
// Smrnmi.
pub(crate) static MNRET: Opcode =
    Opcode::new(Mnemonic::Mnret, Codec::None, fmt::NONE).requires(&[Extension::Smrnmi]);
// Ssctr.
pub(crate) static SCTRCLR: Opcode =
    Opcode::new(Mnemonic::Sctrclr, Codec::None, fmt::NONE).requires(&[Extension::Ssctr]);
pub(crate) static CSRRW: Opcode = Opcode {
    extra: &Extra {
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
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Csrrw, Codec::ICsr, fmt::RD_CSR_RS1).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRS: Opcode = Opcode {
    extra: &Extra {
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
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Csrrs, Codec::ICsr, fmt::RD_CSR_RS1).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRC: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::CSRC, &[RdEq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Csrrc, Codec::ICsr, fmt::RD_CSR_RS1).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRWI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&pseudo::FSRMI, &[CsrEq(0x002)]),
            Pseudo::new(&pseudo::FSFLAGSI, &[CsrEq(0x001)]),
            Pseudo::new(&pseudo::CSRWI, &[RdEq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Csrrwi, Codec::ICsr, fmt::RD_CSR_ZIMM).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRSI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::CSRSI, &[RdEq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Csrrsi, Codec::ICsr, fmt::RD_CSR_ZIMM).requires(&[Extension::Zicsr])
};
pub(crate) static CSRRCI: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::CSRCI, &[RdEq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Csrrci, Codec::ICsr, fmt::RD_CSR_ZIMM).requires(&[Extension::Zicsr])
};
