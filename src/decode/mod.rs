//! Decoding: encoding → opcode → operands → expansion → pseudoinstruction.

mod opcode;
mod operands;

use crate::instruction::Instruction;
use crate::opcodes::{Constraint, ILLEGAL};
use crate::Isa;

/// Decodes the instruction `inst` located at address `pc`.
pub(crate) fn decode(isa: Isa, pc: u64, inst: u64) -> Instruction {
    let op = opcode::lookup(isa, inst).unwrap_or(&ILLEGAL);
    let mut ins = Instruction::new(pc, inst, op);
    operands::extract(&mut ins, op.codec);
    decompress(&mut ins, isa);
    lift_pseudo(&mut ins);
    ins
}

/// Replaces a compressed instruction with the instruction it expands to.
fn decompress(ins: &mut Instruction, isa: Isa) {
    if let Some(expanded) = ins.op.decompress[isa as usize] {
        ins.op = if ins.op.check_imm_nz && ins.imm == 0 {
            &ILLEGAL
        } else {
            expanded
        };
    }
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
