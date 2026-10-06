// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Opcode tables: one static [`Opcode`] per instruction, grouped by extension.

pub(crate) mod a;
pub(crate) mod b;
pub(crate) mod c;
pub(crate) mod d;
pub(crate) mod f;
pub(crate) mod i;
pub(crate) mod m;
pub(crate) mod pseudo;
pub(crate) mod q;
pub(crate) mod system;
pub(crate) mod zicond;

use crate::Isa;

/// How an instruction's operands are laid out in its encoding.
///
/// The names follow the instruction formats in the RISC-V specification
/// (R, I, S, SB, U, UJ, and the compressed CR, CI, CSS, CIW, CL, CS, CB, CJ),
/// with a suffix where a format has several immediate layouts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Codec {
    Li,
    CssSqsp,
    CssSdsp,
    CssSwsp,
    CsSq,
    CsSd,
    CsSw,
    Cs,
    CrJr,
    CrJalr,
    CrMv,
    Cr,
    ClLq,
    ClLd,
    ClLw,
    CjJal,
    Cj,
    Ciw4spn,
    CiNone,
    CiLui,
    CiLi,
    CiLqsp,
    CiLdsp,
    CiLwsp,
    Ci16sp,
    CiSh6,
    Ci,
    CbSh6,
    CbImm,
    Cb,
    RF,
    RL,
    RA,
    R4M,
    RM,
    R,
    SB,
    S,
    ICsr,
    ISh7,
    ISh6,
    ISh5,
    I,
    Uj,
    U,
    None,
    Illegal,
}

/// Operand format strings.
///
/// Each character expands to one piece of the operand text:
///
/// | Char | Output |
/// |------|--------|
/// | `0` `1` `2` | integer register `rd`, `rs1`, `rs2` |
/// | `3` `4` `5` `6` | floating-point register `rd`, `rs1`, `rs2`, `rs3` |
/// | `7` | `rs1` field as an unsigned immediate (CSR `zimm`) |
/// | `i` | immediate, in decimal |
/// | `>` | shift amount, in hex |
/// | `u` | upper immediate (bits 31:12), in hex |
/// | `o` | target address of a PC-relative offset |
/// | `c` | CSR name |
/// | `r` | `,` and the rounding mode, unless it is dynamic |
/// | `R` | `,` and the rounding mode, unless it is RNE (for exact conversions) |
/// | `p` `s` | fence predecessor and successor sets |
/// | `l` | length of the encoding in bytes |
/// | `x` | the encoding, in hex, padded to a whole number of 16-bit parcels |
/// | `,` ` ` `(` `)` | themselves |
pub(crate) mod fmt {
    pub(crate) const NONE: &str = "";
    pub(crate) const RS1: &str = "1";
    pub(crate) const OFFSET: &str = "o";
    pub(crate) const PRED_SUCC: &str = "p,s";
    pub(crate) const RS1_RS2: &str = "1,2";
    pub(crate) const RD_IMM: &str = "0,i";
    pub(crate) const RD_UIMM: &str = "0,u";
    pub(crate) const RD_OFFSET: &str = "0,o";
    pub(crate) const RD_RS1_RS2: &str = "0,1,2";
    pub(crate) const FRD_RS1: &str = "3,1";
    pub(crate) const FRD_FRS1: &str = "3,4";
    pub(crate) const RD_FRS1: &str = "0,4";
    pub(crate) const RD_FRS1_FRS2: &str = "0,4,5";
    pub(crate) const FRD_FRS1_FRS2: &str = "3,4,5";
    pub(crate) const RM_FRD_FRS1: &str = "3,4r";
    pub(crate) const RM_FRD_RS1: &str = "3,1r";
    pub(crate) const RM_RD_FRS1: &str = "0,4r";
    pub(crate) const RM_FRD_FRS1_FRS2: &str = "3,4,5r";
    pub(crate) const RM_FRD_FRS1_FRS2_FRS3: &str = "3,4,5,6r";
    pub(crate) const RD_RS1_IMM: &str = "0,1,i";
    pub(crate) const RD_RS1_SHAMT: &str = "0,1,>";
    pub(crate) const RD_OFFSET_RS1: &str = "0,i(1)";
    pub(crate) const FRD_OFFSET_RS1: &str = "3,i(1)";
    pub(crate) const RD_CSR_RS1: &str = "0,c,1";
    pub(crate) const RD_CSR_ZIMM: &str = "0,c,7";
    pub(crate) const RS2_OFFSET_RS1: &str = "2,i(1)";
    pub(crate) const FRS2_OFFSET_RS1: &str = "5,i(1)";
    pub(crate) const RS1_RS2_OFFSET: &str = "1,2,o";
    pub(crate) const RD_RS2_ADDR_RS1: &str = "0,2,(1)";
    pub(crate) const RD_ADDR_RS1: &str = "0,(1)";
    pub(crate) const RD: &str = "0";
    pub(crate) const RD_ZIMM: &str = "0,7";
    pub(crate) const RD_RS1: &str = "0,1";
    pub(crate) const RD_RS2: &str = "0,2";
    pub(crate) const RS1_OFFSET: &str = "1,o";
    pub(crate) const RS2_OFFSET: &str = "2,o";
    pub(crate) const OFFSET_RS1: &str = "i(1)";
    pub(crate) const RD_SHAMT: &str = "0,>";
    pub(crate) const RD_CSR: &str = "0,c";
    pub(crate) const CSR_RS1: &str = "c,1";
    pub(crate) const CSR_ZIMM: &str = "c,7";
    pub(crate) const IMM: &str = "i";
    pub(crate) const WIDEN_FRD_FRS1: &str = "3,4R";
    pub(crate) const WIDEN_FRD_RS1: &str = "3,1R";
    pub(crate) const INSN: &str = "l, x";
    pub(crate) const RAW: &str = "x";
}

/// A condition on decoded operands, used to pick a pseudoinstruction.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Constraint {
    RdEq(u8),
    Rs1Eq(u8),
    Rs2Eq(u8),
    Rs2EqRs1,
    ImmEq(i32),
    /// The CSR number, which is held in the immediate.
    CsrEq(i32),
}

/// An alternative way to show an instruction when all of `when` hold.
pub(crate) struct Pseudo {
    pub(crate) op: &'static Opcode,
    pub(crate) when: &'static [Constraint],
}

impl Pseudo {
    pub(crate) const fn new(op: &'static Opcode, when: &'static [Constraint]) -> Self {
        Pseudo { op, when }
    }
}

/// One entry in the opcode tables.
pub(crate) struct Opcode {
    pub(crate) name: &'static str,
    pub(crate) codec: Codec,
    /// Operand format string; see [`fmt`].
    pub(crate) format: &'static str,
    /// Pseudoinstructions to try, in order; the first match is shown instead.
    pub(crate) pseudo: &'static [Pseudo],
    /// For compressed instructions, the expansion for RV32, RV64 and RV128;
    /// `None` means the encoding is not valid for that ISA.
    pub(crate) decompress: [Option<&'static Opcode>; 3],
    /// Operand values that make the encoding reserved: it is illegal if any
    /// of these hold.
    pub(crate) illegal_if: &'static [Constraint],
    /// Operand values that make the encoding a HINT, which objdump shows as
    /// encoded rather than expanded: it is a HINT if any of these hold.
    pub(crate) hint_if: &'static [Constraint],
    /// The base ISAs the instruction exists in, as a bit set indexed by
    /// [`Isa`].
    pub(crate) isas: u8,
}

impl Opcode {
    pub(crate) const fn new(name: &'static str, codec: Codec, format: &'static str) -> Self {
        Opcode {
            name,
            codec,
            format,
            pseudo: &[],
            decompress: [None; 3],
            illegal_if: &[],
            hint_if: &[],
            isas: 0b111,
        }
    }

    /// Marks an instruction as existing only in RV64 and RV128.
    pub(crate) const fn rv64(self) -> Self {
        Opcode {
            isas: 0b110,
            ..self
        }
    }

    /// Marks an instruction as existing only in RV128.
    pub(crate) const fn rv128(self) -> Self {
        Opcode {
            isas: 0b100,
            ..self
        }
    }

    pub(crate) fn exists_in(&self, isa: Isa) -> bool {
        match self.decompress {
            [None, None, None] => self.isas & (1 << isa as u8) != 0,
            expansions => expansions[isa as usize].is_some_and(|e| e.exists_in(isa)),
        }
    }
}

// Opcodes are compared by address, and `Debug` shows only the mnemonic
// rather than the alias and expansion tables.
impl PartialEq for Opcode {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self, other)
    }
}

impl Eq for Opcode {}

impl core::fmt::Debug for Opcode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name)
    }
}

/// An encoding that is not a valid instruction, shown as objdump does.
pub(crate) static ILLEGAL: Opcode = Opcode::new(".insn", Codec::Illegal, fmt::INSN);
/// A 16-bit parcel of an encoding with a reserved length (10 bytes or more),
/// which `.insn` cannot express.
pub(crate) static RESERVED_PARCEL: Opcode = Opcode::new(".2byte", Codec::Illegal, fmt::RAW);
