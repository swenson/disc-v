// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zicond: integer conditional operations.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static CZERO_EQZ: Opcode =
    Opcode::new(Mnemonic::CzeroEqz, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zicond]);
pub(crate) static CZERO_NEZ: Opcode =
    Opcode::new(Mnemonic::CzeroNez, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zicond]);
