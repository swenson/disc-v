// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Decoding: encoding → opcode → operands → expansion → pseudoinstruction.

mod opcode;
mod operands;

use crate::Isa;
use crate::instruction::Instruction;
use crate::opcodes::{Codec, Constraint, ILLEGAL, RESERVED_PARCEL};

/// Decodes the instruction `inst` located at address `pc`.
pub(crate) fn decode(isa: Isa, pc: u64, inst: u64) -> Instruction {
    let inst = match crate::inst_length(inst as u16) {
        Some(2) => inst & 0xffff,
        Some(len) => inst & (u64::MAX >> (64 - 8 * len)),
        None => return Instruction::new(isa, pc, inst & 0xffff, &RESERVED_PARCEL),
    };
    let op = opcode::lookup(isa, inst).unwrap_or(&ILLEGAL);
    let mut ins = Instruction::new(isa, pc, inst, op);
    operands::extract(&mut ins, op.codec);
    if is_reserved(&ins) {
        return Instruction::new(isa, pc, inst, &ILLEGAL);
    }
    // HINTs are shown as encoded, as objdump does.
    if op.hint_if.iter().any(|&c| holds(&ins, c)) {
        return ins;
    }
    if let Some(expanded) = op.decompress[isa as usize] {
        ins.op = expanded;
    }
    lift_pseudo(&mut ins);
    ins
}

/// Whether the decoded opcode and operands are not a valid instruction for
/// the ISA.
fn is_reserved(ins: &Instruction) -> bool {
    let op = ins.op;
    let shift_too_big = matches!(
        op.codec,
        Codec::ISh5 | Codec::ISh6 | Codec::ISh7 | Codec::CiSh6 | Codec::CbSh6
    ) && ins.imm >= ins.isa.xlen() as i32;
    !op.exists_in(ins.isa) || shift_too_big || op.illegal_if.iter().any(|&c| holds(ins, c))
}

/// Replaces an instruction with the first of its pseudoinstructions whose
/// constraints all hold.
fn lift_pseudo(ins: &mut Instruction) {
    if let Some(p) = ins
        .op
        .pseudo
        .iter()
        .find(|p| p.when.iter().all(|&c| holds(ins, c)))
    {
        ins.op = p.op;
    }
}

fn holds(ins: &Instruction, c: Constraint) -> bool {
    match c {
        Constraint::RdEq(r) => ins.rd == r,
        Constraint::Rs1Eq(r) => ins.rs1 == r,
        Constraint::Rs2Eq(r) => ins.rs2 == r,
        Constraint::Rs2EqRs1 => ins.rs2 == ins.rs1,
        Constraint::ImmEq(imm) | Constraint::CsrEq(imm) => ins.imm == imm,
    }
}
