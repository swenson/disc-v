//! A RISC-V disassembler.
//!
//! ```
//! use disc_v::{decode, Isa};
//!
//! let ins = decode(Isa::Rv32, 0x1000, 0x10018193);
//! assert_eq!(ins.mnemonic(), "addi");
//! assert_eq!(ins.listing().to_string(), "10018193          addi          gp,gp,256");
//! ```
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod csr;
mod decode;
mod format;
mod instruction;
mod opcodes;
mod reg;

pub use format::Listing;
pub use instruction::Instruction;

/// The base integer ISA, which determines the register width and which
/// encodings are valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Isa {
    Rv32,
    Rv64,
    Rv128,
}

/// Decodes the instruction `inst`, located at address `pc`.
///
/// Only the low [`inst_length(inst)`](inst_length) bytes of `inst` are part
/// of the instruction.
pub fn decode(isa: Isa, pc: u64, inst: u64) -> Instruction {
    decode::decode(isa, pc, inst)
}

/// Returns the length in bytes of the instruction whose encoding starts with
/// `inst`, or 0 for reserved lengths of 10 bytes or more.
///
/// Only the low 16 bits of `inst` are examined.
pub fn inst_length(inst: u64) -> usize {
    if inst & 3 != 3 {
        2
    } else if inst & 0x1c != 0x1c {
        4
    } else if inst & 0x3f == 0x1f {
        6
    } else if inst & 0x7f == 0x3f {
        8
    } else {
        0
    }
}

/// Decodes `inst` and formats it as a [`Listing`] line.
#[cfg(feature = "alloc")]
pub fn disasm_inst(isa: Isa, pc: u64, inst: u64) -> alloc::string::String {
    use alloc::string::ToString;
    decode(isa, pc, inst).listing().to_string()
}
