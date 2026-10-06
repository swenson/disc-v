// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zawrs: wait-on-reservation-set.

use super::{Codec, Opcode, fmt};

pub(crate) static WRS_NTO: Opcode = Opcode::new("wrs.nto", Codec::None, fmt::NONE);
pub(crate) static WRS_STO: Opcode = Opcode::new("wrs.sto", Codec::None, fmt::NONE);
