// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Operand fields of the instruction formats.
//!
//! The accessors shift the field to the top of the 64-bit word and then back
//! down, which isolates it and (for signed fields) sign-extends it in one step.

use crate::instruction::Instruction;
use crate::opcodes::Codec;
use crate::reg;

fn operand_rd(inst: u64) -> u32 {
    (inst << 52 >> 59) as u32
}
fn operand_rs1(inst: u64) -> u32 {
    (inst << 44 >> 59) as u32
}
fn operand_rs2(inst: u64) -> u32 {
    (inst << 39 >> 59) as u32
}
fn operand_rs3(inst: u64) -> u32 {
    (inst << 32 >> 59) as u32
}
fn operand_aq(inst: u64) -> u32 {
    (inst << 37 >> 63) as u32
}
fn operand_rl(inst: u64) -> u32 {
    (inst << 38 >> 63) as u32
}
fn operand_pred(inst: u64) -> u32 {
    (inst << 36 >> 60) as u32
}
fn operand_succ(inst: u64) -> u32 {
    (inst << 40 >> 60) as u32
}
fn operand_rm(inst: u64) -> u32 {
    (inst << 49 >> 61) as u32
}
fn operand_shamt5(inst: u64) -> u32 {
    (inst << 39 >> 59) as u32
}
fn operand_shamt6(inst: u64) -> u32 {
    (inst << 38 >> 58) as u32
}
fn operand_shamt7(inst: u64) -> u32 {
    (inst << 37 >> 57) as u32
}
fn operand_crdq(inst: u64) -> u32 {
    (inst << 59 >> 61) as u32
}
fn operand_crs1q(inst: u64) -> u32 {
    (inst << 54 >> 61) as u32
}
fn operand_crs1rdq(inst: u64) -> u32 {
    (inst << 54 >> 61) as u32
}
fn operand_crs2q(inst: u64) -> u32 {
    (inst << 59 >> 61) as u32
}
fn operand_crd(inst: u64) -> u32 {
    (inst << 52 >> 59) as u32
}
fn operand_crs1(inst: u64) -> u32 {
    (inst << 52 >> 59) as u32
}
fn operand_crs1rd(inst: u64) -> u32 {
    (inst << 52 >> 59) as u32
}
fn operand_crs2(inst: u64) -> u32 {
    (inst << 57 >> 59) as u32
}
fn operand_csr12(inst: u64) -> u32 {
    (inst << 32 >> 52) as u32
}
fn operand_imm12(inst: u64) -> i32 {
    ((inst as i64) << 32 >> 52) as i32
}
fn operand_imm20(inst: u64) -> i32 {
    (((inst as i64) << 32 >> 44) << 12) as i32
}
fn operand_jimm20(inst: u64) -> i32 {
    ((((inst as i64) << 32 >> 63) << 20) as u64
        | ((inst << 33 >> 54) << 1)
        | ((inst << 43 >> 63) << 11)
        | ((inst << 44 >> 56) << 12)) as i32
}
fn operand_simm12(inst: u64) -> i32 {
    ((((inst as i64) << 32 >> 57) << 5) as u64 | (inst << 52 >> 59)) as i32
}
fn operand_sbimm12(inst: u64) -> i32 {
    ((((inst as i64) << 32 >> 63) << 12) as u64
        | ((inst << 33 >> 58) << 5)
        | ((inst << 52 >> 60) << 1)
        | ((inst << 56 >> 63) << 11)) as i32
}
fn operand_cimmsh6(inst: u64) -> u32 {
    (((inst << 51 >> 63) << 5) | (inst << 57 >> 59)) as u32
}
fn operand_cimmi(inst: u64) -> i32 {
    ((((inst as i64) << 51 >> 63) << 5) as u64 | (inst << 57 >> 59)) as i32
}
fn operand_cimmui(inst: u64) -> i32 {
    ((((inst as i64) << 51 >> 63) << 17) as u64 | ((inst << 57 >> 59) << 12)) as i32
}
fn operand_cimmlwsp(inst: u64) -> u32 {
    (((inst << 51 >> 63) << 5) | ((inst << 57 >> 61) << 2) | ((inst << 60 >> 62) << 6)) as u32
}
fn operand_cimmldsp(inst: u64) -> u32 {
    (((inst << 51 >> 63) << 5) | ((inst << 57 >> 62) << 3) | ((inst << 59 >> 61) << 6)) as u32
}
fn operand_cimmlqsp(inst: u64) -> u32 {
    (((inst << 51 >> 63) << 5) | ((inst << 57 >> 63) << 4) | ((inst << 58 >> 60) << 6)) as u32
}
fn operand_cimm16sp(inst: u64) -> i32 {
    ((((inst as i64) << 51 >> 63) << 9) as u64
        | ((inst << 57 >> 63) << 4)
        | ((inst << 58 >> 63) << 6)
        | ((inst << 59 >> 62) << 7)
        | ((inst << 61 >> 63) << 5)) as i32
}
fn operand_cimmj(inst: u64) -> i32 {
    ((((inst as i64) << 51 >> 63) << 11) as u64
        | ((inst << 52 >> 63) << 4)
        | ((inst << 53 >> 62) << 8)
        | ((inst << 55 >> 63) << 10)
        | ((inst << 56 >> 63) << 6)
        | ((inst << 57 >> 63) << 7)
        | ((inst << 58 >> 61) << 1)
        | ((inst << 61 >> 63) << 5)) as i32
}
fn operand_cimmb(inst: u64) -> i32 {
    ((((inst as i64) << 51 >> 63) << 8) as u64
        | ((inst << 52 >> 62) << 3)
        | ((inst << 57 >> 62) << 6)
        | ((inst << 59 >> 62) << 1)
        | ((inst << 61 >> 63) << 5)) as i32
}
fn operand_cimmswsp(inst: u64) -> u32 {
    (((inst << 51 >> 60) << 2) | ((inst << 55 >> 62) << 6)) as u32
}
fn operand_cimmsdsp(inst: u64) -> u32 {
    (((inst << 51 >> 61) << 3) | ((inst << 54 >> 61) << 6)) as u32
}
fn operand_cimmsqsp(inst: u64) -> u32 {
    (((inst << 51 >> 62) << 4) | ((inst << 53 >> 60) << 6)) as u32
}
fn operand_cimm4spn(inst: u64) -> u32 {
    (((inst << 51 >> 62) << 4)
        | ((inst << 53 >> 60) << 6)
        | ((inst << 57 >> 63) << 2)
        | ((inst << 58 >> 63) << 3)) as u32
}
fn operand_cimmw(inst: u64) -> u32 {
    (((inst << 51 >> 61) << 3) | ((inst << 57 >> 63) << 2) | ((inst << 58 >> 63) << 6)) as u32
}
fn operand_cimmd(inst: u64) -> u32 {
    (((inst << 51 >> 61) << 3) | ((inst << 57 >> 62) << 6)) as u32
}
fn operand_cimmq(inst: u64) -> u32 {
    (((inst << 51 >> 62) << 4) | ((inst << 53 >> 63) << 8) | ((inst << 57 >> 62) << 6)) as u32
}

/// Extracts the operands of `inst` into `ins` according to the layout given by `codec`.
pub(super) fn extract(ins: &mut Instruction, codec: Codec) {
    let inst = ins.inst;
    match codec {
        Codec::None => {
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.rd = reg::ZERO;
            ins.imm = 0;
        }
        Codec::U => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.imm = operand_imm20(inst);
        }
        Codec::Uj => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.imm = operand_jimm20(inst);
        }
        Codec::I => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_imm12(inst);
        }
        Codec::ISh5 => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_shamt5(inst) as i32;
        }
        Codec::ISh6 => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_shamt6(inst) as i32;
        }
        Codec::ISh7 => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_shamt7(inst) as i32;
        }
        Codec::ICsr => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_csr12(inst) as i32;
        }
        Codec::S => {
            ins.rd = reg::ZERO;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = operand_rs2(inst) as u8;
            ins.imm = operand_simm12(inst);
        }
        Codec::SB => {
            ins.rd = reg::ZERO;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = operand_rs2(inst) as u8;
            ins.imm = operand_sbimm12(inst);
        }
        Codec::R => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = operand_rs2(inst) as u8;
            ins.imm = 0;
        }
        Codec::RM => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = operand_rs2(inst) as u8;
            ins.imm = 0;
            ins.rm = operand_rm(inst) as u8;
        }
        Codec::R4M => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = operand_rs2(inst) as u8;
            ins.rs3 = operand_rs3(inst) as u8;
            ins.imm = 0;
            ins.rm = operand_rm(inst) as u8;
        }
        Codec::RA => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = operand_rs2(inst) as u8;
            ins.imm = 0;
            ins.aq = operand_aq(inst) != 0;
            ins.rl = operand_rl(inst) != 0;
        }
        Codec::RL => {
            ins.rd = operand_rd(inst) as u8;
            ins.rs1 = operand_rs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = 0;
            ins.aq = operand_aq(inst) != 0;
            ins.rl = operand_rl(inst) != 0;
        }
        Codec::RF => {
            ins.rd = reg::ZERO;
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.pred = operand_pred(inst) as u8;
            ins.succ = operand_succ(inst) as u8;
            ins.imm = 0;
        }
        Codec::Cb => {
            ins.rd = reg::ZERO;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmb(inst);
        }
        Codec::CbImm => {
            ins.rs1 = (operand_crs1rdq(inst)).wrapping_add(8) as u8;
            ins.rd = ins.rs1;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmi(inst);
        }
        Codec::CbSh6 => {
            ins.rs1 = (operand_crs1rdq(inst)).wrapping_add(8) as u8;
            ins.rd = ins.rs1;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmsh6(inst) as i32;
        }
        Codec::Ci => {
            ins.rs1 = operand_crs1rd(inst) as u8;
            ins.rd = ins.rs1;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmi(inst);
        }
        Codec::CiSh6 => {
            ins.rs1 = operand_crs1rd(inst) as u8;
            ins.rd = ins.rs1;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmsh6(inst) as i32;
        }
        Codec::Ci16sp => {
            ins.rd = reg::SP;
            ins.rs1 = reg::SP;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimm16sp(inst);
        }
        Codec::CiLwsp => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = reg::SP;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmlwsp(inst) as i32;
        }
        Codec::CiLdsp => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = reg::SP;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmldsp(inst) as i32;
        }
        Codec::CiLqsp => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = reg::SP;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmlqsp(inst) as i32;
        }
        Codec::CiLi => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = reg::ZERO;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmi(inst);
        }
        Codec::CiLui => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = reg::ZERO;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmui(inst);
        }
        Codec::CiNone => {
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.rd = reg::ZERO;
            ins.imm = 0;
        }
        Codec::Ciw4spn => {
            ins.rd = (operand_crdq(inst)).wrapping_add(8) as u8;
            ins.rs1 = reg::SP;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimm4spn(inst) as i32;
        }
        Codec::Cj => {
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.rd = reg::ZERO;
            ins.imm = operand_cimmj(inst);
        }
        Codec::CjJal => {
            ins.rd = reg::RA;
            ins.rs2 = reg::ZERO;
            ins.rs1 = reg::ZERO;
            ins.imm = operand_cimmj(inst);
        }
        Codec::ClLw => {
            ins.rd = (operand_crdq(inst)).wrapping_add(8) as u8;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmw(inst) as i32;
        }
        Codec::ClLd => {
            ins.rd = (operand_crdq(inst)).wrapping_add(8) as u8;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmd(inst) as i32;
        }
        Codec::ClLq => {
            ins.rd = (operand_crdq(inst)).wrapping_add(8) as u8;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_cimmq(inst) as i32;
        }
        Codec::Cr => {
            ins.rs1 = operand_crs1rd(inst) as u8;
            ins.rd = ins.rs1;
            ins.rs2 = operand_crs2(inst) as u8;
            ins.imm = 0;
        }
        Codec::CrMv => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = operand_crs2(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = 0;
        }
        Codec::CrJalr => {
            ins.rd = reg::RA;
            ins.rs1 = operand_crs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = 0;
        }
        Codec::CrJr => {
            ins.rd = reg::ZERO;
            ins.rs1 = operand_crs1(inst) as u8;
            ins.rs2 = reg::ZERO;
            ins.imm = 0;
        }
        Codec::Cs => {
            ins.rs1 = (operand_crs1rdq(inst)).wrapping_add(8) as u8;
            ins.rd = ins.rs1;
            ins.rs2 = (operand_crs2q(inst)).wrapping_add(8) as u8;
            ins.imm = 0;
        }
        Codec::CsSw => {
            ins.rd = reg::ZERO;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = (operand_crs2q(inst)).wrapping_add(8) as u8;
            ins.imm = operand_cimmw(inst) as i32;
        }
        Codec::CsSd => {
            ins.rd = reg::ZERO;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = (operand_crs2q(inst)).wrapping_add(8) as u8;
            ins.imm = operand_cimmd(inst) as i32;
        }
        Codec::CsSq => {
            ins.rd = reg::ZERO;
            ins.rs1 = (operand_crs1q(inst)).wrapping_add(8) as u8;
            ins.rs2 = (operand_crs2q(inst)).wrapping_add(8) as u8;
            ins.imm = operand_cimmq(inst) as i32;
        }
        Codec::CssSwsp => {
            ins.rd = reg::ZERO;
            ins.rs1 = reg::SP;
            ins.rs2 = operand_crs2(inst) as u8;
            ins.imm = operand_cimmswsp(inst) as i32;
        }
        Codec::CssSdsp => {
            ins.rd = reg::ZERO;
            ins.rs1 = reg::SP;
            ins.rs2 = operand_crs2(inst) as u8;
            ins.imm = operand_cimmsdsp(inst) as i32;
        }
        Codec::CssSqsp => {
            ins.rd = reg::ZERO;
            ins.rs1 = reg::SP;
            ins.rs2 = operand_crs2(inst) as u8;
            ins.imm = operand_cimmsqsp(inst) as i32;
        }
        Codec::Li => {
            ins.rd = operand_crd(inst) as u8;
            ins.rs1 = reg::ZERO;
            ins.rs2 = reg::ZERO;
            ins.imm = operand_imm12(inst);
        }
        Codec::Illegal => {}
    }
}
