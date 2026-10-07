// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! H: the hypervisor extension, and its Svinval invalidation instructions.

use super::Constraint::*;
use super::{Codec, Opcode, Pseudo, fmt};
use crate::Extension;
use crate::reg;

pub(crate) static HFENCE_VVMA: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&HFENCE_VVMA_ALL, &[Rs1Eq(reg::ZERO), Rs2Eq(reg::ZERO)]),
        Pseudo::new(&HFENCE_VVMA_RS1, &[Rs2Eq(reg::ZERO)]),
    ],
    ..Opcode::new("hfence.vvma", Codec::R, fmt::RS1_RS2).requires(&[Extension::H])
};
pub(crate) static HFENCE_GVMA: Opcode = Opcode {
    pseudo: &[
        Pseudo::new(&HFENCE_GVMA_ALL, &[Rs1Eq(reg::ZERO), Rs2Eq(reg::ZERO)]),
        Pseudo::new(&HFENCE_GVMA_RS1, &[Rs2Eq(reg::ZERO)]),
    ],
    ..Opcode::new("hfence.gvma", Codec::R, fmt::RS1_RS2).requires(&[Extension::H])
};
static HFENCE_VVMA_ALL: Opcode = Opcode::new("hfence.vvma", Codec::R, fmt::NONE);
static HFENCE_VVMA_RS1: Opcode = Opcode::new("hfence.vvma", Codec::R, fmt::RS1);
static HFENCE_GVMA_ALL: Opcode = Opcode::new("hfence.gvma", Codec::R, fmt::NONE);
static HFENCE_GVMA_RS1: Opcode = Opcode::new("hfence.gvma", Codec::R, fmt::RS1);

pub(crate) static HLV_B: Opcode =
    Opcode::new("hlv.b", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_BU: Opcode =
    Opcode::new("hlv.bu", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_H: Opcode =
    Opcode::new("hlv.h", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_HU: Opcode =
    Opcode::new("hlv.hu", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLVX_HU: Opcode =
    Opcode::new("hlvx.hu", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_W: Opcode =
    Opcode::new("hlv.w", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_WU: Opcode = Opcode::new("hlv.wu", Codec::R, fmt::RD_ADDR_RS1)
    .requires(&[Extension::H])
    .rv64();
pub(crate) static HLVX_WU: Opcode =
    Opcode::new("hlvx.wu", Codec::R, fmt::RD_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HLV_D: Opcode = Opcode::new("hlv.d", Codec::R, fmt::RD_ADDR_RS1)
    .requires(&[Extension::H])
    .rv64();
pub(crate) static HSV_B: Opcode =
    Opcode::new("hsv.b", Codec::R, fmt::RS2_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HSV_H: Opcode =
    Opcode::new("hsv.h", Codec::R, fmt::RS2_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HSV_W: Opcode =
    Opcode::new("hsv.w", Codec::R, fmt::RS2_ADDR_RS1).requires(&[Extension::H]);
pub(crate) static HSV_D: Opcode = Opcode::new("hsv.d", Codec::R, fmt::RS2_ADDR_RS1)
    .requires(&[Extension::H])
    .rv64();

pub(crate) static HINVAL_VVMA: Opcode = Opcode::new("hinval.vvma", Codec::R, fmt::RS1_RS2)
    .requires(&[Extension::H, Extension::Svinval]);
pub(crate) static HINVAL_GVMA: Opcode = Opcode::new("hinval.gvma", Codec::R, fmt::RS1_RS2)
    .requires(&[Extension::H, Extension::Svinval]);
