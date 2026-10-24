// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Compressed push, pop and register moves (Zcmp) and table jumps (Zcmt).
//!
//! Both use the encodings of `c.fsdsp`, so they conflict with Zcd.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, fmt};
use crate::{Extension, Mnemonic};

/// Register lists below 4 (which would not save `ra`) are reserved.
const RESERVED_RLIST: &[super::Constraint] = &[Rs1Eq(0), Rs1Eq(1), Rs1Eq(2), Rs1Eq(3)];

pub(crate) static CM_PUSH: Opcode = Opcode {
    extra: &Extra {
        illegal_if: RESERVED_RLIST,
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CmPush, Codec::CmPushPop, fmt::RLIST_NEG_ADJ)
        .requires(&[Extension::Zcmp])
};
pub(crate) static CM_POP: Opcode = Opcode {
    extra: &Extra {
        illegal_if: RESERVED_RLIST,
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CmPop, Codec::CmPushPop, fmt::RLIST_ADJ).requires(&[Extension::Zcmp])
};
pub(crate) static CM_POPRETZ: Opcode = Opcode {
    extra: &Extra {
        illegal_if: RESERVED_RLIST,
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CmPopretz, Codec::CmPushPop, fmt::RLIST_ADJ)
        .requires(&[Extension::Zcmp])
};
pub(crate) static CM_POPRET: Opcode = Opcode {
    extra: &Extra {
        illegal_if: RESERVED_RLIST,
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CmPopret, Codec::CmPushPop, fmt::RLIST_ADJ).requires(&[Extension::Zcmp])
};
/// The two s-registers must differ.
pub(crate) static CM_MVSA01: Opcode = Opcode {
    extra: &Extra {
        illegal_if: &[Rs2EqRs1],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::CmMvsa01, Codec::CmMv, fmt::RS1_RS2).requires(&[Extension::Zcmp])
};
pub(crate) static CM_MVA01S: Opcode =
    Opcode::new(Mnemonic::CmMva01s, Codec::CmMv, fmt::RS1_RS2).requires(&[Extension::Zcmp]);
pub(crate) static CM_JT: Opcode =
    Opcode::new(Mnemonic::CmJt, Codec::CmJt, fmt::IMM).requires(&[Extension::Zcmt]);
pub(crate) static CM_JALT: Opcode =
    Opcode::new(Mnemonic::CmJalt, Codec::CmJt, fmt::IMM).requires(&[Extension::Zcmt]);
