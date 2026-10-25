// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "D" extension: double-precision floating point.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt, pseudo};
use crate::{Extension, Mnemonic};

pub(crate) static FLD: Opcode = Opcode::new(Mnemonic::Fld, Codec::I, fmt::FRD_OFFSET_RS1)
    .requires(&[Extension::D])
    .f_registers();
pub(crate) static FSD: Opcode = Opcode::new(Mnemonic::Fsd, Codec::S, fmt::FRS2_OFFSET_RS1)
    .requires(&[Extension::D])
    .f_registers();
pub(crate) static FMADD_D: Opcode =
    Opcode::new(Mnemonic::FmaddD, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::D]);
pub(crate) static FMSUB_D: Opcode =
    Opcode::new(Mnemonic::FmsubD, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::D]);
pub(crate) static FNMSUB_D: Opcode =
    Opcode::new(Mnemonic::FnmsubD, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::D]);
pub(crate) static FNMADD_D: Opcode =
    Opcode::new(Mnemonic::FnmaddD, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::D]);
pub(crate) static FADD_D: Opcode =
    Opcode::new(Mnemonic::FaddD, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FSUB_D: Opcode =
    Opcode::new(Mnemonic::FsubD, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FMUL_D: Opcode =
    Opcode::new(Mnemonic::FmulD, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FDIV_D: Opcode =
    Opcode::new(Mnemonic::FdivD, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FSGNJ_D: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FMV_D, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjD, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::D])
};
pub(crate) static FSGNJN_D: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FNEG_D, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjnD, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::D])
};
pub(crate) static FSGNJX_D: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FABS_D, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjxD, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::D])
};
pub(crate) static FMIN_D: Opcode =
    Opcode::new(Mnemonic::FminD, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FMAX_D: Opcode =
    Opcode::new(Mnemonic::FmaxD, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FCVT_S_D: Opcode =
    Opcode::new(Mnemonic::FcvtSD, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::D]);
pub(crate) static FCVT_D_S: Opcode =
    Opcode::new(Mnemonic::FcvtDS, Codec::RM, fmt::WIDEN_FRD_FRS1).requires(&[Extension::D]);
pub(crate) static FSQRT_D: Opcode =
    Opcode::new(Mnemonic::FsqrtD, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::D]);
pub(crate) static FLE_D: Opcode =
    Opcode::new(Mnemonic::FleD, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FLT_D: Opcode =
    Opcode::new(Mnemonic::FltD, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FEQ_D: Opcode =
    Opcode::new(Mnemonic::FeqD, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::D]);
pub(crate) static FCVT_W_D: Opcode =
    Opcode::new(Mnemonic::FcvtWD, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::D]);
pub(crate) static FCVT_WU_D: Opcode =
    Opcode::new(Mnemonic::FcvtWuD, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::D]);
pub(crate) static FCVT_D_W: Opcode =
    Opcode::new(Mnemonic::FcvtDW, Codec::RM, fmt::WIDEN_FRD_RS1).requires(&[Extension::D]);
pub(crate) static FCVT_D_WU: Opcode =
    Opcode::new(Mnemonic::FcvtDWu, Codec::RM, fmt::WIDEN_FRD_RS1).requires(&[Extension::D]);
pub(crate) static FCLASS_D: Opcode =
    Opcode::new(Mnemonic::FclassD, Codec::R, fmt::RD_FRS1).requires(&[Extension::D]);
pub(crate) static FCVT_L_D: Opcode = Opcode::new(Mnemonic::FcvtLD, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::D])
    .rv64();
pub(crate) static FCVT_LU_D: Opcode = Opcode::new(Mnemonic::FcvtLuD, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::D])
    .rv64();
pub(crate) static FMV_X_D: Opcode = Opcode::new(Mnemonic::FmvXD, Codec::R, fmt::RD_FRS1)
    .requires(&[Extension::D])
    .rv64()
    .f_registers();
pub(crate) static FCVT_D_L: Opcode = Opcode::new(Mnemonic::FcvtDL, Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::D])
    .rv64();
pub(crate) static FCVT_D_LU: Opcode = Opcode::new(Mnemonic::FcvtDLu, Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::D])
    .rv64();
pub(crate) static FMV_D_X: Opcode = Opcode::new(Mnemonic::FmvDX, Codec::R, fmt::FRD_RS1)
    .requires(&[Extension::D])
    .rv64()
    .f_registers();
