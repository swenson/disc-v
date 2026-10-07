// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! A RISC-V disassembler.
//!
//! Supports RV32, RV64 and RV128 with these extensions:
//!
//! - M, A, F, D, Q and C
//! - Zicsr and Zifencei
//! - Zba, Zbb, Zbc and Zbs
//! - Zicond
//! - Zawrs
//! - Zicbom, Zicboz and Zicbop; Zihintntl and Zihintpause
//! - Zimop and Zcmop, with the Zicfiss and Zicfilp instructions
//! - Zcb
//! - Zfh and Zfhmin
//! - Zfa
//! - V (vectors)
//! - Zvbb and Zvbc; Zvkg, Zvkned, Zvknha, Zvknhb, Zvksed and Zvksh
//! - H (hypervisor), Svinval, Smrnmi and Ssctr
//! - Zalasr, Zacas and Zabha
//! - Zfbfmin, Zvfbfmin and Zvfbfwma (BFloat16)
//!
//! The text follows GNU objdump's for a raw binary (`objdump -D -b binary`),
//! so branch targets are written as `0x1008`.
//!
//! [`decode`](fn@decode), [`decode_bytes`] and [`disassemble`] decode every
//! supported extension that does not conflict with another. A [`Decoder`]
//! decodes for a particular target instead, such as [`Decoder::RVA23U64`] or
//! one from an ISA string with [`Decoder::from_march`].
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
#![no_std]
#![warn(missing_docs)]

mod csr;
mod decode;
mod decoder;
mod extension;
mod format;
mod instruction;
mod opcodes;
mod reg;

pub use decoder::{Conflict, Decoder, MarchError};
pub use extension::{Extension, Extensions};
pub use format::Operands;
pub use instruction::Instruction;

// Compiles and runs the README's examples as doc tests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// The base integer ISA, which determines the register width and which
/// encodings are valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Isa {
    /// RV32: 32-bit registers.
    Rv32,
    /// RV64: 64-bit registers.
    Rv64,
    /// RV128: 128-bit registers. The RV128 encodings are not yet ratified.
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
/// Only the low [`inst_length`] bytes of `inst` are part of the instruction; RISC-V encodings are little-endian, so this is the
/// value of those bytes read as a little-endian integer.
///
/// ```
/// use disc_v::{decode, Isa};
///
/// let ins = decode(Isa::Rv32, 0x1000, 0xfe050ee3);
/// assert_eq!(ins.to_string(), "beqz a0,0xffc");
/// ```
pub fn decode(isa: Isa, pc: u64, inst: u64) -> Instruction {
    Decoder::new(isa).decode(pc, inst)
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
    Decoder::new(isa).decode_bytes(pc, bytes)
}

/// Disassembles `bytes`, the first of which is at address `pc`.
///
/// Iteration stops when the remaining bytes are too short for the next
/// instruction; see [`Disassembler::remainder`].
pub fn disassemble(isa: Isa, pc: u64, bytes: &[u8]) -> Disassembler<'_> {
    Decoder::new(isa).disassemble(pc, bytes)
}

/// An iterator over the instructions in a byte slice. Create one with
/// [`disassemble`] or [`Decoder::disassemble`].
#[derive(Clone, Debug)]
pub struct Disassembler<'a> {
    decoder: Decoder,
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
        let ins = self.decoder.decode_bytes(self.pc, self.bytes)?;
        self.bytes = &self.bytes[ins.length()..];
        self.pc = self.pc.wrapping_add(ins.length() as u64);
        Some(ins)
    }
}

impl core::iter::FusedIterator for Disassembler<'_> {}

/// Returns the length in bytes of the instruction whose first 16-bit parcel
/// is `parcel`, or `None` for the reserved lengths of 10 bytes or more.
///
/// ```
/// use disc_v::inst_length;
///
/// assert_eq!(inst_length(0x4505), Some(2)); // li a0,1
/// assert_eq!(inst_length(0x0513), Some(4)); // the low half of li a0,10
/// assert_eq!(inst_length(0x007f), None);
/// ```
pub fn inst_length(parcel: u16) -> Option<usize> {
    if parcel & 3 != 3 {
        Some(2)
    } else if parcel & 0x1c != 0x1c {
        Some(4)
    } else if parcel & 0x3f == 0x1f {
        Some(6)
    } else if parcel & 0x7f == 0x3f {
        Some(8)
    } else {
        None
    }
}
