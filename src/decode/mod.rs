// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Decoding: encoding → opcode → operands → expansion → pseudoinstruction.

mod opcode;
mod operands;

use crate::instruction::Instruction;
use crate::opcodes::{Codec, Constraint, ILLEGAL, RESERVED_PARCEL};
use crate::{Extension, Extensions, Isa};

/// Decodes the instruction `inst` located at address `pc`, for `isa` with
/// the extensions in `exts`.
pub(crate) fn decode(isa: Isa, exts: Extensions, pc: u64, inst: u64) -> Instruction {
    let inst = match crate::inst_length(inst as u16) {
        Some(2) => inst & 0xffff,
        Some(len) => inst & (u64::MAX >> (64 - 8 * len)),
        None => return Instruction::new(isa, pc, inst & 0xffff, &RESERVED_PARCEL),
    };
    let op = opcode::lookup(isa, exts, inst).unwrap_or(&ILLEGAL);
    let mut ins = Instruction::new(isa, pc, inst, op);
    ins.fp_in_x = exts.contains(Extension::Zfinx);
    operands::extract(&mut ins, op.codec);
    if is_reserved(&ins, exts) {
        return Instruction::new(isa, pc, inst, &ILLEGAL);
    }
    // HINTs are shown as encoded, as objdump does, unless they have an alias
    // (such as c.ntl.p1 for a c.add HINT).
    if op.hint_if.iter().any(|&c| holds(&ins, c)) {
        lift_pseudo(&mut ins, exts);
        return ins;
    }
    if let Some(expanded) = op.decompress[isa as usize] {
        ins.op = expanded;
    }
    lift_pseudo(&mut ins, exts);
    ins
}

/// Whether the decoded opcode and operands are not a valid instruction for
/// the ISA and extensions.
fn is_reserved(ins: &Instruction, exts: Extensions) -> bool {
    let op = ins.op;
    let shift_too_big = matches!(
        op.codec,
        Codec::ISh5 | Codec::ISh6 | Codec::ISh7 | Codec::CiSh6 | Codec::CbSh6
    ) && ins.imm >= ins.isa.xlen() as i32;
    !op.exists_in(ins.isa, exts)
        || shift_too_big
        || op.illegal_if.iter().any(|&c| holds(ins, c))
        || (exts.contains(Extension::E) && uses_upper_registers(ins))
        || (ins.isa == Isa::Rv32 && exts.contains(Extension::Zdinx) && odd_register_pair(ins))
}

/// Whether `ins` uses one of the integer registers x16-x31, which RV32E and
/// RV64E do not have: as an operand, or (for cm.push and cm.pop) by saving
/// s2-s11.
fn uses_upper_registers(ins: &Instruction) -> bool {
    let operand = ins.op.format.chars().any(|c| match c {
        '0' => ins.rd >= 16,
        '1' => ins.rs1 >= 16,
        '2' => ins.rs2 >= 16,
        '3' if ins.fp_in_x => ins.rd >= 16,
        '4' if ins.fp_in_x => ins.rs1 >= 16,
        '5' if ins.fp_in_x => ins.rs2 >= 16,
        '6' if ins.fp_in_x => ins.rs3 >= 16,
        _ => false,
    });
    // The register list (in rs1) saves s2 and up from 7.
    operand || (ins.op.codec == Codec::CmPushPop && ins.rs1 > 6)
}

/// Whether `ins` has a double-precision operand in an odd register. With
/// Zdinx on RV32, these are even-odd register pairs, so odd registers are
/// reserved.
fn odd_register_pair(ins: &Instruction) -> bool {
    // The operands' formats are in the mnemonic: fcvt.<rd>.<rs1>, or a
    // single suffix for all of them (fadd.d).
    let name = ins.op.name;
    let (rd, rs) = match name.strip_prefix("fcvt.").and_then(|t| t.split_once('.')) {
        Some((to, from)) => (to == "d", from == "d"),
        None => (name.ends_with(".d"), name.ends_with(".d")),
    };
    ins.op.format.chars().any(|c| match c {
        '3' => rd && ins.rd & 1 == 1,
        '4' => rs && ins.rs1 & 1 == 1,
        '5' => rs && ins.rs2 & 1 == 1,
        '6' => rs && ins.rs3 & 1 == 1,
        _ => false,
    })
}

/// Replaces an instruction with the first of its pseudoinstructions whose
/// extensions are enabled and whose constraints all hold.
fn lift_pseudo(ins: &mut Instruction, exts: Extensions) {
    if let Some(p) = ins
        .op
        .pseudo
        .iter()
        .find(|p| p.op.provided_by(exts) && p.when.iter().all(|&c| holds(ins, c)))
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
        Constraint::RdEqRs1 => ins.rd == ins.rs1,
        Constraint::RdOdd => ins.rd & 1 == 1,
        Constraint::Rs2Odd => ins.rs2 & 1 == 1,
        Constraint::ImmEq(imm) | Constraint::CsrEq(imm) => ins.imm == imm,
        Constraint::ImmMaskEq(mask, value) => ins.imm & mask == value,
    }
}
