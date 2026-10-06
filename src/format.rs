//! Text output for decoded instructions.

use core::fmt::{self, Write};

use crate::csr::csr_name;
use crate::instruction::Instruction;
use crate::reg::{FP_NAMES, INT_NAMES};

/// Column at which the operands start in a [`Listing`]; offset comments start
/// at twice this.
const TAB_SIZE: usize = 32;

/// An instruction formatted as a listing line: the encoding in hex, the
/// instruction, and the target address of PC-relative operands.
///
/// ```text
/// 10018193          addi          gp,gp,256
/// ```
///
/// This is the output format of the Caliptra emulator's disassembler.
/// Create one with [`Instruction::listing`].
pub struct Listing<'a>(&'a Instruction);

impl Instruction {
    /// Formats the instruction as a [`Listing`] line.
    pub fn listing(&self) -> Listing<'_> {
        Listing(self)
    }
}

impl fmt::Display for Listing<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ins = self.0;
        let mut out = Columns { f, col: 0 };
        match ins.length() {
            2 => write!(out, "{:04x}              ", ins.inst & 0xffff)?,
            4 => write!(out, "{:08x}          ", ins.inst & 0xffff_ffff)?,
            6 => write!(out, "{:012x}      ", ins.inst & 0xffff_ffff_ffff)?,
            _ => write!(out, "{:016x}  ", ins.inst)?,
        }
        for c in ins.op.format.chars() {
            match c {
                '\t' => out.pad_to(TAB_SIZE)?,
                'o' => {
                    write!(out, "{}", ins.imm)?;
                    out.pad_to(TAB_SIZE * 2)?;
                    let target = (ins.pc as i64).wrapping_add(ins.imm as i64);
                    write!(out, "# 0x{target:x}")?;
                }
                c => write_part(&mut out, ins, c)?,
            }
        }
        Ok(())
    }
}

/// Writes the output for format character `c` other than `\t` and `o`; see
/// [`fmt`](crate::opcodes::fmt).
fn write_part(out: &mut impl Write, ins: &Instruction, c: char) -> fmt::Result {
    match c {
        'O' => out.write_str(ins.op.name),
        '(' | ',' | ')' => out.write_char(c),
        '0' => out.write_str(INT_NAMES[ins.rd as usize]),
        '1' => out.write_str(INT_NAMES[ins.rs1 as usize]),
        '2' => out.write_str(INT_NAMES[ins.rs2 as usize]),
        '3' => out.write_str(FP_NAMES[ins.rd as usize]),
        '4' => out.write_str(FP_NAMES[ins.rs1 as usize]),
        '5' => out.write_str(FP_NAMES[ins.rs2 as usize]),
        '6' => out.write_str(FP_NAMES[ins.rs3 as usize]),
        '7' => write!(out, "{}", ins.rs1),
        'i' => write!(out, "{}", ins.imm),
        'c' => match csr_name(ins.imm & 0xfff) {
            "" => write!(out, "0x{:03x}", ins.imm & 0xfff),
            name => out.write_str(name),
        },
        'r' => out.write_str(match ins.rm {
            0 => "rne",
            1 => "rtz",
            2 => "rdn",
            3 => "rup",
            4 => "rmm",
            7 => "dyn",
            _ => "inv",
        }),
        'p' => write_fence_set(out, ins.pred),
        's' => write_fence_set(out, ins.succ),
        'A' if ins.aq != 0 => out.write_str(".aq"),
        'R' if ins.rl != 0 => out.write_str(".rl"),
        _ => Ok(()),
    }
}

/// Writes the device input, device output, memory read and memory write bits
/// of a fence predecessor or successor set.
fn write_fence_set(out: &mut impl Write, set: u8) -> fmt::Result {
    for (bit, name) in [(8, 'i'), (4, 'o'), (2, 'r'), (1, 'w')] {
        if set & bit != 0 {
            out.write_char(name)?;
        }
    }
    Ok(())
}

/// A writer that tracks the current column so output can be aligned.
struct Columns<'a, 'b> {
    f: &'a mut fmt::Formatter<'b>,
    col: usize,
}

impl Columns<'_, '_> {
    fn pad_to(&mut self, col: usize) -> fmt::Result {
        while self.col < col {
            self.write_char(' ')?;
        }
        Ok(())
    }
}

impl Write for Columns<'_, '_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.col += s.len();
        self.f.write_str(s)
    }
}
