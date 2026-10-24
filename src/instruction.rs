// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! The decoded instruction type.

use crate::format::Operands;
use crate::opcodes::c::C_UNIMP;
use crate::opcodes::{Codec, Opcode};
use crate::{Isa, Mnemonic};

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
    /// A second immediate, for instructions with two.
    pub(crate) imm2: i32,
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
            imm2: 0,
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
        self.op.mnemonic.name()
    }

    /// The mnemonic, as a type: the [`Mnemonic`] whose
    /// [`name`](Mnemonic::name) [`mnemonic`](Self::mnemonic) returns.
    ///
    /// ```
    /// use disc_v::{decode, Isa, Mnemonic};
    ///
    /// let ins = decode(Isa::Rv64, 0, 0x4505); // c.li a0,1
    /// assert_eq!(ins.id(), Mnemonic::Li);
    /// assert_eq!(ins.without_aliases().id(), Mnemonic::CLi);
    /// ```
    pub fn id(&self) -> Mnemonic {
        self.op.mnemonic
    }

    /// The destination register's number, 0 to 31: an integer,
    /// floating-point or vector register, as the instruction's operands
    /// are. 0 for an instruction with no destination.
    ///
    /// The register fields are as the instruction is executed, not as it
    /// is encoded: a compressed instruction's 3-bit register field gives
    /// `x8` to `x15`, and a register the encoding implies is filled in, such
    /// as `sp` for `c.lwsp` and `ra` for `c.jalr`.
    pub fn rd(&self) -> u8 {
        self.rd
    }

    /// The first source register's number, as [`rd`](Self::rd) describes.
    /// 0 for an instruction with no such source.
    ///
    /// For `cm.push` and `cm.pop*` it is the register-list field, and for
    /// `cm.mvsa01` and `cm.mva01s` the first s-register.
    pub fn rs1(&self) -> u8 {
        self.rs1
    }

    /// The second source register's number, as [`rd`](Self::rd) describes.
    /// 0 for an instruction with no such source.
    pub fn rs2(&self) -> u8 {
        self.rs2
    }

    /// The third source register's number, of a fused multiply-add. 0 for
    /// other instructions.
    pub fn rs3(&self) -> u8 {
        self.rs3
    }

    /// The immediate, as the instruction uses it, or 0 for an instruction
    /// with none:
    ///
    /// - sign-extended where the encoding's immediate is signed;
    /// - the value written for `lui`, `auipc` and `c.lui`, with its low 12
    ///   bits clear;
    /// - the offset from the instruction for a branch or jump (see
    ///   [`target`](Self::target));
    /// - the byte offset of a load or store, a compressed encoding's scaled
    ///   field multiplied out;
    /// - the shift amount of a shift by an immediate;
    /// - the CSR number of a CSR instruction, whose `zimm` is in
    ///   [`rs1`](Self::rs1);
    /// - the stack adjustment in bytes of `cm.push` and `cm.pop*`, and the
    ///   table index of `cm.jt` and `cm.jalt`.
    pub fn imm(&self) -> i32 {
        self.imm
    }

    /// The second immediate, of the few vendor instructions that have two
    /// (`th.ext`, `cv.extract`, `cv.beqimm`). 0 for other instructions.
    pub fn imm2(&self) -> i32 {
        self.imm2
    }

    /// The rounding mode of a floating-point instruction that encodes one,
    /// or `None` for an instruction without a rounding-mode field.
    pub fn rounding_mode(&self) -> Option<RoundingMode> {
        matches!(self.decoded.codec, Codec::RM | Codec::R4M).then(|| match self.rm & 7 {
            0 => RoundingMode::Rne,
            1 => RoundingMode::Rtz,
            2 => RoundingMode::Rdn,
            3 => RoundingMode::Rup,
            4 => RoundingMode::Rmm,
            5 => RoundingMode::Reserved5,
            6 => RoundingMode::Reserved6,
            _ => RoundingMode::Dyn,
        })
    }

    /// Whether an atomic instruction has the acquire (`.aq`) ordering bit.
    pub fn aq(&self) -> bool {
        self.aq
    }

    /// Whether an atomic instruction has the release (`.rl`) ordering bit.
    pub fn rl(&self) -> bool {
        self.rl
    }

    /// The predecessor set of a `fence`: bit 3 device input, bit 2 device
    /// output, bit 1 memory reads and bit 0 memory writes (`iorw`). 0 for
    /// other instructions.
    pub fn pred(&self) -> u8 {
        self.pred
    }

    /// The successor set of a `fence`, laid out as [`pred`](Self::pred).
    pub fn succ(&self) -> u8 {
        self.succ
    }

    /// Whether a vector instruction is masked by `v0` (`v0.t`).
    pub fn is_masked(&self) -> bool {
        self.masked
    }

    /// Whether the floating-point operands are in the integer registers, as
    /// with Zfinx, Zdinx and Zhinx: [`rd`](Self::rd) and the sources then
    /// name `x` registers where they would otherwise name `f` registers.
    pub fn fp_in_x_registers(&self) -> bool {
        self.fp_in_x
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

    /// The address the instruction plus its [`imm`](Self::imm) names: the
    /// target of a branch or jump, and the value `auipc` writes. It is
    /// wrapped to the register width on RV32; addresses are 64 bits, so on
    /// RV128 it is the low 64 bits of the target.
    pub fn target(&self) -> u64 {
        let target = self.pc.wrapping_add(self.imm as i64 as u64);
        match self.isa {
            Isa::Rv32 => target & 0xffff_ffff,
            Isa::Rv64 | Isa::Rv128 => target,
        }
    }
}

/// The rounding mode of a floating-point instruction, from its `rm` field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    /// Round to nearest, ties to even (`rne`).
    Rne,
    /// Round towards zero (`rtz`).
    Rtz,
    /// Round down, towards negative infinity (`rdn`).
    Rdn,
    /// Round up, towards positive infinity (`rup`).
    Rup,
    /// Round to nearest, ties to max magnitude (`rmm`).
    Rmm,
    /// The reserved value 5.
    Reserved5,
    /// The reserved value 6.
    Reserved6,
    /// The dynamic rounding mode, from the `frm` CSR (`dyn`).
    Dyn,
}
