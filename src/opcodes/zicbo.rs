// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Cache-block management (Zicbom, Zicboz) and the prefetch (Zicbop),
//! non-temporal locality (Zihintntl) and pause (Zihintpause) hints.
//!
//! The hints are aliases of existing instructions: `ori`, `add`, `c.add` and
//! `fence`.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static CBO_INVAL: Opcode =
    Opcode::new(Mnemonic::CboInval, Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicbom]);
pub(crate) static CBO_CLEAN: Opcode =
    Opcode::new(Mnemonic::CboClean, Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicbom]);
pub(crate) static CBO_FLUSH: Opcode =
    Opcode::new(Mnemonic::CboFlush, Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicbom]);
pub(crate) static CBO_ZERO: Opcode =
    Opcode::new(Mnemonic::CboZero, Codec::I, fmt::ADDR_RS1).requires(&[Extension::Zicboz]);
pub(crate) static PREFETCH_I: Opcode =
    Opcode::new(Mnemonic::PrefetchI, Codec::I, fmt::PREFETCH).requires(&[Extension::Zicbop]);
pub(crate) static PREFETCH_R: Opcode =
    Opcode::new(Mnemonic::PrefetchR, Codec::I, fmt::PREFETCH).requires(&[Extension::Zicbop]);
pub(crate) static PREFETCH_W: Opcode =
    Opcode::new(Mnemonic::PrefetchW, Codec::I, fmt::PREFETCH).requires(&[Extension::Zicbop]);
pub(crate) static NTL_P1: Opcode =
    Opcode::new(Mnemonic::NtlP1, Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static NTL_PALL: Opcode =
    Opcode::new(Mnemonic::NtlPall, Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static NTL_S1: Opcode =
    Opcode::new(Mnemonic::NtlS1, Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static NTL_ALL: Opcode =
    Opcode::new(Mnemonic::NtlAll, Codec::R, fmt::NONE).requires(&[Extension::Zihintntl]);
pub(crate) static PAUSE: Opcode =
    Opcode::new(Mnemonic::Pause, Codec::RF, fmt::NONE).requires(&[Extension::Zihintpause]);
