// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zfh and Zfhmin: half-precision floating point.
//!
//! Zfhmin is the loads, stores, moves and conversions; Zfh adds the rest.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt, pseudo};
use crate::{Extension, Mnemonic};

pub(crate) static FLH: Opcode = Opcode::new(Mnemonic::Flh, Codec::I, fmt::FRD_OFFSET_RS1)
    .requires(&[Extension::Zfhmin])
    .f_registers();
pub(crate) static FSH: Opcode = Opcode::new(Mnemonic::Fsh, Codec::S, fmt::FRS2_OFFSET_RS1)
    .requires(&[Extension::Zfhmin])
    .f_registers();
pub(crate) static FMADD_H: Opcode =
    Opcode::new(Mnemonic::FmaddH, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::Zfh]);
pub(crate) static FMSUB_H: Opcode =
    Opcode::new(Mnemonic::FmsubH, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::Zfh]);
pub(crate) static FNMSUB_H: Opcode =
    Opcode::new(Mnemonic::FnmsubH, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::Zfh]);
pub(crate) static FNMADD_H: Opcode =
    Opcode::new(Mnemonic::FnmaddH, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::Zfh]);
pub(crate) static FADD_H: Opcode =
    Opcode::new(Mnemonic::FaddH, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FSUB_H: Opcode =
    Opcode::new(Mnemonic::FsubH, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FMUL_H: Opcode =
    Opcode::new(Mnemonic::FmulH, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FDIV_H: Opcode =
    Opcode::new(Mnemonic::FdivH, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FSGNJ_H: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FMV_H, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjH, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh])
};
pub(crate) static FSGNJN_H: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FNEG_H, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjnH, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh])
};
pub(crate) static FSGNJX_H: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FABS_H, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjxH, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh])
};
pub(crate) static FMIN_H: Opcode =
    Opcode::new(Mnemonic::FminH, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FMAX_H: Opcode =
    Opcode::new(Mnemonic::FmaxH, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FSQRT_H: Opcode =
    Opcode::new(Mnemonic::FsqrtH, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FLE_H: Opcode =
    Opcode::new(Mnemonic::FleH, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FLT_H: Opcode =
    Opcode::new(Mnemonic::FltH, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FEQ_H: Opcode =
    Opcode::new(Mnemonic::FeqH, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FCVT_W_H: Opcode =
    Opcode::new(Mnemonic::FcvtWH, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FCVT_WU_H: Opcode =
    Opcode::new(Mnemonic::FcvtWuH, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FCVT_H_W: Opcode =
    Opcode::new(Mnemonic::FcvtHW, Codec::RM, fmt::RM_FRD_RS1).requires(&[Extension::Zfh]);
pub(crate) static FCVT_H_WU: Opcode =
    Opcode::new(Mnemonic::FcvtHWu, Codec::RM, fmt::RM_FRD_RS1).requires(&[Extension::Zfh]);
pub(crate) static FMV_X_H: Opcode = Opcode::new(Mnemonic::FmvXH, Codec::R, fmt::RD_FRS1)
    .requires(&[Extension::Zfhmin])
    .f_registers();
pub(crate) static FCLASS_H: Opcode =
    Opcode::new(Mnemonic::FclassH, Codec::R, fmt::RD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FMV_H_X: Opcode = Opcode::new(Mnemonic::FmvHX, Codec::R, fmt::FRD_RS1)
    .requires(&[Extension::Zfhmin])
    .f_registers();
pub(crate) static FCVT_L_H: Opcode = Opcode::new(Mnemonic::FcvtLH, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_LU_H: Opcode = Opcode::new(Mnemonic::FcvtLuH, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_H_L: Opcode = Opcode::new(Mnemonic::FcvtHL, Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_H_LU: Opcode = Opcode::new(Mnemonic::FcvtHLu, Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_S_H: Opcode =
    Opcode::new(Mnemonic::FcvtSH, Codec::RM, fmt::WIDEN_FRD_FRS1).requires(&[Extension::Zfhmin]);
pub(crate) static FCVT_H_S: Opcode =
    Opcode::new(Mnemonic::FcvtHS, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfhmin]);
pub(crate) static FCVT_D_H: Opcode = Opcode::new(Mnemonic::FcvtDH, Codec::RM, fmt::WIDEN_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::D]);
pub(crate) static FCVT_H_D: Opcode = Opcode::new(Mnemonic::FcvtHD, Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::D]);
pub(crate) static FCVT_Q_H: Opcode = Opcode::new(Mnemonic::FcvtQH, Codec::RM, fmt::WIDEN_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::Q]);
pub(crate) static FCVT_H_Q: Opcode = Opcode::new(Mnemonic::FcvtHQ, Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::Q]);
