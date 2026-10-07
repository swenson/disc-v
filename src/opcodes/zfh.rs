// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zfh and Zfhmin: half-precision floating point.
//!
//! Zfhmin is the loads, stores, moves and conversions; Zfh adds the rest.

use super::Constraint::*;
use super::{Codec, Opcode, Pseudo, fmt, pseudo};
use crate::Extension;

pub(crate) static FLH: Opcode =
    Opcode::new("flh", Codec::I, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zfhmin]);
pub(crate) static FSH: Opcode =
    Opcode::new("fsh", Codec::S, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zfhmin]);
pub(crate) static FMADD_H: Opcode =
    Opcode::new("fmadd.h", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::Zfh]);
pub(crate) static FMSUB_H: Opcode =
    Opcode::new("fmsub.h", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::Zfh]);
pub(crate) static FNMSUB_H: Opcode =
    Opcode::new("fnmsub.h", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::Zfh]);
pub(crate) static FNMADD_H: Opcode =
    Opcode::new("fnmadd.h", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::Zfh]);
pub(crate) static FADD_H: Opcode =
    Opcode::new("fadd.h", Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FSUB_H: Opcode =
    Opcode::new("fsub.h", Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FMUL_H: Opcode =
    Opcode::new("fmul.h", Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FDIV_H: Opcode =
    Opcode::new("fdiv.h", Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FSGNJ_H: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FMV_H, &[Rs2EqRs1])],
    ..Opcode::new("fsgnj.h", Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh])
};
pub(crate) static FSGNJN_H: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FNEG_H, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjn.h", Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh])
};
pub(crate) static FSGNJX_H: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FABS_H, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjx.h", Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh])
};
pub(crate) static FMIN_H: Opcode =
    Opcode::new("fmin.h", Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FMAX_H: Opcode =
    Opcode::new("fmax.h", Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FSQRT_H: Opcode =
    Opcode::new("fsqrt.h", Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FLE_H: Opcode =
    Opcode::new("fle.h", Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FLT_H: Opcode =
    Opcode::new("flt.h", Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FEQ_H: Opcode =
    Opcode::new("feq.h", Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Zfh]);
pub(crate) static FCVT_W_H: Opcode =
    Opcode::new("fcvt.w.h", Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FCVT_WU_H: Opcode =
    Opcode::new("fcvt.wu.h", Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FCVT_H_W: Opcode =
    Opcode::new("fcvt.h.w", Codec::RM, fmt::RM_FRD_RS1).requires(&[Extension::Zfh]);
pub(crate) static FCVT_H_WU: Opcode =
    Opcode::new("fcvt.h.wu", Codec::RM, fmt::RM_FRD_RS1).requires(&[Extension::Zfh]);
pub(crate) static FMV_X_H: Opcode =
    Opcode::new("fmv.x.h", Codec::R, fmt::RD_FRS1).requires(&[Extension::Zfhmin]);
pub(crate) static FCLASS_H: Opcode =
    Opcode::new("fclass.h", Codec::R, fmt::RD_FRS1).requires(&[Extension::Zfh]);
pub(crate) static FMV_H_X: Opcode =
    Opcode::new("fmv.h.x", Codec::R, fmt::FRD_RS1).requires(&[Extension::Zfhmin]);
pub(crate) static FCVT_L_H: Opcode = Opcode::new("fcvt.l.h", Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_LU_H: Opcode = Opcode::new("fcvt.lu.h", Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_H_L: Opcode = Opcode::new("fcvt.h.l", Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_H_LU: Opcode = Opcode::new("fcvt.h.lu", Codec::RM, fmt::RM_FRD_RS1)
    .requires(&[Extension::Zfh])
    .rv64();
pub(crate) static FCVT_S_H: Opcode =
    Opcode::new("fcvt.s.h", Codec::RM, fmt::WIDEN_FRD_FRS1).requires(&[Extension::Zfhmin]);
pub(crate) static FCVT_H_S: Opcode =
    Opcode::new("fcvt.h.s", Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfhmin]);
pub(crate) static FCVT_D_H: Opcode = Opcode::new("fcvt.d.h", Codec::RM, fmt::WIDEN_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::D]);
pub(crate) static FCVT_H_D: Opcode = Opcode::new("fcvt.h.d", Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::D]);
pub(crate) static FCVT_Q_H: Opcode = Opcode::new("fcvt.q.h", Codec::RM, fmt::WIDEN_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::Q]);
pub(crate) static FCVT_H_Q: Opcode = Opcode::new("fcvt.h.q", Codec::RM, fmt::RM_FRD_FRS1)
    .requires(&[Extension::Zfhmin, Extension::Q]);
