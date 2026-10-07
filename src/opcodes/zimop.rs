// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! May-be-operations (Zimop, Zcmop), and the shadow-stack (Zicfiss) and
//! landing-pad (Zicfilp) instructions defined in terms of them.

use super::Constraint::*;
use super::{Codec, Opcode, Pseudo, fmt};
use crate::Extension;
use crate::reg;

/// Builds a numbered family of opcodes, such as `mop.r.0` to `mop.r.31`.
const fn family<const N: usize>(
    names: [&'static str; N],
    codec: Codec,
    format: &'static str,
    requires: &'static [Extension],
) -> [Opcode; N] {
    let mut ops = [const { Opcode::new("", Codec::None, fmt::NONE) }; N];
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
            "mop.r.0", "mop.r.1", "mop.r.2", "mop.r.3", "mop.r.4", "mop.r.5", "mop.r.6", "mop.r.7",
            "mop.r.8", "mop.r.9", "mop.r.10", "mop.r.11", "mop.r.12", "mop.r.13", "mop.r.14",
            "mop.r.15", "mop.r.16", "mop.r.17", "mop.r.18", "mop.r.19", "mop.r.20", "mop.r.21",
            "mop.r.22", "mop.r.23", "mop.r.24", "mop.r.25", "mop.r.26", "mop.r.27", "mop.r.28",
            "mop.r.29", "mop.r.30", "mop.r.31",
        ],
        Codec::R,
        fmt::RD_RS1,
        &[Extension::Zimop],
    );
    ops[28].pseudo = &MOP_R_28_ALIASES;
    ops
};

/// `mop.rr.0` to `mop.rr.7`.
pub(crate) static MOP_RR: [Opcode; 8] = {
    let mut ops = family(
        [
            "mop.rr.0", "mop.rr.1", "mop.rr.2", "mop.rr.3", "mop.rr.4", "mop.rr.5", "mop.rr.6",
            "mop.rr.7",
        ],
        Codec::R,
        fmt::RD_RS1_RS2,
        &[Extension::Zimop],
    );
    ops[7].pseudo = &MOP_RR_7_ALIASES;
    ops
};

/// `c.mop.1`, `c.mop.3`, ..., `c.mop.15`. The rd field holds the number.
pub(crate) static C_MOP: [Opcode; 8] = {
    let mut ops = family(
        [
            "c.mop.1", "c.mop.3", "c.mop.5", "c.mop.7", "c.mop.9", "c.mop.11", "c.mop.13",
            "c.mop.15",
        ],
        Codec::Ci,
        fmt::NONE,
        &[Extension::Zcmop],
    );
    ops[0].pseudo = &C_MOP_1_ALIASES;
    ops[2].pseudo = &C_MOP_5_ALIASES;
    ops
};

static MOP_R_28_ALIASES: [Pseudo; 4] = [
    Pseudo::new(&SSPOPCHK, &[RdEq(reg::ZERO), Rs1Eq(reg::RA)]),
    Pseudo::new(&SSPOPCHK, &[RdEq(reg::ZERO), Rs1Eq(reg::T0)]),
    // ssrdp needs a nonzero rd.
    Pseudo::new(&MOP_R[28], &[RdEq(reg::ZERO), Rs1Eq(reg::ZERO)]),
    Pseudo::new(&SSRDP, &[Rs1Eq(reg::ZERO)]),
];
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
static C_MOP_1_ALIASES: [Pseudo; 1] = [Pseudo::new(&C_SSPUSH, &[])];
static C_MOP_5_ALIASES: [Pseudo; 1] = [Pseudo::new(&C_SSPOPCHK, &[])];

pub(crate) static SSPOPCHK: Opcode =
    Opcode::new("sspopchk", Codec::R, fmt::RS1).requires(&[Extension::Zicfiss]);
pub(crate) static SSRDP: Opcode =
    Opcode::new("ssrdp", Codec::R, fmt::RD).requires(&[Extension::Zicfiss]);
pub(crate) static SSPUSH: Opcode =
    Opcode::new("sspush", Codec::R, fmt::RS2).requires(&[Extension::Zicfiss]);
pub(crate) static C_SSPUSH: Opcode =
    Opcode::new("sspush", Codec::Ci, fmt::RS1).requires(&[Extension::Zicfiss]);
pub(crate) static C_SSPOPCHK: Opcode =
    Opcode::new("sspopchk", Codec::Ci, fmt::RS1).requires(&[Extension::Zicfiss]);
pub(crate) static SSAMOSWAP_W: Opcode =
    Opcode::new("ssamoswap.w", Codec::RA, fmt::RD_RS2_ADDR_RS1).requires(&[Extension::Zicfiss]);
pub(crate) static SSAMOSWAP_D: Opcode = Opcode::new("ssamoswap.d", Codec::RA, fmt::RD_RS2_ADDR_RS1)
    .requires(&[Extension::Zicfiss])
    .rv64();
pub(crate) static LPAD: Opcode =
    Opcode::new("lpad", Codec::U, fmt::UIMM).requires(&[Extension::Zicfilp]);
