// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zfbfmin: conversions between BFloat16 and single precision.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static FCVT_BF16_S: Opcode =
    Opcode::new(Mnemonic::FcvtBf16S, Codec::RM, fmt::RM_FRD_FRS1).requires(&[Extension::Zfbfmin]);
pub(crate) static FCVT_S_BF16: Opcode =
    Opcode::new(Mnemonic::FcvtSBf16, Codec::RM, fmt::WIDEN_FRD_FRS1)
        .requires(&[Extension::Zfbfmin]);
