// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "M" extension: integer multiplication and division.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static MUL: Opcode =
    Opcode::new(Mnemonic::Mul, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static MULH: Opcode =
    Opcode::new(Mnemonic::Mulh, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static MULHSU: Opcode =
    Opcode::new(Mnemonic::Mulhsu, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static MULHU: Opcode =
    Opcode::new(Mnemonic::Mulhu, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static DIV: Opcode =
    Opcode::new(Mnemonic::Div, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static DIVU: Opcode =
    Opcode::new(Mnemonic::Divu, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static REM: Opcode =
    Opcode::new(Mnemonic::Rem, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static REMU: Opcode =
    Opcode::new(Mnemonic::Remu, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::M]);
pub(crate) static MULW: Opcode = Opcode::new(Mnemonic::Mulw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv64();
pub(crate) static DIVW: Opcode = Opcode::new(Mnemonic::Divw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv64();
pub(crate) static DIVUW: Opcode = Opcode::new(Mnemonic::Divuw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv64();
pub(crate) static REMW: Opcode = Opcode::new(Mnemonic::Remw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv64();
pub(crate) static REMUW: Opcode = Opcode::new(Mnemonic::Remuw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv64();
pub(crate) static MULD: Opcode = Opcode::new(Mnemonic::Muld, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv128();
pub(crate) static DIVD: Opcode = Opcode::new(Mnemonic::Divd, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv128();
pub(crate) static DIVUD: Opcode = Opcode::new(Mnemonic::Divud, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv128();
pub(crate) static REMD: Opcode = Opcode::new(Mnemonic::Remd, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv128();
pub(crate) static REMUD: Opcode = Opcode::new(Mnemonic::Remud, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::M])
    .rv128();
