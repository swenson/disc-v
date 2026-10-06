// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zfa: additional floating-point instructions, for each of the S, D, H and Q
//! formats.

use super::{Codec, Opcode, fmt};

/// The constants `fli` loads, indexed by its rs1 field, written as objdump
/// writes them (C hexadecimal floating point). `min` is the smallest
/// positive normal number of the format.
pub(crate) static FLI_CONSTANTS: [&str; 32] = [
    "-0x1p+0", "min", "0x1p-16", "0x1p-15", "0x1p-8", "0x1p-7", "0x1p-4", "0x1p-3", "0x1p-2",
    "0x1.4p-2", "0x1.8p-2", "0x1.cp-2", "0x1p-1", "0x1.4p-1", "0x1.8p-1", "0x1.cp-1", "0x1p+0",
    "0x1.4p+0", "0x1.8p+0", "0x1.cp+0", "0x1p+1", "0x1.4p+1", "0x1.8p+1", "0x1p+2", "0x1p+3",
    "0x1p+4", "0x1p+7", "0x1p+8", "0x1p+15", "0x1p+16", "inf", "nan",
];

pub(crate) static FLI_S: Opcode = Opcode::new("fli.s", Codec::R, fmt::FRD_FLI);
pub(crate) static FMINM_S: Opcode = Opcode::new("fminm.s", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAXM_S: Opcode = Opcode::new("fmaxm.s", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FROUND_S: Opcode = Opcode::new("fround.s", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FROUNDNX_S: Opcode = Opcode::new("froundnx.s", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLEQ_S: Opcode = Opcode::new("fleq.s", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLTQ_S: Opcode = Opcode::new("fltq.s", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLI_D: Opcode = Opcode::new("fli.d", Codec::R, fmt::FRD_FLI);
pub(crate) static FMINM_D: Opcode = Opcode::new("fminm.d", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAXM_D: Opcode = Opcode::new("fmaxm.d", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FROUND_D: Opcode = Opcode::new("fround.d", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FROUNDNX_D: Opcode = Opcode::new("froundnx.d", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLEQ_D: Opcode = Opcode::new("fleq.d", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLTQ_D: Opcode = Opcode::new("fltq.d", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLI_H: Opcode = Opcode::new("fli.h", Codec::R, fmt::FRD_FLI);
pub(crate) static FMINM_H: Opcode = Opcode::new("fminm.h", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAXM_H: Opcode = Opcode::new("fmaxm.h", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FROUND_H: Opcode = Opcode::new("fround.h", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FROUNDNX_H: Opcode = Opcode::new("froundnx.h", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLEQ_H: Opcode = Opcode::new("fleq.h", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLTQ_H: Opcode = Opcode::new("fltq.h", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLI_Q: Opcode = Opcode::new("fli.q", Codec::R, fmt::FRD_FLI);
pub(crate) static FMINM_Q: Opcode = Opcode::new("fminm.q", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAXM_Q: Opcode = Opcode::new("fmaxm.q", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FROUND_Q: Opcode = Opcode::new("fround.q", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FROUNDNX_Q: Opcode = Opcode::new("froundnx.q", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLEQ_Q: Opcode = Opcode::new("fleq.q", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLTQ_Q: Opcode = Opcode::new("fltq.q", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FCVTMOD_W_D: Opcode = Opcode::new("fcvtmod.w.d", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FMVH_X_D: Opcode = Opcode::new("fmvh.x.d", Codec::R, fmt::RD_FRS1).rv32_only();
pub(crate) static FMVP_D_X: Opcode =
    Opcode::new("fmvp.d.x", Codec::R, fmt::FRD_RS1_RS2).rv32_only();
pub(crate) static FMVH_X_Q: Opcode = Opcode::new("fmvh.x.q", Codec::R, fmt::RD_FRS1).rv64_only();
pub(crate) static FMVP_Q_X: Opcode =
    Opcode::new("fmvp.q.x", Codec::R, fmt::FRD_RS1_RS2).rv64_only();
