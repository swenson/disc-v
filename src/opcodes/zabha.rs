// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Byte and halfword atomics (Zabha) and compare-and-swap (Zacas).

use super::Constraint::*;
use super::{Codec, Opcode, fmt};
use crate::Extension;

pub(crate) static AMOSWAP_B: Opcode =
    Opcode::new("amoswap.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOSWAP_H: Opcode =
    Opcode::new("amoswap.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOADD_B: Opcode =
    Opcode::new("amoadd.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOADD_H: Opcode =
    Opcode::new("amoadd.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOXOR_B: Opcode =
    Opcode::new("amoxor.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOXOR_H: Opcode =
    Opcode::new("amoxor.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOAND_B: Opcode =
    Opcode::new("amoand.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOAND_H: Opcode =
    Opcode::new("amoand.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOOR_B: Opcode =
    Opcode::new("amoor.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOOR_H: Opcode =
    Opcode::new("amoor.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMIN_B: Opcode =
    Opcode::new("amomin.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMIN_H: Opcode =
    Opcode::new("amomin.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAX_B: Opcode =
    Opcode::new("amomax.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAX_H: Opcode =
    Opcode::new("amomax.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMINU_B: Opcode =
    Opcode::new("amominu.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMINU_H: Opcode =
    Opcode::new("amominu.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAXU_B: Opcode =
    Opcode::new("amomaxu.b", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAXU_H: Opcode =
    Opcode::new("amomaxu.h", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOCAS_W: Opcode =
    Opcode::new("amocas.w", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zacas]);
pub(crate) static AMOCAS_D: Opcode = Opcode::new("amocas.d", Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::Zacas])
    .rv64();
/// On RV32, `amocas.d` operates on even-odd register pairs.
pub(crate) static AMOCAS_D_RV32: Opcode = Opcode {
    illegal_if: &[RdOdd, Rs2Odd],
    ..Opcode::new("amocas.d", Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zacas])
        .rv32_only()
};
/// `amocas.q` operates on even-odd register pairs.
pub(crate) static AMOCAS_Q: Opcode = Opcode {
    illegal_if: &[RdOdd, Rs2Odd],
    ..Opcode::new("amocas.q", Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zacas])
        .rv64_only()
};
pub(crate) static AMOCAS_B: Opcode = Opcode::new("amocas.b", Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::Zabha, Extension::Zacas]);
pub(crate) static AMOCAS_H: Opcode = Opcode::new("amocas.h", Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::Zabha, Extension::Zacas]);
