// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Compressed push, pop and register moves (Zcmp) and table jumps (Zcmt).
//!
//! Both use the encodings of `c.fsdsp`, so they conflict with Zcd.

use super::Constraint::*;
use super::{Codec, Opcode, fmt};
use crate::Extension;

/// Register lists below 4 (which would not save `ra`) are reserved.
const RESERVED_RLIST: &[super::Constraint] = &[Rs1Eq(0), Rs1Eq(1), Rs1Eq(2), Rs1Eq(3)];

pub(crate) static CM_PUSH: Opcode = Opcode {
    illegal_if: RESERVED_RLIST,
    ..Opcode::new("cm.push", Codec::CmPushPop, fmt::RLIST_NEG_ADJ).requires(&[Extension::Zcmp])
};
pub(crate) static CM_POP: Opcode = Opcode {
    illegal_if: RESERVED_RLIST,
    ..Opcode::new("cm.pop", Codec::CmPushPop, fmt::RLIST_ADJ).requires(&[Extension::Zcmp])
};
pub(crate) static CM_POPRETZ: Opcode = Opcode {
    illegal_if: RESERVED_RLIST,
    ..Opcode::new("cm.popretz", Codec::CmPushPop, fmt::RLIST_ADJ).requires(&[Extension::Zcmp])
};
pub(crate) static CM_POPRET: Opcode = Opcode {
    illegal_if: RESERVED_RLIST,
    ..Opcode::new("cm.popret", Codec::CmPushPop, fmt::RLIST_ADJ).requires(&[Extension::Zcmp])
};
/// The two s-registers must differ.
pub(crate) static CM_MVSA01: Opcode = Opcode {
    illegal_if: &[Rs2EqRs1],
    ..Opcode::new("cm.mvsa01", Codec::CmMv, fmt::RS1_RS2).requires(&[Extension::Zcmp])
};
pub(crate) static CM_MVA01S: Opcode =
    Opcode::new("cm.mva01s", Codec::CmMv, fmt::RS1_RS2).requires(&[Extension::Zcmp]);
pub(crate) static CM_JT: Opcode =
    Opcode::new("cm.jt", Codec::CmJt, fmt::IMM).requires(&[Extension::Zcmt]);
pub(crate) static CM_JALT: Opcode =
    Opcode::new("cm.jalt", Codec::CmJt, fmt::IMM).requires(&[Extension::Zcmt]);
