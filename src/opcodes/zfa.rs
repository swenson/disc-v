// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zfa: additional floating-point instructions, for each of the S, D, H and Q
//! formats.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

/// The constants `fli` loads, indexed by its rs1 field, written as objdump
/// writes them (C hexadecimal floating point). `min` is the smallest
/// positive normal number of the format.
pub(crate) static FLI_CONSTANTS: [&str; 32] = [
    "-0x1p+0", "min", "0x1p-16", "0x1p-15", "0x1p-8", "0x1p-7", "0x1p-4", "0x1p-3", "0x1p-2",
    "0x1.4p-2", "0x1.8p-2", "0x1.cp-2", "0x1p-1", "0x1.4p-1", "0x1.8p-1", "0x1.cp-1", "0x1p+0",
    "0x1.4p+0", "0x1.8p+0", "0x1.cp+0", "0x1p+1", "0x1.4p+1", "0x1.8p+1", "0x1p+2", "0x1p+3",
    "0x1p+4", "0x1p+7", "0x1p+8", "0x1p+15", "0x1p+16", "inf", "nan",
];

pub(crate) static FLI_S: Opcode =
    Opcode::new(Mnemonic::FliS, Codec::R, fmt::FRD_FLI).requires(&[Extension::Zfa]);
pub(crate) static FMINM_S: Opcode =
    Opcode::new(Mnemonic::FminmS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfa]);
pub(crate) static FMAXM_S: Opcode =
    Opcode::new(Mnemonic::FmaxmS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfa]);
pub(crate) static FROUND_S: Opcode =
    Opcode::new(Mnemonic::FroundS, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfa]);
pub(crate) static FROUNDNX_S: Opcode =
    Opcode::new(Mnemonic::FroundnxS, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfa]);
pub(crate) static FLEQ_S: Opcode =
    Opcode::new(Mnemonic::FleqS, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfa]);
pub(crate) static FLTQ_S: Opcode =
    Opcode::new(Mnemonic::FltqS, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfa]);
pub(crate) static FLI_D: Opcode =
    Opcode::new(Mnemonic::FliD, Codec::R, fmt::FRD_FLI).requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FMINM_D: Opcode = Opcode::new(Mnemonic::FminmD, Codec::R, fmt::FRD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FMAXM_D: Opcode = Opcode::new(Mnemonic::FmaxmD, Codec::R, fmt::FRD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FROUND_D: Opcode = Opcode::new(Mnemonic::FroundD, Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FROUNDNX_D: Opcode =
    Opcode::new(Mnemonic::FroundnxD, Codec::RM, fmt::RM_FRD_FRS1)
        .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FLEQ_D: Opcode = Opcode::new(Mnemonic::FleqD, Codec::R, fmt::RD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FLTQ_D: Opcode = Opcode::new(Mnemonic::FltqD, Codec::R, fmt::RD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FLI_H: Opcode =
    Opcode::new(Mnemonic::FliH, Codec::R, fmt::FRD_FLI).requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FMINM_H: Opcode = Opcode::new(Mnemonic::FminmH, Codec::R, fmt::FRD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FMAXM_H: Opcode = Opcode::new(Mnemonic::FmaxmH, Codec::R, fmt::FRD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FROUND_H: Opcode = Opcode::new(Mnemonic::FroundH, Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FROUNDNX_H: Opcode =
    Opcode::new(Mnemonic::FroundnxH, Codec::RM, fmt::RM_FRD_FRS1)
        .requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FLEQ_H: Opcode = Opcode::new(Mnemonic::FleqH, Codec::R, fmt::RD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FLTQ_H: Opcode = Opcode::new(Mnemonic::FltqH, Codec::R, fmt::RD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Zfh]);
pub(crate) static FLI_Q: Opcode =
    Opcode::new(Mnemonic::FliQ, Codec::R, fmt::FRD_FLI).requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FMINM_Q: Opcode = Opcode::new(Mnemonic::FminmQ, Codec::R, fmt::FRD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FMAXM_Q: Opcode = Opcode::new(Mnemonic::FmaxmQ, Codec::R, fmt::FRD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FROUND_Q: Opcode = Opcode::new(Mnemonic::FroundQ, Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FROUNDNX_Q: Opcode =
    Opcode::new(Mnemonic::FroundnxQ, Codec::RM, fmt::RM_FRD_FRS1)
        .requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FLEQ_Q: Opcode = Opcode::new(Mnemonic::FleqQ, Codec::R, fmt::RD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FLTQ_Q: Opcode = Opcode::new(Mnemonic::FltqQ, Codec::R, fmt::RD_FRS1_FRS2)
    .requires(&[Extension::Zfa, Extension::Q]);
pub(crate) static FCVTMOD_W_D: Opcode =
    Opcode::new(Mnemonic::FcvtmodWD, Codec::RM, fmt::RM_RD_FRS1)
        .requires(&[Extension::Zfa, Extension::D]);
pub(crate) static FMVH_X_D: Opcode = Opcode::new(Mnemonic::FmvhXD, Codec::R, fmt::RD_FRS1)
    .requires(&[Extension::Zfa, Extension::D])
    .rv32_only();
pub(crate) static FMVP_D_X: Opcode = Opcode::new(Mnemonic::FmvpDX, Codec::R, fmt::FRD_RS1_RS2)
    .requires(&[Extension::Zfa, Extension::D])
    .rv32_only();
pub(crate) static FMVH_X_Q: Opcode = Opcode::new(Mnemonic::FmvhXQ, Codec::R, fmt::RD_FRS1)
    .requires(&[Extension::Zfa, Extension::Q])
    .rv64_only();
pub(crate) static FMVP_Q_X: Opcode = Opcode::new(Mnemonic::FmvpQX, Codec::R, fmt::FRD_RS1_RS2)
    .requires(&[Extension::Zfa, Extension::Q])
    .rv64_only();
