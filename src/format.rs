// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! Text output for decoded instructions, in the style of GNU objdump.

use core::fmt::{self, Display, Write};

use crate::csr::csr_name;
use crate::instruction::Instruction;
use crate::opcodes::zfa::FLI_CONSTANTS;
use crate::reg::{FP_NAMES, INT_NAMES};

impl Display for Instruction {
    /// Formats the instruction as objdump does, with a single space between
    /// the mnemonic and the operands: `lw a0,8(sp)`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Some names include an ordering (lw.aq); it is shown with the other
        // ordering bit (lw.aqrl) from the encoding.
        let name = self.op.name;
        let name = match (self.aq || self.rl)
            .then(|| name.strip_suffix(".aq").or(name.strip_suffix(".rl")))
        {
            Some(Some(base)) => base,
            _ => name,
        };
        f.write_str(name)?;
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
        '(' | ',' | ' ' | ')' => out.write_char(c),
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
        'P' => write!(out, "{}", ins.imm & !0x1f),
        'D' => write!(out, "v{}", ins.rd),
        'A' => write!(out, "v{}", ins.rs1),
        'B' => write!(out, "v{}", ins.rs2),
        'M' if ins.masked => out.write_str(",v0.t"),
        'M' => Ok(()),
        'Z' => out.write_str("v0"),
        'T' => write_vtype(out, ins.imm as u32),
        'F' => out.write_str(FLI_CONSTANTS[ins.rs1 as usize]),
        'l' => write!(out, "{}", ins.len),
        'x' => {
            // As objdump does, padded to a whole number of 16-bit parcels.
            let parcels = (64 - ins.inst.leading_zeros()).div_ceil(16).max(1) as usize;
            write!(out, "0x{:01$x}", ins.inst, parcels * 4)
        }
        'p' => write_fence_set(out, ins.pred),
        's' => write_fence_set(out, ins.succ),
        _ => unreachable!("unknown format character {c:?}"),
    }
}

/// Writes a `vtype` immediate as objdump does, such as `e32,m1,ta,ma`, or as
/// a number if it has reserved values.
fn write_vtype(out: &mut fmt::Formatter<'_>, vtype: u32) -> fmt::Result {
    let (sew, lmul) = ((vtype >> 3) & 7, vtype & 7);
    let lmul = ["m1", "m2", "m4", "m8", "", "mf8", "mf4", "mf2"][lmul as usize];
    if vtype >> 8 != 0 || sew > 3 || lmul.is_empty() {
        return write!(out, "{vtype}");
    }
    let tail = if vtype & 0x40 != 0 { "ta" } else { "tu" };
    let mask = if vtype & 0x80 != 0 { "ma" } else { "mu" };
    write!(out, "e{},{lmul},{tail},{mask}", 8 << sew)
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
