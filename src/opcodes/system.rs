//! Environment calls, trap returns, fences for virtual memory, and Zicsr.

use super::Constraint::*;
use super::{fmt, pseudo, Codec, Opcode, Pseudo};
use crate::reg;

pub(crate) static ECALL: Opcode = Opcode::new("ecall", Codec::None, fmt::NONE);
pub(crate) static EBREAK: Opcode = Opcode::new("ebreak", Codec::None, fmt::NONE);
pub(crate) static URET: Opcode = Opcode::new("uret", Codec::None, fmt::NONE);
pub(crate) static SRET: Opcode = Opcode::new("sret", Codec::None, fmt::NONE);
pub(crate) static HRET: Opcode = Opcode::new("hret", Codec::None, fmt::NONE);
pub(crate) static MRET: Opcode = Opcode::new("mret", Codec::None, fmt::NONE);
pub(crate) static DRET: Opcode = Opcode::new("dret", Codec::None, fmt::NONE);
pub(crate) static SFENCE_VM: Opcode = Opcode::new("sfence.vm", Codec::R, fmt::RS1);
pub(crate) static SFENCE_VMA: Opcode = Opcode::new("sfence.vma", Codec::R, fmt::RS1_RS2);
pub(crate) static WFI: Opcode = Opcode::new("wfi", Codec::None, fmt::NONE);
pub(crate) static CSRRW: Opcode = Opcode {
    pseudo: &[
        Pseudo {
            op: &pseudo::FSCSR,
            when: &[CsrEq(0x003)],
        },
        Pseudo {
            op: &pseudo::FSRM,
            when: &[CsrEq(0x002)],
        },
        Pseudo {
            op: &pseudo::FSFLAGS,
            when: &[CsrEq(0x001)],
        },
    ],
    ..Opcode::new("csrrw", Codec::ICsr, fmt::RD_CSR_RS1)
};
pub(crate) static CSRRS: Opcode = Opcode {
    pseudo: &[
        Pseudo {
            op: &pseudo::RDCYCLE,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0xc00)],
        },
        Pseudo {
            op: &pseudo::RDTIME,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0xc01)],
        },
        Pseudo {
            op: &pseudo::RDINSTRET,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0xc02)],
        },
        Pseudo {
            op: &pseudo::RDCYCLEH,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0xc80)],
        },
        Pseudo {
            op: &pseudo::RDTIMEH,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0xc81)],
        },
        Pseudo {
            op: &pseudo::RDINSTRETH,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0xc82)],
        },
        Pseudo {
            op: &pseudo::FRCSR,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0x003)],
        },
        Pseudo {
            op: &pseudo::FRRM,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0x002)],
        },
        Pseudo {
            op: &pseudo::FRFLAGS,
            when: &[Rs1Eq(reg::ZERO), CsrEq(0x001)],
        },
    ],
    ..Opcode::new("csrrs", Codec::ICsr, fmt::RD_CSR_RS1)
};
pub(crate) static CSRRC: Opcode = Opcode::new("csrrc", Codec::ICsr, fmt::RD_CSR_RS1);
pub(crate) static CSRRWI: Opcode = Opcode {
    pseudo: &[
        Pseudo {
            op: &pseudo::FSRMI,
            when: &[CsrEq(0x002)],
        },
        Pseudo {
            op: &pseudo::FSFLAGSI,
            when: &[CsrEq(0x001)],
        },
    ],
    ..Opcode::new("csrrwi", Codec::ICsr, fmt::RD_CSR_ZIMM)
};
pub(crate) static CSRRSI: Opcode = Opcode::new("csrrsi", Codec::ICsr, fmt::RD_CSR_ZIMM);
pub(crate) static CSRRCI: Opcode = Opcode::new("csrrci", Codec::ICsr, fmt::RD_CSR_ZIMM);
