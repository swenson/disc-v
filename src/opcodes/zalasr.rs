// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Zalasr: load-acquire and store-release.
//!
//! The names include the ordering that each instruction always has; the
//! other ordering bit is shown as for the atomics (`lw.aqrl`).

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static LB_AQ: Opcode =
    Opcode::new(Mnemonic::LbAq, Codec::RL, fmt::RD_ADDR_RS1).requires(&[Extension::Zalasr]);
pub(crate) static LH_AQ: Opcode =
    Opcode::new(Mnemonic::LhAq, Codec::RL, fmt::RD_ADDR_RS1).requires(&[Extension::Zalasr]);
pub(crate) static LW_AQ: Opcode =
    Opcode::new(Mnemonic::LwAq, Codec::RL, fmt::RD_ADDR_RS1).requires(&[Extension::Zalasr]);
pub(crate) static LD_AQ: Opcode = Opcode::new(Mnemonic::LdAq, Codec::RL, fmt::RD_ADDR_RS1)
    .requires(&[Extension::Zalasr])
    .rv64();
pub(crate) static SB_RL: Opcode =
    Opcode::new(Mnemonic::SbRl, Codec::RA, fmt::RS2_ADDR_RS1).requires(&[Extension::Zalasr]);
pub(crate) static SH_RL: Opcode =
    Opcode::new(Mnemonic::ShRl, Codec::RA, fmt::RS2_ADDR_RS1).requires(&[Extension::Zalasr]);
pub(crate) static SW_RL: Opcode =
    Opcode::new(Mnemonic::SwRl, Codec::RA, fmt::RS2_ADDR_RS1).requires(&[Extension::Zalasr]);
pub(crate) static SD_RL: Opcode = Opcode::new(Mnemonic::SdRl, Codec::RA, fmt::RS2_ADDR_RS1)
    .requires(&[Extension::Zalasr])
    .rv64();
