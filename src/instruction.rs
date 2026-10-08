// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! The decoded instruction type.

use crate::Isa;
use crate::format::Operands;
use crate::opcodes::c::C_UNIMP;
use crate::opcodes::{Codec, Opcode};

/// A decoded instruction.
///
/// Create one with [`decode`](fn@crate::decode),
/// [`decode_bytes`](crate::decode_bytes), or by iterating over
/// [`disassemble`](crate::disassemble). Its [`Display`](core::fmt::Display)
/// output follows GNU objdump's for a raw binary, such as `addi gp,gp,256`
/// or `beqz a0,0x1040`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub(crate) isa: Isa,
    pub(crate) pc: u64,
    pub(crate) inst: u64,
    pub(crate) len: u8,
    /// The opcode to show: expanded if compressed, and aliased if a
    /// pseudoinstruction applies.
    pub(crate) op: &'static Opcode,
    /// The opcode as encoded, before expansion and aliasing.
    pub(crate) decoded: &'static Opcode,
    pub(crate) rd: u8,
    pub(crate) rs1: u8,
    pub(crate) rs2: u8,
    pub(crate) rs3: u8,
    pub(crate) rm: u8,
    pub(crate) pred: u8,
    pub(crate) succ: u8,
    pub(crate) aq: bool,
    pub(crate) rl: bool,
    pub(crate) imm: i32,
    /// For vector instructions, whether the operation is masked by `v0`.
    pub(crate) masked: bool,
    /// Whether floating-point operands are in the integer registers (Zfinx).
    pub(crate) fp_in_x: bool,
}

impl Instruction {
    pub(crate) fn new(isa: Isa, pc: u64, inst: u64, op: &'static Opcode) -> Self {
        // Reserved lengths are shown one 16-bit parcel at a time so that
        // disassembly can continue.
        let len = crate::inst_length(inst as u16).unwrap_or(2) as u8;
        Instruction {
            isa,
            pc,
            inst,
            len,
            op,
            decoded: op,
            rd: 0,
            rs1: 0,
            rs2: 0,
            rs3: 0,
            rm: 0,
            pred: 0,
            succ: 0,
            aq: false,
            rl: false,
            imm: 0,
            masked: false,
            fp_in_x: false,
        }
    }

    /// The base ISA the instruction was decoded for.
    pub fn isa(&self) -> Isa {
        self.isa
    }

    /// The address of the instruction.
    pub fn pc(&self) -> u64 {
        self.pc
    }

    /// The instruction encoding, in the low [`length`](Self::length) bytes.
    pub fn raw(&self) -> u64 {
        self.inst
    }

    /// The length of the encoding in bytes: 2, 4, 6 or 8.
    ///
    /// Encodings with a reserved length (10 bytes or more) are decoded as a
    /// 2-byte illegal instruction.
    pub fn length(&self) -> usize {
        self.len as usize
    }

    /// The mnemonic, such as `"addi"`, `"li"` or `"amoadd.w"`.
    ///
    /// An encoding that is not a valid instruction is shown as objdump shows
    /// it, as an `.insn` directive (`.insn 4, 0x0911003b`), or as a `.2byte`
    /// directive for a parcel of a reserved-length encoding. Use
    /// [`is_illegal`](Self::is_illegal) to check for these.
    ///
    /// Compressed instructions are shown as the instruction they expand to,
    /// and aliases (pseudoinstructions) are used where objdump uses them.
    /// [`without_aliases`](Self::without_aliases) gives the instruction as
    /// encoded. The mnemonic does not include the `.aq`/`.rl` ordering
    /// suffixes of atomic instructions, which [`Display`](core::fmt::Display)
    /// adds.
    pub fn mnemonic(&self) -> &'static str {
        self.op.name
    }

    /// The operands, without the mnemonic, such as `a0,8(sp)`.
    pub fn operands(&self) -> Operands<'_> {
        Operands(self)
    }

    /// Whether the encoding is not a valid instruction. This includes the
    /// all-zeros 16-bit encoding, which the ISA defines to be illegal and
    /// which is shown as `unimp`.
    pub fn is_illegal(&self) -> bool {
        self.op.codec == Codec::Illegal || *self.decoded == C_UNIMP
    }

    /// The same instruction, shown as encoded: compressed instructions keep
    /// their `c.` names and pseudoinstructions are not used. This matches
    /// `objdump -M no-aliases`.
    pub fn without_aliases(&self) -> Instruction {
        Instruction {
            op: self.decoded,
            ..*self
        }
    }

    /// The target address of a PC-relative operand, wrapped to the
    /// register width on RV32. Addresses are 64 bits, so on RV128 it is the
    /// low 64 bits of the target.
    pub(crate) fn target(&self) -> u64 {
        let target = self.pc.wrapping_add(self.imm as i64 as u64);
        match self.isa {
            Isa::Rv32 => target & 0xffff_ffff,
            Isa::Rv64 | Isa::Rv128 => target,
        }
    }
}
