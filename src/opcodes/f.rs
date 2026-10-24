// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "F" extension: single-precision floating point.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt, pseudo};
use crate::{Extension, Mnemonic};

pub(crate) static FLW: Opcode = Opcode::new(Mnemonic::Flw, Codec::I, fmt::FRD_OFFSET_RS1)
    .requires(&[Extension::F])
    .f_registers();
pub(crate) static FSW: Opcode = Opcode::new(Mnemonic::Fsw, Codec::S, fmt::FRS2_OFFSET_RS1)
    .requires(&[Extension::F])
    .f_registers();
pub(crate) static FMADD_S: Opcode =
    Opcode::new(Mnemonic::FmaddS, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::F]);
pub(crate) static FMSUB_S: Opcode =
    Opcode::new(Mnemonic::FmsubS, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::F]);
pub(crate) static FNMSUB_S: Opcode =
    Opcode::new(Mnemonic::FnmsubS, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::F]);
pub(crate) static FNMADD_S: Opcode =
    Opcode::new(Mnemonic::FnmaddS, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::F]);
pub(crate) static FADD_S: Opcode =
    Opcode::new(Mnemonic::FaddS, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FSUB_S: Opcode =
    Opcode::new(Mnemonic::FsubS, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FMUL_S: Opcode =
    Opcode::new(Mnemonic::FmulS, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FDIV_S: Opcode =
    Opcode::new(Mnemonic::FdivS, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FSGNJ_S: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FMV_S, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::F])
};
pub(crate) static FSGNJN_S: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FNEG_S, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjnS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::F])
};
pub(crate) static FSGNJX_S: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FABS_S, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjxS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::F])
};
pub(crate) static FMIN_S: Opcode =
    Opcode::new(Mnemonic::FminS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FMAX_S: Opcode =
    Opcode::new(Mnemonic::FmaxS, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FSQRT_S: Opcode =
    Opcode::new(Mnemonic::FsqrtS, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::F]);
pub(crate) static FLE_S: Opcode =
    Opcode::new(Mnemonic::FleS, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FLT_S: Opcode =
    Opcode::new(Mnemonic::FltS, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FEQ_S: Opcode =
    Opcode::new(Mnemonic::FeqS, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::F]);
pub(crate) static FCVT_W_S: Opcode =
    Opcode::new(Mnemonic::FcvtWS, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::F]);
pub(crate) static FCVT_WU_S: Opcode =
    Opcode::new(Mnemonic::FcvtWuS, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::F]);
pub(crate) static FCVT_S_W: Opcode =
    Opcode::new(Mnemonic::FcvtSW, Codec::RM, fmt::RM_FRD_RS1).requires(&[Extension::F]);
pub(crate) static FCVT_S_WU: Opcode =
    Opcode::new(Mnemonic::FcvtSWu, Codec::RM, fmt::RM_FRD_RS1).requires(&[Extension::F]);
pub(crate) static FMV_X_W: Opcode = Opcode::new(Mnemonic::FmvXW, Codec::R, fmt::RD_FRS1)
    .requires(&[Extension::F])
    .f_registers();
pub(crate) static FCLASS_S: Opcode =
    Opcode::new(Mnemonic::FclassS, Codec::R, fmt::RD_FRS1).requires(&[Extension::F]);
pub(crate) static FMV_W_X: Opcode = Opcode::new(Mnemonic::FmvWX, Codec::R, fmt::FRD_RS1)
    .requires(&[Extension::F])
    .f_registers();
pub(crate) static FCVT_L_S: Opcode = Opcode::new(Mnemonic::FcvtLS, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::F])
    .rv64();
pub(crate) static FCVT_LU_S: Opcode = Opcode::new(Mnemonic::FcvtLuS, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::F])
    .rv64();
pub(crate) static FCVT_S_L: Opcode = Opcode::new(Mnemonic::FcvtSL, Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::F])
    .rv64();
pub(crate) static FCVT_S_LU: Opcode = Opcode::new(Mnemonic::FcvtSLu, Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::F])
    .rv64();
