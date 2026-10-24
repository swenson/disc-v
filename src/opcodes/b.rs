// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Bit-manipulation extensions: Zba, Zbb, Zbc and Zbs.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, fmt};
use crate::reg;
use crate::{Extension, Mnemonic};

pub(crate) static ANDN: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Andn, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static ADD_UW: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&ZEXT_W, &[Rs2Eq(reg::ZERO)])],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::AddUw, Codec::R, fmt::RD_RS1_RS2)
        .requires(&[Extension::Zba])
        .rv64()
};
pub(crate) static BSET: Opcode =
    Opcode::new(Mnemonic::Bset, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbs]);
pub(crate) static BSETI: Opcode =
    Opcode::new(Mnemonic::Bseti, Codec::ISh5, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BSET_64: Opcode =
    Opcode::new(Mnemonic::Bseti, Codec::ISh6, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BCLR: Opcode =
    Opcode::new(Mnemonic::Bclr, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbs]);
pub(crate) static BCLRI: Opcode =
    Opcode::new(Mnemonic::Bclri, Codec::ISh5, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BCLRI_64: Opcode =
    Opcode::new(Mnemonic::Bclri, Codec::ISh6, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BEXT: Opcode =
    Opcode::new(Mnemonic::Bext, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbs]);
pub(crate) static BEXTI: Opcode =
    Opcode::new(Mnemonic::Bexti, Codec::ISh5, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BEXTI_64: Opcode =
    Opcode::new(Mnemonic::Bexti, Codec::ISh6, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BINV: Opcode =
    Opcode::new(Mnemonic::Binv, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbs]);
pub(crate) static BINVI: Opcode =
    Opcode::new(Mnemonic::Binvi, Codec::ISh5, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static BINVI_64: Opcode =
    Opcode::new(Mnemonic::Binvi, Codec::ISh6, fmt::RD_RS1_SHAMT).requires(&[Extension::Zbs]);
pub(crate) static CLMUL: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbc, Extension::Zbkc],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Clmul, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static CLMULH: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbc, Extension::Zbkc],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Clmulh, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static CLMULR: Opcode =
    Opcode::new(Mnemonic::Clmulr, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbc]);
pub(crate) static CLZ: Opcode =
    Opcode::new(Mnemonic::Clz, Codec::R, fmt::RD_RS1).requires(&[Extension::Zbb]);
pub(crate) static CLZW: Opcode = Opcode::new(Mnemonic::Clzw, Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zbb])
    .rv64();
pub(crate) static CPOP: Opcode =
    Opcode::new(Mnemonic::Cpop, Codec::R, fmt::RD_RS1).requires(&[Extension::Zbb]);
pub(crate) static CPOPW: Opcode = Opcode::new(Mnemonic::Cpopw, Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zbb])
    .rv64();
pub(crate) static CTZ: Opcode =
    Opcode::new(Mnemonic::Ctz, Codec::R, fmt::RD_RS1).requires(&[Extension::Zbb]);
pub(crate) static CTZW: Opcode = Opcode::new(Mnemonic::Ctzw, Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zbb])
    .rv64();
pub(crate) static MAX: Opcode =
    Opcode::new(Mnemonic::Max, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbb]);
pub(crate) static MAXU: Opcode =
    Opcode::new(Mnemonic::Maxu, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbb]);
pub(crate) static MIN: Opcode =
    Opcode::new(Mnemonic::Min, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbb]);
pub(crate) static MINU: Opcode =
    Opcode::new(Mnemonic::Minu, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbb]);
pub(crate) static ORC_B: Opcode =
    Opcode::new(Mnemonic::OrcB, Codec::R, fmt::RD_RS1).requires(&[Extension::Zbb]);
pub(crate) static ORN: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Orn, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static REV8: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Rev8, Codec::R, fmt::RD_RS1)
};
pub(crate) static ROL: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Rol, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static ROLW: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Rolw, Codec::R, fmt::RD_RS1_RS2).rv64()
};
pub(crate) static ROR: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Ror, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static RORI: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Rori, Codec::ISh5, fmt::RD_RS1_SHAMT)
};
pub(crate) static RORI_64: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Rori, Codec::ISh6, fmt::RD_RS1_SHAMT)
};
pub(crate) static RORIW: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Roriw, Codec::ISh5, fmt::RD_RS1_SHAMT).rv64()
};
pub(crate) static RORW: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Rorw, Codec::R, fmt::RD_RS1_RS2).rv64()
};
pub(crate) static SEXT_B: Opcode =
    Opcode::new(Mnemonic::SextB, Codec::R, fmt::RD_RS1).requires(&[Extension::Zbb]);
pub(crate) static SEXT_H: Opcode =
    Opcode::new(Mnemonic::SextH, Codec::R, fmt::RD_RS1).requires(&[Extension::Zbb]);
pub(crate) static SH1ADD: Opcode =
    Opcode::new(Mnemonic::Sh1add, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zba]);
pub(crate) static SH1ADD_UW: Opcode = Opcode::new(Mnemonic::Sh1addUw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zba])
    .rv64();
pub(crate) static SH2ADD: Opcode =
    Opcode::new(Mnemonic::Sh2add, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zba]);
pub(crate) static SH2ADD_UW: Opcode = Opcode::new(Mnemonic::Sh2addUw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zba])
    .rv64();
pub(crate) static SH3ADD: Opcode =
    Opcode::new(Mnemonic::Sh3add, Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zba]);
pub(crate) static SH3ADD_UW: Opcode = Opcode::new(Mnemonic::Sh3addUw, Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zba])
    .rv64();
pub(crate) static SLLI_UW: Opcode = Opcode::new(Mnemonic::SlliUw, Codec::ISh6, fmt::RD_RS1_SHAMT)
    .requires(&[Extension::Zba])
    .rv64();
pub(crate) static XNOR: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::Xnor, Codec::R, fmt::RD_RS1_RS2)
};
pub(crate) static ZEXT_H: Opcode = Opcode {
    extra: &Extra {
        requires_any: &[Extension::Zbb, Extension::Zbkb],
        ..Extra::NONE
    },
    ..Opcode::new(Mnemonic::ZextH, Codec::R, fmt::RD_RS1)
};
pub(crate) static ZEXT_W: Opcode =
    Opcode::new(Mnemonic::ZextW, Codec::R, fmt::RD_RS1).requires(&[Extension::Zba]);
