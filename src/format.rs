// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Text output for decoded instructions, in the style of GNU objdump.

use core::fmt::{self, Display, Write};

use crate::csr::csr_name;
use crate::instruction::Instruction;
use crate::reg::{FP_NAMES, INT_NAMES};

impl Display for Instruction {
    /// Formats the instruction as objdump does, with a single space between
    /// the mnemonic and the operands: `lw a0,8(sp)`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.op.name)?;
        match (self.aq, self.rl) {
            (true, true) => f.write_str(".aqrl")?,
            (true, false) => f.write_str(".aq")?,
            (false, true) => f.write_str(".rl")?,
            (false, false) => {}
        }
        if !self.op.format.is_empty() {
            write!(f, " {}", self.operands())?;
        }
        Ok(())
    }
}

/// The operands of an instruction, formatted as objdump does: `a0,8(sp)`.
///
/// Create one with [`Instruction::operands`].
#[derive(Clone, Copy, Debug)]
pub struct Operands<'a>(pub(crate) &'a Instruction);

impl Display for Operands<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ins = self.0;
        for c in ins.op.format.chars() {
            write_part(f, ins, c)?;
        }
        Ok(())
    }
}

/// Writes the output for format character `c`; see
/// [`fmt`](crate::opcodes::fmt).
fn write_part(out: &mut fmt::Formatter<'_>, ins: &Instruction, c: char) -> fmt::Result {
    match c {
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
        '>' => write!(out, "{:#x}", ins.imm),
        'u' => write!(out, "{:#x}", (ins.imm as u32 >> 12) & 0xfffff),
        'o' => write!(out, "{:#x}", ins.target()),
        'c' => match csr_name(ins.imm as u16 & 0xfff) {
            Some(name) => out.write_str(name),
            None => write!(out, "{:#x}", ins.imm & 0xfff),
        },
        'r' if ins.rm == 7 => Ok(()),
        'R' if ins.rm == 0 => Ok(()),
        'r' | 'R' => {
            let names = [
                "rne", "rtz", "rdn", "rup", "rmm", "unknown", "unknown", "dyn",
            ];
            write!(out, ",{}", names[ins.rm as usize])
        }
        'p' => write_fence_set(out, ins.pred),
        's' => write_fence_set(out, ins.succ),
        _ => unreachable!("unknown format character {c:?}"),
    }
}

/// Writes the device input, device output, memory read and memory write bits
/// of a fence predecessor or successor set.
fn write_fence_set(out: &mut fmt::Formatter<'_>, set: u8) -> fmt::Result {
    if set == 0 {
        return out.write_str("unknown");
    }
    for (bit, name) in [(8, 'i'), (4, 'o'), (2, 'r'), (1, 'w')] {
        if set & bit != 0 {
            out.write_char(name)?;
        }
    }
    Ok(())
}
