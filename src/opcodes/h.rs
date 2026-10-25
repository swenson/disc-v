// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! H: the hypervisor extension, and its Svinval invalidation instructions.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt};
use crate::reg;
use crate::{Extension, Mnemonic};

pub(crate) static HFENCE_VVMA: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&HFENCE_VVMA_ALL, &[Rs1Eq(reg::ZERO), Rs2Eq(reg::ZERO)]),
            Pseudo::new(&HFENCE_VVMA_RS1, &[Rs2Eq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::HfenceVvma, Codec::R, fmt::RS1_RS2).requires(&[Extension::H])
};
pub(crate) static HFENCE_GVMA: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[
            Pseudo::new(&HFENCE_GVMA_ALL, &[Rs1Eq(reg::ZERO), Rs2Eq(reg::ZERO)]),
            Pseudo::new(&HFENCE_GVMA_RS1, &[Rs2Eq(reg::ZERO)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::HfenceGvma, Codec::R, fmt::RS1_RS2).requires(&[Extension::H])
};
static HFENCE_VVMA_ALL: Opcode = Opcode::new(Mnemonic::HfenceVvma, Codec::R, fmt::NONE);
static HFENCE_VVMA_RS1: Opcode = Opcode::new(Mnemonic::HfenceVvma, Codec::R, fmt::RS1);
static HFENCE_GVMA_ALL: Opcode = Opcode::new(Mnemonic::HfenceGvma, Codec::R, fmt::NONE);
static HFENCE_GVMA_RS1: Opcode = Opcode::new(Mnemonic::HfenceGvma, Codec::R, fmt::RS1);

pub(crate) static HLV_B: Opcode =
    Opcode::new(Mnemonic::HlvB, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_BU: Opcode =
    Opcode::new(Mnemonic::HlvBu, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_H: Opcode =
    Opcode::new(Mnemonic::HlvH, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_HU: Opcode =
    Opcode::new(Mnemonic::HlvHu, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLVX_HU: Opcode =
    Opcode::new(Mnemonic::HlvxHu, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_W: Opcode =
    Opcode::new(Mnemonic::HlvW, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_WU: Opcode = Opcode::new(Mnemonic::HlvWu, Codec::R, fmt::RD_ADDR_RS1)
    .requires(&[Extension::H])
    .rv64();
pub(crate) static HLVX_WU: Opcode =
    Opcode::new(Mnemonic::HlvxWu, Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_D: Opcode = Opcode::new(Mnemonic::HlvD, Codec::R, fmt::RD_ADDR_RS1)
    .requires(&[Extension::H])
    .rv64();
pub(crate) static HSV_B: Opcode =
    Opcode::new(Mnemonic::HsvB, Codec::R, fmt::RS2_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HSV_H: Opcode =
    Opcode::new(Mnemonic::HsvH, Codec::R, fmt::RS2_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HSV_W: Opcode =
    Opcode::new(Mnemonic::HsvW, Codec::R, fmt::RS2_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HSV_D: Opcode = Opcode::new(Mnemonic::HsvD, Codec::R, fmt::RS2_ADDR_RS1)
    .requires(&[Extension::H])
    .rv64();

pub(crate) static HINVAL_VVMA: Opcode = Opcode::new(Mnemonic::HinvalVvma, Codec::R, fmt::RS1_RS2)
    .requires(&[Extension::H, Extension::Svinval]);
pub(crate) static HINVAL_GVMA: Opcode = Opcode::new(Mnemonic::HinvalGvma, Codec::R, fmt::RS1_RS2)
    .requires(&[Extension::H, Extension::Svinval]);
