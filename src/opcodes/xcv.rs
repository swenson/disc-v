// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! CORE-V vendor extensions (XCValu, XCVbi, XCVbitmanip, XCVelw, XCVmac,
//! XCVmem, XCVsimd), for RV32 in the custom-0 to custom-3 opcode spaces.
//!
//! The encodings are from LLVM's RISCVInstrInfoXCV.td (Apache-2.0 WITH
//! LLVM-exception), which follows the CV32E40P user manual, and the operand
//! syntax is objdump's.

use super::{Codec, MaskedOpcode, Opcode, masked};
use crate::{Extension, Mnemonic};

pub(crate) static CUSTOM: [MaskedOpcode; 312] = [
    masked(
        0x5000302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvAbs, Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6000302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvExths, Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6200302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvExthz, Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6400302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvExtbs, Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6600302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvExtbz, Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSle, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5400302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSleu, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5600302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMin, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5800302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinu, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5a00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMax, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5c00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxu, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7400302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvClipr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7600302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvClipur, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddnr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddunr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8400302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddrnr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8600302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddurnr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8800302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubnr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8a00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubunr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8c00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubrnr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8e00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSuburnr, Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvClip, Codec::ISh5, "0,1,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvClipu, Codec::ISh5, "0,1,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x0000205b,
        0xc000707f,
        Opcode::new(Mnemonic::CvAddn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x4000205b,
        0xc000707f,
        Opcode::new(Mnemonic::CvAddun, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8000205b,
        0xc000707f,
        Opcode::new(Mnemonic::CvAddrn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0xc000205b,
        0xc000707f,
        Opcode::new(Mnemonic::CvAddurn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x0000305b,
        0xc000707f,
        Opcode::new(Mnemonic::CvSubn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x4000305b,
        0xc000707f,
        Opcode::new(Mnemonic::CvSubun, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8000305b,
        0xc000707f,
        Opcode::new(Mnemonic::CvSubrn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0xc000305b,
        0xc000707f,
        Opcode::new(Mnemonic::CvSuburn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x0000600b,
        0x0000707f,
        Opcode::new(Mnemonic::CvBeqimm, Codec::BImm5, "1,j,o")
            .requires(&[Extension::XCVbi])
            .rv32_only(),
    ),
    masked(
        0x0000700b,
        0x0000707f,
        Opcode::new(Mnemonic::CvBneimm, Codec::BImm5, "1,j,o")
            .requires(&[Extension::XCVbi])
            .rv32_only(),
    ),
    masked(
        0x0000005b,
        0xc000707f,
        Opcode::new(Mnemonic::CvExtract, Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4000005b,
        0xc000707f,
        Opcode::new(Mnemonic::CvExtractu, Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x8000005b,
        0xc000707f,
        Opcode::new(Mnemonic::CvInsert, Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x0000105b,
        0xc000707f,
        Opcode::new(Mnemonic::CvBclr, Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4000105b,
        0xc000707f,
        Opcode::new(Mnemonic::CvBset, Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0xc000105b,
        0xf800707f,
        Opcode::new(Mnemonic::CvBitrev, Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvExtractr, Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvExtractur, Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3400302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvInsertr, Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3800302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvBclrr, Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3a00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvBsetr, Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvRor, Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4200302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvFf1, Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4400302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvFl1, Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4600302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvClb, Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4800302b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvCnt, Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x0000300b,
        0x0000707f,
        Opcode::new(Mnemonic::CvElw, Codec::I, "0,i(1)")
            .requires(&[Extension::XCVelw])
            .rv32_only(),
    ),
    masked(
        0x9000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMac, Codec::R, "0,1,2")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x9200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMsu, Codec::R, "0,1,2")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000405b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulsn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000405b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulhhsn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000405b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulsrn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000405b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulhhsrn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000505b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulun, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000505b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulhhun, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000505b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulurn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000505b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMulhhurn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000605b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMacsn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000605b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMachhsn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000605b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMacsrn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000605b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMachhsrn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000705b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMacun, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000705b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMachhun, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000705b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMacurn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000705b,
        0xc000707f,
        Opcode::new(Mnemonic::CvMachhurn, Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000000b,
        0x0000707f,
        Opcode::new(Mnemonic::CvLb, Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000100b,
        0x0000707f,
        Opcode::new(Mnemonic::CvLh, Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000200b,
        0x0000707f,
        Opcode::new(Mnemonic::CvLw, Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000400b,
        0x0000707f,
        Opcode::new(Mnemonic::CvLbu, Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000500b,
        0x0000707f,
        Opcode::new(Mnemonic::CvLhu, Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLb, Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLbu, Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLh, Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLhu, Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0400302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLw, Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0800302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLb, Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1800302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLbu, Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0a00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLh, Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1a00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLhu, Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0c00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvLw, Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000002b,
        0x0000707f,
        Opcode::new(Mnemonic::CvSb, Codec::S, "2,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000102b,
        0x0000707f,
        Opcode::new(Mnemonic::CvSh, Codec::S, "2,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000202b,
        0x0000707f,
        Opcode::new(Mnemonic::CvSw, Codec::S, "2,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2000302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSb, Codec::R, "2,(1),0")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2200302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSh, Codec::R, "2,(1),0")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2400302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSw, Codec::R, "2,(1),0")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2800302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSb, Codec::R, "2,0(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2a00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSh, Codec::R, "2,0(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2c00302b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSw, Codec::R, "2,0(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAddSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAddSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSubSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSubSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvgH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvgB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvgScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvgScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAvgSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAvgSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvguH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvguB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvguScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAvguScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAvguSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAvguSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMinSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMinSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinuH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinuB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinuScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMinuScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMinuSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMinuSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMaxSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMaxSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxuH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxuB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxuScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvMaxuScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMaxuSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvMaxuSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSrlH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSrlB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSrlScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSrlScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSrlSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSrlSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSraH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSraB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSraScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSraScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSraSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSraSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSllH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSllB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSllScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSllScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSllSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSllSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvOrH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvOrB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvOrScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvOrScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvOrSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvOrSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvXorH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvXorB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvXorScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvXorScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvXorSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvXorSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAndH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAndB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAndScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAndScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAndSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvAndSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotupH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotupB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotupScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotupScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvDotupSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvDotupSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotuspH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotuspB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotuspScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotuspScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvDotuspSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvDotuspSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotspH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotspB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotspScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvDotspScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvDotspSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvDotspSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotupH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotupB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotupScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotupScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSdotupSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSdotupSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotuspH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotuspB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotuspScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotuspScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSdotuspSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSdotuspSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotspH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotspB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotspScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSdotspScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSdotspSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvSdotspSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7000007b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvAbsH, Codec::R, "0,1")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7000107b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvAbsB, Codec::R, "0,1")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800007b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvExtractH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800107b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvExtractB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800207b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvExtractuH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800307b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvExtractuB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800407b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvInsertH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800507b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvInsertB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvShuffleH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvShuffleB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvShuffleSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvShufflei0SciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvShufflei1SciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xd000707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvShufflei2SciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xd800707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvShufflei3SciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xe000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvShuffle2H, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xe000107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvShuffle2B, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xf000007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvPack, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xf200007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvPackH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xf800107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvPackloB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xfa00107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvPackhiB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpeqH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpeqB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpeqScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpeqScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpeqSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpeqSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpneH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpneB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpneScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpneScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpneSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpneSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgtSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgtSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgeSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgeSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpltSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpltSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpleSciH, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpleSciB, Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtuH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtuB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtuScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgtuScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgtuSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgtuSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeuH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeuB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeuScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpgeuScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgeuSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpgeuSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltuH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltuB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltuScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpltuScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpltuSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpltuSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleuH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00107b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleuB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleuScH, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00507b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCmpleuScB, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00607b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpleuSciH, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00707b,
        0xfc00707f,
        Opcode::new(Mnemonic::CvCmpleuSciB, Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulR, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulI, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400207b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulRDiv2, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600207b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulIDiv2, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulRDiv4, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulIDiv4, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400607b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulRDiv8, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600607b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvCplxmulIDiv8, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5c00007b,
        0xfff0707f,
        Opcode::new(Mnemonic::CvCplxconj, Codec::R, "0,1")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400007b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubrotmj, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400207b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubrotmjDiv2, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubrotmjDiv4, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400607b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubrotmjDiv8, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6c00207b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddDiv2, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7400207b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubDiv2, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6c00407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddDiv4, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7400407b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubDiv4, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6c00607b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvAddDiv8, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7400607b,
        0xfe00707f,
        Opcode::new(Mnemonic::CvSubDiv8, Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
];
