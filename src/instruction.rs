//! The decoded instruction type.

use crate::opcodes::{Opcode, ILLEGAL};

/// A decoded instruction.
///
/// Create one with [`decode`](crate::decode) or by iterating over
/// [`disassemble`](crate::disassemble).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub(crate) pc: u64,
    pub(crate) inst: u64,
    pub(crate) op: &'static Opcode,
    pub(crate) rd: u8,
    pub(crate) rs1: u8,
    pub(crate) rs2: u8,
    pub(crate) rs3: u8,
    pub(crate) rm: u8,
    pub(crate) pred: u8,
    pub(crate) succ: u8,
    pub(crate) aq: u8,
    pub(crate) rl: u8,
    pub(crate) imm: i32,
}

impl Instruction {
    pub(crate) fn new(pc: u64, inst: u64, op: &'static Opcode) -> Self {
        Instruction {
            pc,
            inst,
            op,
            rd: 0,
            rs1: 0,
            rs2: 0,
            rs3: 0,
            rm: 0,
            pred: 0,
            succ: 0,
            aq: 0,
            rl: 0,
            imm: 0,
        }
    }

    /// The address of the instruction.
    pub fn pc(&self) -> u64 {
        self.pc
    }

    /// The instruction encoding, in the low [`length`](Self::length) bytes.
    pub fn raw(&self) -> u64 {
        self.inst
    }

    /// The length of the encoding in bytes; see [`inst_length`](crate::inst_length).
    pub fn length(&self) -> usize {
        crate::inst_length(self.inst)
    }

    /// The mnemonic, such as `"addi"` or `"c.addi"`, or `"illegal"`.
    pub fn mnemonic(&self) -> &'static str {
        self.op.name
    }

    /// Whether the encoding was not recognized.
    pub fn is_illegal(&self) -> bool {
        *self.op == ILLEGAL
    }
}
