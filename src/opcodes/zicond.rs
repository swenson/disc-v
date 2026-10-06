// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zicond: integer conditional operations.

use super::{Codec, Opcode, fmt};

pub(crate) static CZERO_EQZ: Opcode = Opcode::new("czero.eqz", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static CZERO_NEZ: Opcode = Opcode::new("czero.nez", Codec::R, fmt::RD_RS1_RS2);
