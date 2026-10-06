// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Compares disc-v's output with GNU objdump.
//!
//! The encodings tested are every compressed encoding, random 32-bit
//! encodings, and random encodings of every instruction in riscv-opcodes.
//! They are assembled with `.insn` directives (so the assembler does no
//! checking), disassembled with `objdump -d`, and compared after putting
//! objdump's text in disc-v's format (see `normalize`).
//!
//! The test is skipped if no RISC-V binutils are found. It looks for
//! `riscv64-elf-`, `riscv64-unknown-elf-` and `riscv64-linux-gnu-` prefixed
//! tools, or the prefix in `DISC_V_BINUTILS_PREFIX`. Set
//! `DISC_V_REQUIRE_BINUTILS=1` to fail instead of skipping.

pub mod common;

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Command;

use common::{Failures, Rng, riscv_opcodes_samples};
use disc_v::{Isa, decode};

const MARCH_EXTENSIONS: &str = "imafdqc_zicsr_zifencei_zba_zbb_zbc_zbs_zicond_zawrs_zicbom_zicboz_zicbop_zihintntl_zihintpause_zimop_zcmop_zicfiss_zicfilp";

fn binutils_prefix() -> Option<String> {
    let candidates = match std::env::var("DISC_V_BINUTILS_PREFIX") {
        Ok(prefix) => vec![prefix],
        Err(_) => ["riscv64-elf-", "riscv64-unknown-elf-", "riscv64-linux-gnu-"]
            .map(String::from)
            .to_vec(),
    };
    let found = candidates.into_iter().find(|p| {
        Command::new(format!("{p}objdump"))
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success())
    });
    if found.is_none() {
        assert!(
            std::env::var_os("DISC_V_REQUIRE_BINUTILS").is_none(),
            "RISC-V binutils not found"
        );
        eprintln!("skipping: RISC-V binutils not found");
    }
    found
}

/// Assembles `insts` at consecutive addresses from 0 and returns objdump's
/// text for each one.
fn objdump(prefix: &str, isa: Isa, insts: &[u32]) -> Vec<String> {
    let xlen = match isa {
        Isa::Rv32 => 32,
        _ => 64,
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("objdump-rv{xlen}"));
    std::fs::create_dir_all(&dir).unwrap();
    let mut asm = String::new();
    for &inst in insts {
        let len = if inst & 3 == 3 { 4 } else { 2 };
        writeln!(asm, ".insn {len}, {inst:#x}").unwrap();
    }
    std::fs::write(dir.join("t.s"), asm).unwrap();
    let run = |cmd: &mut Command| {
        let out = cmd.current_dir(&dir).output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    };
    run(Command::new(format!("{prefix}as"))
        .arg(format!("-march=rv{xlen}{MARCH_EXTENSIONS}"))
        .args(["t.s", "-o", "t.o"]));
    let dump = run(Command::new(format!("{prefix}objdump")).args(["-d", "t.o"]));

    // Lines look like "   1c:\t00b50463          \tbeq\ta0,a1,0x24".
    let mut by_addr = std::collections::HashMap::new();
    for line in dump.lines() {
        let mut parts = line.splitn(3, '\t');
        let (Some(addr), Some(_hex), Some(text)) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        let Some(addr) = addr.trim().strip_suffix(':') else {
            continue;
        };
        let Ok(addr) = u64::from_str_radix(addr, 16) else {
            continue;
        };
        by_addr.insert(addr, normalize(text));
    }
    let mut pc = 0;
    insts
        .iter()
        .map(|&inst| {
            let text = by_addr
                .remove(&pc)
                .unwrap_or_else(|| panic!("no objdump output at {pc:#x}"));
            pc += if inst & 3 == 3 { 4 } else { 2 };
            text
        })
        .collect()
}

/// Puts objdump's text in disc-v's format: a space after the mnemonic, no
/// comments, and `0x`-prefixed branch targets without symbols.
fn normalize(text: &str) -> String {
    let text = text.split(" # ").next().unwrap().trim_end();
    let mut text = text.replacen('\t', " ", 1);
    // Older binutils (such as 2.42) name the shift-by-zero HINTs c.slli64,
    // c.srli64 and c.srai64.
    for name in ["c.slli", "c.srli", "c.srai"] {
        if let Some(reg) = text.strip_prefix(&format!("{name}64 ")) {
            text = format!("{name} {reg},0x0");
        }
    }
    // "beq a0,a1,1c <.text+0x1c>" -> "beq a0,a1,0x1c"
    if let Some(i) = text.find(" <") {
        text.truncate(i);
        let start = text.rfind([' ', ',']).unwrap() + 1;
        text.insert_str(start, "0x");
    }
    text
}

/// The mnemonic and CSR operand of a CSR instruction.
fn csr_operand(text: &str) -> Option<(&str, &str)> {
    let (mnemonic, operands) = text.split_once(' ')?;
    let operands: Vec<&str> = operands.split(',').collect();
    match mnemonic {
        "csrrw" | "csrrs" | "csrrc" | "csrrwi" | "csrrsi" | "csrrci" | "csrr" => {
            Some((mnemonic, operands[1]))
        }
        "csrw" | "csrs" | "csrc" | "csrwi" | "csrsi" | "csrci" => Some((mnemonic, operands[0])),
        _ => None,
    }
}

/// Why disc-v's output may differ from objdump's for `inst`, or `None` if
/// it should not. These are encodings where binutils disagrees with the ISA
/// specification about what is valid, where disc-v follows the
/// specification (as encoded in riscv-opcodes), and CSRs that binutils has
/// no name for.
fn known_difference(isa: Isa, inst: u32, objdump: &str, disc_v: &str) -> Option<&'static str> {
    let mnemonic = |s: &str| s.split([' ', '.']).next().unwrap().to_string();
    if disc_v.starts_with(".insn ") {
        // A shift amount of 32 or more is reserved on RV32.
        let shift = [
            "slli", "srli", "srai", "bclri", "bseti", "binvi", "bexti", "rori", "c.slli", "c.srli",
            "c.srai",
        ];
        if isa == Isa::Rv32 && shift.contains(&objdump.split(' ').next().unwrap()) {
            return Some("RV32 shift amount >= 32");
        }
        // c.addi16sp with a zero immediate is reserved.
        if inst & 0xef83 == 0x6101 {
            return Some("c.addi16sp 0");
        }
    }
    if objdump.starts_with(".insn ") {
        // Base implementations must ignore the reserved fields of fence and
        // fence.i (and treat reserved fence modes as normal fences).
        if mnemonic(disc_v) == "fence" {
            return Some("fence with nonzero reserved fields");
        }
        // binutils only accepts RNE for conversions that are always exact.
        if disc_v.starts_with("fcvt.") && disc_v.contains(',') {
            return Some("exact fcvt with a rounding mode other than rne");
        }
    }
    // riscv-opcodes names some CSRs that binutils does not.
    if let Some((_, csr)) = csr_operand(objdump) {
        if csr.starts_with("0x") && csr_operand(disc_v).is_some_and(|(_, c)| !c.starts_with("0x")) {
            return Some("CSR named by riscv-opcodes but not binutils");
        }
    }
    // c.addi rd,0 is a HINT that objdump shows as addi rd,rd,0.
    if inst & 0xf07f == 0x0001 && objdump.starts_with("addi ") && disc_v.starts_with("mv ") {
        return Some("c.addi rd,0 HINT");
    }
    None
}

fn test_encodings(rng: &mut Rng) -> Vec<u32> {
    let mut insts: Vec<u32> = (0..=0xffffu32).filter(|x| x & 3 != 3).collect();
    // Random 32-bit encodings (the low bits mark the instruction length).
    insts.extend(
        (0..200_000)
            .map(|_| rng.next_u32() | 3)
            .filter(|&x| disc_v::inst_length(x as u16) == Some(4)),
    );
    insts.extend(riscv_opcodes_samples(rng, 64));
    insts
}

#[test]
fn matches_objdump() {
    let Some(prefix) = binutils_prefix() else {
        return;
    };
    let mut rng = Rng::new(3);
    let insts = test_encodings(&mut rng);
    let mut failures = Failures::default();
    for isa in [Isa::Rv32, Isa::Rv64] {
        let expected = objdump(&prefix, isa, &insts);
        let mut pc = 0;
        for (&inst, want) in insts.iter().zip(&expected) {
            let got = decode(isa, pc, inst as u64).to_string();
            if got != *want && known_difference(isa, inst, want, &got).is_none() {
                let mnemonic = |s: &str| s.split(' ').next().unwrap().to_string();
                failures.add(
                    format_args!(
                        "{isa:?} objdump {} vs disc-v {}",
                        mnemonic(want),
                        mnemonic(&got)
                    ),
                    format_args!("{inst:#010x} at {pc:#x}: objdump `{want}`, disc-v `{got}`"),
                );
            }
            pc += if inst & 3 == 3 { 4 } else { 2 };
        }
    }
    failures.assert_none();
}
