// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "Q" extension: quad-precision floating point.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt, pseudo};
use crate::{Extension, Mnemonic};

pub(crate) static FLQ: Opcode = Opcode::new(Mnemonic::Flq, Codec::I, fmt::FRD_OFFSET_RS1)
    .requires(&[Extension::Q])
    .f_registers();
pub(crate) static FSQ: Opcode = Opcode::new(Mnemonic::Fsq, Codec::S, fmt::FRS2_OFFSET_RS1)
    .requires(&[Extension::Q])
    .f_registers();
pub(crate) static FMADD_Q: Opcode =
    Opcode::new(Mnemonic::FmaddQ, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::Q]);
pub(crate) static FMSUB_Q: Opcode =
    Opcode::new(Mnemonic::FmsubQ, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3).requires(&[Extension::Q]);
pub(crate) static FNMSUB_Q: Opcode =
    Opcode::new(Mnemonic::FnmsubQ, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::Q]);
pub(crate) static FNMADD_Q: Opcode =
    Opcode::new(Mnemonic::FnmaddQ, Codec::R4M, fmt::RM_FRD_FRS1_FRS2_FRS3)
        .requires(&[Extension::Q]);
pub(crate) static FADD_Q: Opcode =
    Opcode::new(Mnemonic::FaddQ, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FSUB_Q: Opcode =
    Opcode::new(Mnemonic::FsubQ, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FMUL_Q: Opcode =
    Opcode::new(Mnemonic::FmulQ, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FDIV_Q: Opcode =
    Opcode::new(Mnemonic::FdivQ, Codec::RM, fmt::RM_FRD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FSGNJ_Q: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FMV_Q, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjQ, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Q])
};
pub(crate) static FSGNJN_Q: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FNEG_Q, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjnQ, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Q])
};
pub(crate) static FSGNJX_Q: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::FABS_Q, &[Rs2EqRs1])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::FsgnjxQ, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Q])
};
pub(crate) static FMIN_Q: Opcode =
    Opcode::new(Mnemonic::FminQ, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FMAX_Q: Opcode =
    Opcode::new(Mnemonic::FmaxQ, Codec::R, fmt::FRD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FCVT_S_Q: Opcode =
    Opcode::new(Mnemonic::FcvtSQ, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Q]);
pub(crate) static FCVT_Q_S: Opcode =
    Opcode::new(Mnemonic::FcvtQS, Codec::RM, fmt::WIDEN_FRD_FRS1).requires(&[Extension::Q]);
pub(crate) static FCVT_D_Q: Opcode =
    Opcode::new(Mnemonic::FcvtDQ, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Q]);
pub(crate) static FCVT_Q_D: Opcode =
    Opcode::new(Mnemonic::FcvtQD, Codec::RM, fmt::WIDEN_FRD_FRS1).requires(&[Extension::Q]);
pub(crate) static FSQRT_Q: Opcode =
    Opcode::new(Mnemonic::FsqrtQ, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Q]);
pub(crate) static FLE_Q: Opcode =
    Opcode::new(Mnemonic::FleQ, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FLT_Q: Opcode =
    Opcode::new(Mnemonic::FltQ, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FEQ_Q: Opcode =
    Opcode::new(Mnemonic::FeqQ, Codec::R, fmt::RD_FRS1_FRS2).requires(&[Extension::Q]);
pub(crate) static FCVT_W_Q: Opcode =
    Opcode::new(Mnemonic::FcvtWQ, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::Q]);
pub(crate) static FCVT_WU_Q: Opcode =
    Opcode::new(Mnemonic::FcvtWuQ, Codec::RM, fmt::RM_RD_FRS1).requires(&[Extension::Q]);
pub(crate) static FCVT_Q_W: Opcode =
    Opcode::new(Mnemonic::FcvtQW, Codec::RM, fmt::WIDEN_FRD_RS1).requires(&[Extension::Q]);
pub(crate) static FCVT_Q_WU: Opcode =
    Opcode::new(Mnemonic::FcvtQWu, Codec::RM, fmt::WIDEN_FRD_RS1).requires(&[Extension::Q]);
pub(crate) static FCLASS_Q: Opcode =
    Opcode::new(Mnemonic::FclassQ, Codec::R, fmt::RD_FRS1).requires(&[Extension::Q]);
pub(crate) static FCVT_L_Q: Opcode = Opcode::new(Mnemonic::FcvtLQ, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::Q])
    .rv64();
pub(crate) static FCVT_LU_Q: Opcode = Opcode::new(Mnemonic::FcvtLuQ, Codec::RM, fmt::RM_RD_FRS1)
    .requires(&[Extension::Q])
    .rv64();
pub(crate) static FCVT_Q_L: Opcode = Opcode::new(Mnemonic::FcvtQL, Codec::RM, fmt::WIDEN_FRD_RS1)
    .requires(&[Extension::Q])
    .rv64();
pub(crate) static FCVT_Q_LU: Opcode = Opcode::new(Mnemonic::FcvtQLu, Codec::RM, fmt::WIDEN_FRD_RS1)
    .requires(&[Extension::Q])
    .rv64();
pub(crate) static FMV_X_Q: Opcode = Opcode::new(Mnemonic::FmvXQ, Codec::R, fmt::RD_FRS1)
    .requires(&[Extension::Q])
    .rv128()
    .f_registers();
pub(crate) static FMV_Q_X: Opcode = Opcode::new(Mnemonic::FmvQX, Codec::R, fmt::FRD_RS1)
    .requires(&[Extension::Q])
    .rv128()
    .f_registers();
