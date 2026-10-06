//! A RISC-V disassembler.
//!
//! Supports RV32, RV64 and RV128 with the I, M, A, F, D, Q and C extensions,
//! Zicsr, Zifencei, and the bit-manipulation extensions Zba, Zbb, Zbc and
//! Zbs. Output matches GNU objdump's.
//!
//! ```
//! use disc_v::{disassemble, Isa};
//!
//! let code = [0x13, 0x05, 0x10, 0x00, 0x82, 0x80]; // li a0,1; ret
//! for ins in disassemble(Isa::Rv64, 0x1000, &code) {
//!     println!("{:x}: {ins}", ins.pc());
//! }
//! # let text: Vec<_> = disassemble(Isa::Rv64, 0x1000, &code).map(|i| i.to_string()).collect();
//! # assert_eq!(text, ["li a0,1", "ret"]);
//! ```
//!
//! The crate is `no_std` and does not allocate.
#![no_std]

mod csr;
mod decode;
mod format;
mod instruction;
mod opcodes;
mod reg;

pub use format::Operands;
pub use instruction::Instruction;

/// The base integer ISA, which determines the register width and which
/// encodings are valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Isa {
    Rv32,
    Rv64,
    Rv128,
}

impl Isa {
    /// The width of the integer registers in bits.
    pub fn xlen(self) -> u32 {
        match self {
            Isa::Rv32 => 32,
            Isa::Rv64 => 64,
            Isa::Rv128 => 128,
        }
    }
}

/// Decodes the instruction `inst`, located at address `pc`.
///
/// Only the low [`inst_length(inst)`](inst_length) bytes of `inst` are part
/// of the instruction; RISC-V encodings are little-endian, so this is the
/// value of those bytes read as a little-endian integer.
///
/// ```
/// use disc_v::{decode, Isa};
///
/// let ins = decode(Isa::Rv32, 0x1000, 0xfe050ee3);
/// assert_eq!(ins.to_string(), "beqz a0,0xffc");
/// ```
pub fn decode(isa: Isa, pc: u64, inst: u64) -> Instruction {
    decode::decode(isa, pc, inst)
}

/// Decodes the instruction at the start of `bytes`, located at address `pc`.
///
/// Returns `None` if `bytes` is shorter than the instruction.
///
/// ```
/// use disc_v::{decode_bytes, Isa};
///
/// let ins = decode_bytes(Isa::Rv64, 0, &[0x93, 0x81, 0x01, 0x10]).unwrap();
/// assert_eq!(ins.to_string(), "addi gp,gp,256");
/// assert_eq!(ins.length(), 4);
/// ```
pub fn decode_bytes(isa: Isa, pc: u64, bytes: &[u8]) -> Option<Instruction> {
    let first = u16::from_le_bytes([*bytes.first()?, *bytes.get(1)?]) as u64;
    let len = inst_length(first).max(2);
    let mut word = [0; 8];
    word[..len].copy_from_slice(bytes.get(..len)?);
    Some(decode(isa, pc, u64::from_le_bytes(word)))
}

/// Disassembles `bytes`, the first of which is at address `pc`.
///
/// Iteration stops when the remaining bytes are too short for the next
/// instruction; see [`Disassembler::remainder`].
pub fn disassemble(isa: Isa, pc: u64, bytes: &[u8]) -> Disassembler<'_> {
    Disassembler { isa, pc, bytes }
}

/// An iterator over the instructions in a byte slice. Create one with
/// [`disassemble`].
#[derive(Clone, Debug)]
pub struct Disassembler<'a> {
    isa: Isa,
    pc: u64,
    bytes: &'a [u8],
}

impl<'a> Disassembler<'a> {
    /// The bytes not yet disassembled. After iteration ends, these are the
    /// bytes of a truncated final instruction, if any.
    pub fn remainder(&self) -> &'a [u8] {
        self.bytes
    }

    /// The address of the next instruction.
    pub fn pc(&self) -> u64 {
        self.pc
    }
}

impl Iterator for Disassembler<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {
        let ins = decode_bytes(self.isa, self.pc, self.bytes)?;
        self.bytes = &self.bytes[ins.length()..];
        self.pc = self.pc.wrapping_add(ins.length() as u64);
        Some(ins)
    }
}

impl core::iter::FusedIterator for Disassembler<'_> {}

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
