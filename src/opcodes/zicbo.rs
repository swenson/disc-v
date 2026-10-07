// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Cache-block management (Zicbom, Zicboz) and the prefetch (Zicbop),
//! non-temporal locality (Zihintntl) and pause (Zihintpause) hints.
//!
//! The hints are aliases of existing instructions: `ori`, `add`, `c.add` and
//! `fence`.

use super::{Codec, Opcode, fmt};
use crate::Extension;

pub(crate) static CBO_INVAL: Opcode =
    Opcode::new("cbo.inval", Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicbom]);
pub(crate) static CBO_CLEAN: Opcode =
    Opcode::new("cbo.clean", Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicbom]);
pub(crate) static CBO_FLUSH: Opcode =
    Opcode::new("cbo.flush", Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicbom]);
pub(crate) static CBO_ZERO: Opcode =
    Opcode::new("cbo.zero", Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicboz]);
pub(crate) static PREFETCH_I: Opcode =
    Opcode::new("prefetch.i", Codec::I, fmt::PREFETCH).requires(&[Extension::Zicbop]);
pub(crate) static PREFETCH_R: Opcode =
    Opcode::new("prefetch.r", Codec::I, fmt::PREFETCH).requires(&[Extension::Zicbop]);
pub(crate) static PREFETCH_W: Opcode =
    Opcode::new("prefetch.w", Codec::I, fmt::PREFETCH).requires(&[Extension::Zicbop]);
pub(crate) static NTL_P1: Opcode =
    Opcode::new("ntl.p1", Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static NTL_PALL: Opcode =
    Opcode::new("ntl.pall", Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static NTL_S1: Opcode =
    Opcode::new("ntl.s1", Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static NTL_ALL: Opcode =
    Opcode::new("ntl.all", Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static PAUSE: Opcode =
    Opcode::new("pause", Codec::RF, fmt::NONE).requires(&[Extension::Zihintpause]);
