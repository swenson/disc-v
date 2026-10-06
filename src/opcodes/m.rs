// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "M" extension: integer multiplication and division.

use super::{Codec, Opcode, fmt};

pub(crate) static MUL: Opcode = Opcode::new("mul", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MULH: Opcode = Opcode::new("mulh", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MULHSU: Opcode = Opcode::new("mulhsu", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MULHU: Opcode = Opcode::new("mulhu", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static DIV: Opcode = Opcode::new("div", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static DIVU: Opcode = Opcode::new("divu", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static REM: Opcode = Opcode::new("rem", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static REMU: Opcode = Opcode::new("remu", Codec::R, fmt::RD_RS1_RS2);
pub(crate) static MULW: Opcode = Opcode::new("mulw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static DIVW: Opcode = Opcode::new("divw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static DIVUW: Opcode = Opcode::new("divuw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static REMW: Opcode = Opcode::new("remw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static REMUW: Opcode = Opcode::new("remuw", Codec::R, fmt::RD_RS1_RS2).rv64();
pub(crate) static MULD: Opcode = Opcode::new("muld", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static DIVD: Opcode = Opcode::new("divd", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static DIVUD: Opcode = Opcode::new("divud", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static REMD: Opcode = Opcode::new("remd", Codec::R, fmt::RD_RS1_RS2).rv128();
pub(crate) static REMUD: Opcode = Opcode::new("remud", Codec::R, fmt::RD_RS1_RS2).rv128();
