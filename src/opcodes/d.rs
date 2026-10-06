//! "D" extension: double-precision floating point.

use super::Constraint::*;
use super::{fmt, pseudo, Codec, Opcode, Pseudo};

pub(crate) static FLD: Opcode = Opcode::new("fld", Codec::I, fmt::FRD_OFFSET_RS1);
pub(crate) static FSD: Opcode = Opcode::new("fsd", Codec::S, fmt::FRS2_OFFSET_RS1);
pub(crate) static FMADD_D: Opcode = Opcode::new("fmadd.d", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FMSUB_D: Opcode = Opcode::new("fmsub.d", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FNMSUB_D: Opcode =
    Opcode::new("fnmsub.d", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FNMADD_D: Opcode =
    Opcode::new("fnmadd.d", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FADD_D: Opcode = Opcode::new("fadd.d", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FSUB_D: Opcode = Opcode::new("fsub.d", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FMUL_D: Opcode = Opcode::new("fmul.d", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FDIV_D: Opcode = Opcode::new("fdiv.d", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FSGNJ_D: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FMV_D, &[Rs2EqRs1])],
    ..Opcode::new("fsgnj.d", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FSGNJN_D: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FNEG_D, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjn.d", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FSGNJX_D: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FABS_D, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjx.d", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FMIN_D: Opcode = Opcode::new("fmin.d", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAX_D: Opcode = Opcode::new("fmax.d", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FCVT_S_D: Opcode = Opcode::new("fcvt.s.d", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FCVT_D_S: Opcode = Opcode::new("fcvt.d.s", Codec::RM, fmt::WIDEN_FRD_FRS1);
pub(crate) static FSQRT_D: Opcode = Opcode::new("fsqrt.d", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLE_D: Opcode = Opcode::new("fle.d", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLT_D: Opcode = Opcode::new("flt.d", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FEQ_D: Opcode = Opcode::new("feq.d", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FCVT_W_D: Opcode = Opcode::new("fcvt.w.d", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FCVT_WU_D: Opcode = Opcode::new("fcvt.wu.d", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FCVT_D_W: Opcode = Opcode::new("fcvt.d.w", Codec::RM, fmt::WIDEN_FRD_RS1);
pub(crate) static FCVT_D_WU: Opcode = Opcode::new("fcvt.d.wu", Codec::RM, fmt::WIDEN_FRD_RS1);
pub(crate) static FCLASS_D: Opcode = Opcode::new("fclass.d", Codec::R, fmt::RD_FRS1);
pub(crate) static FCVT_L_D: Opcode = Opcode::new("fcvt.l.d", Codec::RM, fmt::RM_RD_FRS1).rv64();
pub(crate) static FCVT_LU_D: Opcode = Opcode::new("fcvt.lu.d", Codec::RM, fmt::RM_RD_FRS1).rv64();
pub(crate) static FMV_X_D: Opcode = Opcode::new("fmv.x.d", Codec::R, fmt::RD_FRS1).rv64();
pub(crate) static FCVT_D_L: Opcode = Opcode::new("fcvt.d.l", Codec::RM, fmt::RM_FRD_RS1).rv64();
pub(crate) static FCVT_D_LU: Opcode = Opcode::new("fcvt.d.lu", Codec::RM, fmt::RM_FRD_RS1).rv64();
pub(crate) static FMV_D_X: Opcode = Opcode::new("fmv.d.x", Codec::R, fmt::FRD_RS1).rv64();
