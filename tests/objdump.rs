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
//! The `-march` comes from disc-v's default extensions, less any the
//! assembler does not accept, and disc-v decodes with the same extensions,
//! so the test also runs with older binutils (it prints the extensions it
//! skips).
//!
//! The test is skipped if no RISC-V binutils are found. It looks for
//! `riscv64-elf-`, `riscv64-unknown-elf-` and `riscv64-linux-gnu-` prefixed
//! tools, or the prefix in `DISC_V_BINUTILS_PREFIX`. Set
//! `DISC_V_REQUIRE_BINUTILS=1` to fail instead of skipping.

pub mod common;

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::Command;

use common::{Failures, Rng, embedded, riscv_opcodes_samples, rve};
use disc_v::{Decoder, Extension, Extensions, Isa};

/// Extensions whose instructions binutils decodes even when they are not in
/// `-march` (and which it may not accept there).
const ALWAYS_DECODED_BY_BINUTILS: &[Extension] = &[Extension::Sdext];

/// The extensions in `config` that the assembler accepts in `-march` for
/// `isa`. disc-v is compared with the same set, so the test works with older
/// binutils.
fn supported(prefix: &str, isa: Isa, config: Extensions) -> Extensions {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("objdump-probe");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("probe.s"), "nop\n").unwrap();
    // Each extension is tried with the extensions it implies.
    let accepts = |ext: Extension| {
        Command::new(format!("{prefix}as"))
            .current_dir(&dir)
            .arg(format!("-march={}", march(isa, ext.into())))
            .args(["probe.s", "-o", "probe.o"])
            .output()
            .is_ok_and(|o| o.status.success())
    };
    let mut exts = config;
    for &ext in ALWAYS_DECODED_BY_BINUTILS {
        exts = exts.with(ext);
    }
    for ext in config.iter() {
        if !ALWAYS_DECODED_BY_BINUTILS.contains(&ext) && !accepts(ext) {
            eprintln!("binutils does not support {ext} for {isa:?}; not testing it");
            exts = exts.without(ext);
        }
    }
    exts
}

/// The `-march` string for `isa` with `exts`.
fn march(isa: Isa, exts: Extensions) -> String {
    let xlen = if isa == Isa::Rv32 { 32 } else { 64 };
    let base = if exts.contains(Extension::E) {
        "e"
    } else {
        "i"
    };
    let mut march = format!("rv{xlen}{base}");
    // Single-letter extensions come first, in canonical order.
    for letter in ["m", "a", "f", "d", "q", "c", "v", "h"] {
        // (E is the base.)
        if exts.iter().any(|e| e.name() == letter) {
            march += letter;
        }
    }
    for ext in exts.iter().filter(|e| e.name().len() > 1) {
        if !ALWAYS_DECODED_BY_BINUTILS.contains(&ext) {
            march += "_";
            march += ext.name();
        }
    }
    march
}

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
fn objdump(prefix: &str, isa: Isa, exts: Extensions, insts: &[u32]) -> Vec<String> {
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
        .arg(format!("-march={}", march(isa, exts)))
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
fn known_difference(
    isa: Isa,
    exts: Extensions,
    inst: u32,
    objdump: &str,
    disc_v: &str,
) -> Option<&'static str> {
    // RV32E and RV64E reserve x16-x31, which objdump decodes regardless.
    const UPPER: [&str; 16] = [
        "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5",
        "t6",
    ];
    if exts.contains(Extension::E)
        && disc_v.starts_with(".insn ")
        && objdump
            .split(|c: char| !c.is_ascii_alphanumeric())
            .any(|token| UPPER.contains(&token))
    {
        return Some("RV32E/RV64E register x16-x31");
    }
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
        // Zcmp reserves register lists below 4, which objdump decodes.
        if objdump.starts_with("cm.push ") || objdump.starts_with("cm.pop") {
            return Some("cm.push/cm.pop with a reserved register list");
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
    // The defaults, and a configuration with Zcmp and Zcmt, which conflict
    // with the defaults.
    let configs = [
        ("default", Extensions::DEFAULT),
        ("embedded", embedded()),
        ("rve", rve()),
    ];
    for (isa, (config, exts)) in [Isa::Rv32, Isa::Rv64]
        .into_iter()
        .flat_map(|i| configs.map(|c| (i, c)))
    {
        let exts = supported(&prefix, isa, exts);
        let dec = Decoder::with_only(isa, exts).unwrap();
        let expected = objdump(&prefix, isa, exts, &insts);
        let mut pc = 0;
        for (&inst, want) in insts.iter().zip(&expected) {
            let got = dec.decode(pc, inst as u64).to_string();
            if got != *want && known_difference(isa, exts, inst, want, &got).is_none() {
                let mnemonic = |s: &str| s.split(' ').next().unwrap().to_string();
                failures.add(
                    format_args!(
                        "{isa:?} {config}: objdump {} vs disc-v {}",
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
