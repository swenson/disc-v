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
/// Each character expands to one piece of the output:
///
/// | Char | Output |
/// |------|--------|
/// | `O` | mnemonic |
/// | `\t` | separator between the mnemonic and the operands |
/// | `0` `1` `2` | integer register `rd`, `rs1`, `rs2` |
/// | `3` `4` `5` `6` | floating-point register `rd`, `rs1`, `rs2`, `rs3` |
/// | `7` | `rs1` field as an unsigned immediate (CSR `zimm`) |
/// | `i` | immediate |
/// | `o` | PC-relative offset |
/// | `c` | CSR name |
/// | `r` | rounding mode |
/// | `p` `s` | fence predecessor and successor sets |
/// | `A` `R` | `.aq` and `.rl` suffixes, when set |
/// | `,` `(` `)` | themselves |
pub(crate) mod fmt {
    pub(crate) const NONE: &str = "O";
    pub(crate) const RS1: &str = "O\t1";
    pub(crate) const OFFSET: &str = "O\to";
    pub(crate) const PRED_SUCC: &str = "O\tp,s";
    pub(crate) const RS1_RS2: &str = "O\t1,2";
    pub(crate) const RD_IMM: &str = "O\t0,i";
    pub(crate) const RD_OFFSET: &str = "O\t0,o";
    pub(crate) const RD_RS1_RS2: &str = "O\t0,1,2";
    pub(crate) const FRD_RS1: &str = "O\t3,1";
    pub(crate) const RD_FRS1: &str = "O\t0,4";
    pub(crate) const RD_FRS1_FRS2: &str = "O\t0,4,5";
    pub(crate) const FRD_FRS1_FRS2: &str = "O\t3,4,5";
    pub(crate) const RM_FRD_FRS1: &str = "O\tr,3,4";
    pub(crate) const RM_FRD_RS1: &str = "O\tr,3,1";
    pub(crate) const RM_RD_FRS1: &str = "O\tr,0,4";
    pub(crate) const RM_FRD_FRS1_FRS2: &str = "O\tr,3,4,5";
    pub(crate) const RM_FRD_FRS1_FRS2_FRS3: &str = "O\tr,3,4,5,6";
    pub(crate) const RD_RS1_IMM: &str = "O\t0,1,i";
    pub(crate) const RD_RS1_OFFSET: &str = "O\t0,1,i";
    pub(crate) const RD_OFFSET_RS1: &str = "O\t0,i(1)";
    pub(crate) const FRD_OFFSET_RS1: &str = "O\t3,i(1)";
    pub(crate) const RD_CSR_RS1: &str = "O\t0,c,1";
    pub(crate) const RD_CSR_ZIMM: &str = "O\t0,c,7";
    pub(crate) const RS2_OFFSET_RS1: &str = "O\t2,i(1)";
    pub(crate) const FRS2_OFFSET_RS1: &str = "O\t5,i(1)";
    pub(crate) const RS1_RS2_OFFSET: &str = "O\t1,2,o";
    pub(crate) const RS2_RS1_OFFSET: &str = "O\t2,1,o";
    pub(crate) const AQRL_RD_RS2_RS1: &str = "OAR\t0,2,(1)";
    pub(crate) const AQRL_RD_RS1: &str = "OAR\t0,(1)";
    pub(crate) const RD: &str = "O\t0";
    pub(crate) const RD_ZIMM: &str = "O\t0,7";
    pub(crate) const RD_RS1: &str = "O\t0,1";
    pub(crate) const RD_RS2: &str = "O\t0,2";
    pub(crate) const RS1_OFFSET: &str = "O\t1,o";
    pub(crate) const RS2_OFFSET: &str = "O\t2,o";
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

/// One entry in the opcode tables.
pub(crate) struct Opcode {
    pub(crate) name: &'static str,
    pub(crate) codec: Codec,
    /// Operand format string; see [`fmt`].
    pub(crate) format: &'static str,
    /// Pseudoinstructions to try, in order; the first match is shown instead.
    pub(crate) pseudo: &'static [Pseudo],
    /// For compressed instructions, the expansion for RV32, RV64 and RV128.
    pub(crate) decompress: [Option<&'static Opcode>; 3],
    /// The encoding is reserved (illegal) when its immediate is zero.
    pub(crate) check_imm_nz: bool,
}

impl Opcode {
    pub(crate) const fn new(name: &'static str, codec: Codec, format: &'static str) -> Self {
        Opcode {
            name,
            codec,
            format,
            pseudo: &[],
            decompress: [None; 3],
            check_imm_nz: false,
        }
    }
}

// Opcodes refer to each other (an instruction can be its own pseudo), so
// identity is by address and `Debug` shows only the mnemonic.
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

pub(crate) static ILLEGAL: Opcode = Opcode::new("illegal", Codec::Illegal, fmt::NONE);
