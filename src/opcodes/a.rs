// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "A" extension: atomic instructions.

use super::{Codec, Opcode, fmt};
use crate::{Extension, Mnemonic};

pub(crate) static LR_W: Opcode =
    Opcode::new(Mnemonic::LrW, Codec::RL, fmt::RD_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static SC_W: Opcode =
    Opcode::new(Mnemonic::ScW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOSWAP_W: Opcode =
    Opcode::new(Mnemonic::AmoswapW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOADD_W: Opcode =
    Opcode::new(Mnemonic::AmoaddW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOXOR_W: Opcode =
    Opcode::new(Mnemonic::AmoxorW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOOR_W: Opcode =
    Opcode::new(Mnemonic::AmoorW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOAND_W: Opcode =
    Opcode::new(Mnemonic::AmoandW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOMIN_W: Opcode =
    Opcode::new(Mnemonic::AmominW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOMAX_W: Opcode =
    Opcode::new(Mnemonic::AmomaxW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOMINU_W: Opcode =
    Opcode::new(Mnemonic::AmominuW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static AMOMAXU_W: Opcode =
    Opcode::new(Mnemonic::AmomaxuW, Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::A]);
pub(crate) static LR_D: Opcode = Opcode::new(Mnemonic::LrD, Codec::RL, fmt::RD_ADDR_RS1)
    .requires(&[Extension::A])
    .rv64();
pub(crate) static SC_D: Opcode = Opcode::new(Mnemonic::ScD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::A])
    .rv64();
pub(crate) static AMOSWAP_D: Opcode =
    Opcode::new(Mnemonic::AmoswapD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOADD_D: Opcode =
    Opcode::new(Mnemonic::AmoaddD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOXOR_D: Opcode =
    Opcode::new(Mnemonic::AmoxorD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOOR_D: Opcode = Opcode::new(Mnemonic::AmoorD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::A])
    .rv64();
pub(crate) static AMOAND_D: Opcode =
    Opcode::new(Mnemonic::AmoandD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOMIN_D: Opcode =
    Opcode::new(Mnemonic::AmominD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOMAX_D: Opcode =
    Opcode::new(Mnemonic::AmomaxD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOMINU_D: Opcode =
    Opcode::new(Mnemonic::AmominuD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static AMOMAXU_D: Opcode =
    Opcode::new(Mnemonic::AmomaxuD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv64();
pub(crate) static LR_Q: Opcode = Opcode::new(Mnemonic::LrQ, Codec::RL, fmt::RD_ADDR_RS1)
    .requires(&[Extension::A])
    .rv128();
pub(crate) static SC_Q: Opcode = Opcode::new(Mnemonic::ScQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::A])
    .rv128();
pub(crate) static AMOSWAP_Q: Opcode =
    Opcode::new(Mnemonic::AmoswapQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOADD_Q: Opcode =
    Opcode::new(Mnemonic::AmoaddQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOXOR_Q: Opcode =
    Opcode::new(Mnemonic::AmoxorQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOOR_Q: Opcode = Opcode::new(Mnemonic::AmoorQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::A])
    .rv128();
pub(crate) static AMOAND_Q: Opcode =
    Opcode::new(Mnemonic::AmoandQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOMIN_Q: Opcode =
    Opcode::new(Mnemonic::AmominQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOMAX_Q: Opcode =
    Opcode::new(Mnemonic::AmomaxQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOMINU_Q: Opcode =
    Opcode::new(Mnemonic::AmominuQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
pub(crate) static AMOMAXU_Q: Opcode =
    Opcode::new(Mnemonic::AmomaxuQ, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::A])
        .rv128();
