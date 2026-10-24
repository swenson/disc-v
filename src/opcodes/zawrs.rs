// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zawrs: wait-on-reservation-set.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static WRS_NTO: Opcode =
    Opcode::new(Mnemonic::WrsNto, Codec::None, fmt::NONE).requires(&[Extension::Zawrs]);
pub(crate) static WRS_STO: Opcode =
    Opcode::new(Mnemonic::WrsSto, Codec::None, fmt::NONE).requires(&[Extension::Zawrs]);
