// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "F" extension: single-precision floating point.

use super::Constraint::*;
use super::{Codec, Opcode, Pseudo, fmt, pseudo};

pub(crate) static FLW: Opcode = Opcode::new("flw", Codec::I, fmt::FRD_OFFSET_RS1);
pub(crate) static FSW: Opcode = Opcode::new("fsw", Codec::S, fmt::FRS2_OFFSET_RS1);
pub(crate) static FMADD_S: Opcode = Opcode::new("fmadd.s", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FMSUB_S: Opcode = Opcode::new("fmsub.s", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FNMSUB_S: Opcode =
    Opcode::new("fnmsub.s", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FNMADD_S: Opcode =
    Opcode::new("fnmadd.s", Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3);
pub(crate) static FADD_S: Opcode = Opcode::new("fadd.s", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FSUB_S: Opcode = Opcode::new("fsub.s", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FMUL_S: Opcode = Opcode::new("fmul.s", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FDIV_S: Opcode = Opcode::new("fdiv.s", Codec::RM, fmt::RM_FRD_FRS1_FRS2);
pub(crate) static FSGNJ_S: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FMV_S, &[Rs2EqRs1])],
    ..Opcode::new("fsgnj.s", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FSGNJN_S: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FNEG_S, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjn.s", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FSGNJX_S: Opcode = Opcode {
    pseudo: &[Pseudo::new(&pseudo::FABS_S, &[Rs2EqRs1])],
    ..Opcode::new("fsgnjx.s", Codec::R, fmt::FRD_FRS1_FRS2)
};
pub(crate) static FMIN_S: Opcode = Opcode::new("fmin.s", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FMAX_S: Opcode = Opcode::new("fmax.s", Codec::R, fmt::FRD_FRS1_FRS2);
pub(crate) static FSQRT_S: Opcode = Opcode::new("fsqrt.s", Codec::RM, fmt::RM_FRD_FRS1);
pub(crate) static FLE_S: Opcode = Opcode::new("fle.s", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FLT_S: Opcode = Opcode::new("flt.s", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FEQ_S: Opcode = Opcode::new("feq.s", Codec::R, fmt::RD_FRS1_FRS2);
pub(crate) static FCVT_W_S: Opcode = Opcode::new("fcvt.w.s", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FCVT_WU_S: Opcode = Opcode::new("fcvt.wu.s", Codec::RM, fmt::RM_RD_FRS1);
pub(crate) static FCVT_S_W: Opcode = Opcode::new("fcvt.s.w", Codec::RM, fmt::RM_FRD_RS1);
pub(crate) static FCVT_S_WU: Opcode = Opcode::new("fcvt.s.wu", Codec::RM, fmt::RM_FRD_RS1);
pub(crate) static FMV_X_W: Opcode = Opcode::new("fmv.x.w", Codec::R, fmt::RD_FRS1);
pub(crate) static FCLASS_S: Opcode = Opcode::new("fclass.s", Codec::R, fmt::RD_FRS1);
pub(crate) static FMV_W_X: Opcode = Opcode::new("fmv.w.x", Codec::R, fmt::FRD_RS1);
pub(crate) static FCVT_L_S: Opcode = Opcode::new("fcvt.l.s", Codec::RM, fmt::RM_RD_FRS1).rv64();
pub(crate) static FCVT_LU_S: Opcode = Opcode::new("fcvt.lu.s", Codec::RM, fmt::RM_RD_FRS1).rv64();
pub(crate) static FCVT_S_L: Opcode = Opcode::new("fcvt.s.l", Codec::RM, fmt::RM_FRD_RS1).rv64();
pub(crate) static FCVT_S_LU: Opcode = Opcode::new("fcvt.s.lu", Codec::RM, fmt::RM_FRD_RS1).rv64();
