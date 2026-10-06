// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "Q" extension: quad-precision floating point.

use super::Constraint::*;
use super::{fmt, pseudo, Codec, Opcode, Pseudo};

pub(crate) static FLQ: Opcode = Opcode::new("flq", Codec::I, fmt::FRD_OFFSET_RS1);
pub(crate) static FSQ: Opcode = Opcode::new("fsq", Codec::S, fmt::FRS2_OFFSET_RS1);
pub(crate) static FMADD_Q: Opcode = Opcode::new("fmadd.q", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FMSUB_Q: Opcode = Opcode::new("fmsub.q", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FNMSUB_Q: Opcode =
    Opcode::new("fnmsub.q", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FNMADD_Q: Opcode =
    Opcode::new("fnmadd.q", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FADD_Q: Opcode = Opcode::new("fadd.q", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FSUB_Q: Opcode = Opcode::new("fsub.q", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FMUL_Q: Opcode = Opcode::new("fmul.q", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FDIV_Q: Opcode = Opcode::new("fdiv.q", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FSGNJ_Q: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FMV_Q, &[Rs2EqRs1])],
    ..Opcode::new("fsgnj.q", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FSGNJN_Q: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FNEG_Q, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjn.q", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FSGNJX_Q: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FABS_Q, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjx.q", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FMIN_Q: Opcode = Opcode::new("fmin.q", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAX_Q: Opcode = Opcode::new("fmax.q", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FCVT_S_Q: Opcode = Opcode::new("fcvt.s.q", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FCVT_Q_S: Opcode = Opcode::new("fcvt.q.s", Codec::RM, fmt::WIDEN_FRD_FRS1);
pub(crate) static FCVT_D_Q: Opcode = Opcode::new("fcvt.d.q", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FCVT_Q_D: Opcode = Opcode::new("fcvt.q.d", Codec::RM, fmt::WIDEN_FRD_FRS1);
pub(crate) static FSQRT_Q: Opcode = Opcode::new("fsqrt.q", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLE_Q: Opcode = Opcode::new("fle.q", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLT_Q: Opcode = Opcode::new("flt.q", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FEQ_Q: Opcode = Opcode::new("feq.q", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FCVT_W_Q: Opcode = Opcode::new("fcvt.w.q", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FCVT_WU_Q: Opcode = Opcode::new("fcvt.wu.q", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FCVT_Q_W: Opcode = Opcode::new("fcvt.q.w", Codec::RM, fmt::WIDEN_FRD_RS1);
pub(crate) static FCVT_Q_WU: Opcode = Opcode::new("fcvt.q.wu", Codec::RM, fmt::WIDEN_FRD_RS1);
pub(crate) static FCLASS_Q: Opcode = Opcode::new("fclass.q", Codec::R, fmt::RD_FRS1);
pub(crate) static FCVT_L_Q: Opcode = Opcode::new("fcvt.l.q", Codec::RM, fmt::RM_RD_FRS1).rv64();
pub(crate) static FCVT_LU_Q: Opcode = Opcode::new("fcvt.lu.q", Codec::RM, fmt::RM_RD_FRS1).rv64();
pub(crate) static FCVT_Q_L: Opcode = Opcode::new("fcvt.q.l", Codec::RM, fmt::WIDEN_FRD_RS1).rv64();
pub(crate) static FCVT_Q_LU: Opcode =
    Opcode::new("fcvt.q.lu", Codec::RM, fmt::WIDEN_FRD_RS1).rv64();
pub(crate) static FMV_X_Q: Opcode = Opcode::new("fmv.x.q", Codec::R, fmt::RD_FRS1).rv128();
pub(crate) static FMV_Q_X: Opcode = Opcode::new("fmv.q.x", Codec::R, fmt::FRD_RS1).rv128();
