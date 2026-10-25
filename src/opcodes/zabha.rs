// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Byte and halfword atomics (Zabha) and compare-and-swap (Zacas).

use super::Constraint::*;
use super::{Codec, Extra, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static AMOSWAP_B: Opcode =
    Opcode::new(Mnemonic::AmoswapB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOSWAP_H: Opcode =
    Opcode::new(Mnemonic::AmoswapH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOADD_B: Opcode =
    Opcode::new(Mnemonic::AmoaddB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOADD_H: Opcode =
    Opcode::new(Mnemonic::AmoaddH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOXOR_B: Opcode =
    Opcode::new(Mnemonic::AmoxorB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOXOR_H: Opcode =
    Opcode::new(Mnemonic::AmoxorH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOAND_B: Opcode =
    Opcode::new(Mnemonic::AmoandB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOAND_H: Opcode =
    Opcode::new(Mnemonic::AmoandH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOOR_B: Opcode =
    Opcode::new(Mnemonic::AmoorB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOOR_H: Opcode =
    Opcode::new(Mnemonic::AmoorH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMIN_B: Opcode =
    Opcode::new(Mnemonic::AmominB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMIN_H: Opcode =
    Opcode::new(Mnemonic::AmominH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAX_B: Opcode =
    Opcode::new(Mnemonic::AmomaxB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAX_H: Opcode =
    Opcode::new(Mnemonic::AmomaxH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMINU_B: Opcode =
    Opcode::new(Mnemonic::AmominuB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMINU_H: Opcode =
    Opcode::new(Mnemonic::AmominuH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAXU_B: Opcode =
    Opcode::new(Mnemonic::AmomaxuB, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOMAXU_H: Opcode =
    Opcode::new(Mnemonic::AmomaxuH, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zabha]);
pub(crate) static AMOCAS_W: Opcode =
    Opcode::new(Mnemonic::AmocasW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zacas]);
pub(crate) static AMOCAS_D: Opcode =
    Opcode::new(Mnemonic::AmocasD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zacas])
        .rv64();
/// On RV32, `amocas.d` operates on even-odd register pairs.
pub(crate) static AMOCAS_D_RV32: Opcode = Opcode {
    extra: &Extra {
        illegal_if: &[RdOdd, Rs2Odd],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::AmocasD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zacas])
        .rv32_only()
};
/// `amocas.q` operates on even-odd register pairs.
pub(crate) static AMOCAS_Q: Opcode = Opcode {
    extra: &Extra {
        illegal_if: &[RdOdd, Rs2Odd],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::AmocasQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zacas])
        .rv64_only()
};
pub(crate) static AMOCAS_B: Opcode =
    Opcode::new(Mnemonic::AmocasB, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zabha, Extension::Zacas]);
pub(crate) static AMOCAS_H: Opcode =
    Opcode::new(Mnemonic::AmocasH, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zabha, Extension::Zacas]);
