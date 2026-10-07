// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zfbfmin: conversions between BFloat16 and single precision.

use super::{Codec, Opcode, fmt};
use crate::Extension;

pub(crate) static FCVT_BF16_S: Opcode =
    Opcode::new("fcvt.bf16.s", Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfbfmin]);
pub(crate) static FCVT_S_BF16: Opcode =
    Opcode::new("fcvt.s.bf16", Codec::RM, fmt::WIDEN_FRD_FRS1).requires(&[Extension::Zfbfmin]);
