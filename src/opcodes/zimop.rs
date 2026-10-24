// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! May-be-operations (Zimop, Zcmop), and the shadow-stack (Zicfiss) and
//! landing-pad (Zicfilp) instructions defined in terms of them.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt};
use crate::reg;
use crate::{Extension, Mnemonic};

/// Builds a numbered family of opcodes, such as `mop.r.0` to `mop.r.31`.
const fn family<const N: usize>(
    names: [Mnemonic; N],
    codec: Codec,
    format: &'static str,
    requires: &'static [Extension],
) -> [Opcode; N] {
    let mut ops = [const { Opcode::new(Mnemonic::Illegal, Codec::None, fmt::NONE) }; N];
    let mut i = 0;
    while i < N {
        ops[i] = Opcode::new(names[i], codec, format).requires(requires);
        i += 1;
    }
    ops
}

/// `mop.r.0` to `mop.r.31`.
pub(crate) static MOP_R: [Opcode; 32] = {
    let mut ops = family(
        [
            Mnemonic::MopR0,
            Mnemonic::MopR1,
            Mnemonic::MopR2,
            Mnemonic::MopR3,
            Mnemonic::MopR4,
            Mnemonic::MopR5,
            Mnemonic::MopR6,
            Mnemonic::MopR7,
            Mnemonic::MopR8,
            Mnemonic::MopR9,
            Mnemonic::MopR10,
            Mnemonic::MopR11,
            Mnemonic::MopR12,
            Mnemonic::MopR13,
            Mnemonic::MopR14,
            Mnemonic::MopR15,
            Mnemonic::MopR16,
            Mnemonic::MopR17,
            Mnemonic::MopR18,
            Mnemonic::MopR19,
            Mnemonic::MopR20,
            Mnemonic::MopR21,
            Mnemonic::MopR22,
            Mnemonic::MopR23,
            Mnemonic::MopR24,
            Mnemonic::MopR25,
            Mnemonic::MopR26,
            Mnemonic::MopR27,
            Mnemonic::MopR28,
            Mnemonic::MopR29,
            Mnemonic::MopR30,
            Mnemonic::MopR31,
        ],
        Codec::R,
        fmt::RD_RS1,
        &[Extension::Zimop],
    );
    ops[28].extra = &MOP_R_28_EXTRA;
    ops
};

/// `mop.rr.0` to `mop.rr.7`.
pub(crate) static MOP_RR: [Opcode; 8] = {
    let mut ops = family(
        [
            Mnemonic::MopRr0,
            Mnemonic::MopRr1,
            Mnemonic::MopRr2,
            Mnemonic::MopRr3,
            Mnemonic::MopRr4,
            Mnemonic::MopRr5,
            Mnemonic::MopRr6,
            Mnemonic::MopRr7,
        ],
        Codec::R,
        fmt::RD_RS1_RS2,
        &[Extension::Zimop],
    );
    ops[7].extra = &MOP_RR_7_EXTRA;
    ops
};

/// `c.mop.1`, `c.mop.3`, ..., `c.mop.15`. The rd field holds the number.
pub(crate) static C_MOP: [Opcode; 8] = {
    let mut ops = family(
        [
            Mnemonic::CMop1,
            Mnemonic::CMop3,
            Mnemonic::CMop5,
            Mnemonic::CMop7,
            Mnemonic::CMop9,
            Mnemonic::CMop11,
            Mnemonic::CMop13,
            Mnemonic::CMop15,
        ],
        Codec::Ci,
        fmt::NONE,
        &[Extension::Zcmop],
    );
    ops[0].extra = &C_MOP_1_EXTRA;
    ops[2].extra = &C_MOP_5_EXTRA;
    ops
};

static MOP_R_28_EXTRA: Extra = Extra {
    pseudo: &MOP_R_28_ALIASES,
    ..Extra::NONE
};
static MOP_R_28_ALIASES: [Pseudo; 4] = [
    Pseudo::new(&SSPOPCHK, &[RdEq(reg::ZERO), Rs1Eq(reg::RA)]),
    Pseudo::new(&SSPOPCHK, &[RdEq(reg::ZERO), Rs1Eq(reg::T0)]),
    // ssrdp needs a nonzero rd.
    Pseudo::new(&MOP_R[28], &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO)]),
    Pseudo::new(&SSRDP, &[Rs1Eq(reg::ZERO)]),
];
static MOP_RR_7_EXTRA: Extra = Extra {
    pseudo: &MOP_RR_7_ALIASES,
    ..Extra::NONE
};
static MOP_RR_7_ALIASES: [Pseudo; 2] = [
    Pseudo::new(
        &SSPUSH,
        &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), Rs2Eq(reg::RA)],
    ),
    Pseudo::new(
        &SSPUSH,
        &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO), Rs2Eq(reg::T0)],
    ),
];
static C_MOP_1_EXTRA: Extra = Extra {
    pseudo: &C_MOP_1_ALIASES,
    ..Extra::NONE
};
static C_MOP_1_ALIASES: [Pseudo; 1] = [Pseudo::new(&C_SSPUSH, &[])];
static C_MOP_5_EXTRA: Extra = Extra {
    pseudo: &C_MOP_5_ALIASES,
    ..Extra::NONE
};
static C_MOP_5_ALIASES: [Pseudo; 1] = [Pseudo::new(&C_SSPOPCHK, &[])];

pub(crate) static SSPOPCHK: Opcode =
    Opcode::new(Mnemonic::Sspopchk, Codec::R, fmt::RS1).requires(&[Extension::Zicfiss]);
pub(crate) static SSRDP: Opcode =
    Opcode::new(Mnemonic::Ssrdp, Codec::R, fmt::RD).requires(&[Extension::Zicfiss]);
pub(crate) static SSPUSH: Opcode =
    Opcode::new(Mnemonic::Sspush, Codec::R, fmt::RS2).requires(&[Extension::Zicfiss]);
pub(crate) static C_SSPUSH: Opcode =
    Opcode::new(Mnemonic::Sspush, Codec::Ci, fmt::RS1).requires(&[Extension::Zicfiss]);
pub(crate) static C_SSPOPCHK: Opcode =
    Opcode::new(Mnemonic::Sspopchk, Codec::Ci, fmt::RS1).requires(&[Extension::Zicfiss]);
pub(crate) static SSAMOSWAP_W: Opcode =
    Opcode::new(Mnemonic::SsamoswapW, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zicfiss]);
pub(crate) static SSAMOSWAP_D: Opcode =
    Opcode::new(Mnemonic::SsamoswapD, Codec::RA, fmt::RD_RS2_ADDR_RS1)
        .requires(&[Extension::Zicfiss])
        .rv64();
pub(crate) static LPAD: Opcode =
    Opcode::new(Mnemonic::Lpad, Codec::U, fmt::UIMM).requires(&[Extension::Zicfilp]);
