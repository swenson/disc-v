// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! T-Head vendor extensions (XTheadBa, XTheadBb, XTheadBs, XTheadCmo,
//! XTheadCondMov, XTheadFMemIdx, XTheadFmv, XTheadInt, XTheadMac,
//! XTheadMemIdx, XTheadMemPair, XTheadSync), all in the custom-0 opcode
//! space.
//!
//! The encodings are from the T-Head extension specification
//! (<https://github.com/XUANTIE-RV/thead-extension-spec>, commit
//! d16533b5a9cd790c369d849ddf9a48159a5a235c, Apache-2.0), with the RV64-only
//! instructions as in LLVM, and the operand syntax is objdump's.

use super::Constraint::*;
use super::{Codec, Extra, MaskedOpcode, Opcode, masked};
use crate::{Extension, Mnemonic};

pub(crate) static CUSTOM_0: [MaskedOpcode; 106] = [
    masked(
        0x0000100b,
        0xf800707f,
        Opcode::new(Mnemonic::ThAddsl, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadBa]),
    ),
    masked(
        0x1000100b,
        0xfc00707f,
        Opcode::new(Mnemonic::ThSrri, Codec::IUimm6, "0,1,i").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x1400100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThSrriw, Codec::ISh5, "0,1,i")
            .requires(&[Extension::XTheadBb])
            .rv64(),
    ),
    masked(
        0x0000200b,
        0x0000707f,
        Opcode::new(Mnemonic::ThExt, Codec::ImmPair6, "0,1,i,j").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x0000300b,
        0x0000707f,
        Opcode::new(Mnemonic::ThExtu, Codec::ImmPair6, "0,1,i,j").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8400100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThFf0, Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8600100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThFf1, Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8200100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThRev, Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8000100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThTstnbz, Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x9000100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThRevw, Codec::R, "0,1")
            .requires(&[Extension::XTheadBb])
            .rv64(),
    ),
    masked(
        0x8800100b,
        0xfc00707f,
        Opcode::new(Mnemonic::ThTst, Codec::IUimm6, "0,1,i").requires(&[Extension::XTheadBs]),
    ),
    masked(
        0x0210000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCsw, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0220000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheIsw, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0230000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCisw, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0240000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCval1, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0250000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCva, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0260000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheIva, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0270000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCiva, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0280000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCpal1, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0290000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCpa, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x02a0000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheIpa, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x02b0000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThDcacheCipa, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0300000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThIcacheIva, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0380000b,
        0xfff07fff,
        Opcode::new(Mnemonic::ThIcacheIpa, Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0010000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThDcacheCall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0020000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThDcacheIall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0030000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThDcacheCiall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0100000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThIcacheIall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0110000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThIcacheIalls, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0150000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThL2cacheCall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0160000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThL2cacheIall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0170000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThL2cacheCiall, Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x4000100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMveqz, Codec::R, "0,1,2").requires(&[Extension::XTheadCondMov]),
    ),
    masked(
        0x4200100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMvnez, Codec::R, "0,1,2").requires(&[Extension::XTheadCondMov]),
    ),
    masked(
        0x4000600b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFlrw, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x6000600b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFlrd, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0x5000600b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFlurw, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x7000600b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFlurd, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0x4000700b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFsrw, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x6000700b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFsrd, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0x5000700b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFsurw, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x7000700b,
        0xf800707f,
        Opcode::new(Mnemonic::ThFsurd, Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0xa000100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThFmvHwX, Codec::R, "3,1")
            .requires(&[Extension::XTheadFmv, Extension::D])
            .rv32_only()
            .f_registers(),
    ),
    masked(
        0xc000100b,
        0xfff0707f,
        Opcode::new(Mnemonic::ThFmvXHw, Codec::R, "0,4")
            .requires(&[Extension::XTheadFmv, Extension::D])
            .rv32_only()
            .f_registers(),
    ),
    masked(
        0x0040000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThIpush, Codec::None, "").requires(&[Extension::XTheadInt]),
    ),
    masked(
        0x0050000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThIpop, Codec::None, "").requires(&[Extension::XTheadInt]),
    ),
    masked(
        0x2000100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMula, Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2200100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMuls, Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2800100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMulah, Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2a00100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMulsh, Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2400100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMulaw, Codec::R, "0,1,2")
            .requires(&[Extension::XTheadMac])
            .rv64(),
    ),
    masked(
        0x2600100b,
        0xfe00707f,
        Opcode::new(Mnemonic::ThMulsw, Codec::R, "0,1,2")
            .requires(&[Extension::XTheadMac])
            .rv64(),
    ),
    masked(
        0x1800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLbia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x0800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLbib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x9800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLbuia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x8800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLbuib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x3800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLhia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x2800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLhib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0xb800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLhuia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0xa800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLhuib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x5800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLwia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x4800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLwib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0xd800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLwuia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0xc800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLwuib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0x7800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLdia, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0x6800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLdib, Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0x1800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSbia, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x0800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSbib, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x3800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThShia, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x2800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThShib, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x5800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSwia, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x4800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSwib, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x7800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSdia, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x6800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSdib, Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x0000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrb, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x8000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrbu, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x1000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurb, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x9000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurbu, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x2000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrh, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0xa000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrhu, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x3000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurh, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0xb000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurhu, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x4000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrw, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x5000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurw, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0xc000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrwu, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0xd000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurwu, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x6000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLrd, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x7000400b,
        0xf800707f,
        Opcode::new(Mnemonic::ThLurd, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x0000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSrb, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x1000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSurb, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x2000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSrh, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x3000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSurh, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x4000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSrw, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x5000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSurw, Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x6000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSrd, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x7000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSurd, Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0xe000400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1, Rs2EqRs1, RdEqRs2],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLwd, Codec::Pair, "0,2,(1),i,j")
                .requires(&[Extension::XTheadMemPair])
        },
    ),
    masked(
        0xf000400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1, Rs2EqRs1, RdEqRs2],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLwud, Codec::Pair, "0,2,(1),i,j")
                .requires(&[Extension::XTheadMemPair])
        },
    ),
    masked(
        0xf800400b,
        0xf800707f,
        Opcode {
            extra: &Extra {
                illegal_if: &[RdEqRs1, Rs2EqRs1, RdEqRs2],
                ..Extra::NONE
            },
            ..Opcode::new(Mnemonic::ThLdd, Codec::Pair, "0,2,(1),i,j")
                .requires(&[Extension::XTheadMemPair])
                .rv64()
        },
    ),
    masked(
        0xe000500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSwd, Codec::Pair, "0,2,(1),i,j")
            .requires(&[Extension::XTheadMemPair]),
    ),
    masked(
        0xf800500b,
        0xf800707f,
        Opcode::new(Mnemonic::ThSdd, Codec::Pair, "0,2,(1),i,j")
            .requires(&[Extension::XTheadMemPair])
            .rv64(),
    ),
    masked(
        0x0400000b,
        0xfe007fff,
        Opcode::new(Mnemonic::ThSfenceVmas, Codec::R, "1,2").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x0180000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThSync, Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x0190000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThSyncS, Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x01a0000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThSyncI, Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x01b0000b,
        0xffffffff,
        Opcode::new(Mnemonic::ThSyncIs, Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
];
