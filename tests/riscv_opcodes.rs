// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Checks the decoder against the official encodings in
//! [riscv-opcodes](https://github.com/riscv/riscv-opcodes), as extracted into
//! `tests/data/riscv-opcodes.txt` by `scripts/gen-riscv-opcodes.py`.
//!
//! The opcodes are compared without aliases, since riscv-opcodes names the
//! underlying instructions (and compressed instructions by their `c.` names).

pub mod common;

use common::{Entry, Failures, Rng, entries};
use disc_v::{Isa, decode};

/// Encodings riscv-opcodes lists for an ISA where the specification says they
/// do not exist, so disc-v decodes them as illegal.
const NOT_IN_SPEC: &[(Isa, &str)] = &[
    // The Zicfiss chapter of the ISA manual says ssamoswap.d is RV64-only, but
    // riscv-opcodes defines it in rv_zicfiss rather than rv64_zicfiss.
    (Isa::Rv32, "ssamoswap.d"),
    // The Zalasr chapter says ld.aq and sd.rl are RV64-only, but riscv-opcodes
    // defines them in rv_zalasr.
    (Isa::Rv32, "ld.aq"),
    (Isa::Rv32, "sd.rl"),
];

/// Whether the specification reserves `inst`, an encoding of `name`, in a
/// way riscv-opcodes does not express.
fn reserved_by_spec(isa: Isa, name: &str, inst: u32) -> bool {
    let (rd, rs2) = ((inst >> 7) & 0x1f, (inst >> 20) & 0x1f);
    match (isa, name) {
        // The register-pair forms of amocas reserve odd rd and rs2.
        (Isa::Rv32, "amocas.d") | (Isa::Rv64, "amocas.q") => rd & 1 == 1 || rs2 & 1 == 1,
        _ => false,
    }
}

/// The mnemonic of `inst` without aliases, or `None` if it is illegal.
fn decoded_name(isa: Isa, inst: u32) -> Option<&'static str> {
    let ins = decode(isa, 0, inst as u64).without_aliases();
    (!ins.is_illegal()).then(|| ins.mnemonic())
}

#[test]
fn every_riscv_opcodes_encoding_decodes_to_its_name() {
    let entries = entries();
    let mut rng = Rng::new(1);
    let mut failures = Failures::default();
    for isa in [Isa::Rv32, Isa::Rv64] {
        for e in entries
            .iter()
            .filter(|e| e.applies_to(isa) && !NOT_IN_SPEC.contains(&(isa, e.name)))
        {
            for inst in e.samples(&mut rng, 64) {
                if reserved_by_spec(isa, e.name, inst) {
                    continue;
                }
                match decoded_name(isa, inst) {
                    Some(name) if e.is_named(name) => {}
                    // Another entry for the same encoding, such as a more
                    // specific pseudo-op, may name it instead.
                    Some(name)
                        if entries
                            .iter()
                            .any(|o| o.applies_to(isa) && o.matches(inst) && o.is_named(name)) => {}
                    got => failures.add(
                        format_args!("{isa:?} {}: decoded as {got:?}", e.name),
                        format_args!("{inst:#010x}"),
                    ),
                }
            }
        }
    }
    failures.assert_none();
}

#[test]
fn every_decoded_encoding_is_in_riscv_opcodes() {
    let entries = entries();
    let mut rng = Rng::new(2);
    let mut failures = Failures::default();
    for isa in [Isa::Rv32, Isa::Rv64] {
        let entries: Vec<&Entry> = entries.iter().filter(|e| e.applies_to(isa)).collect();
        // Every compressed encoding, random 32-bit encodings, and each
        // encoding from riscv-opcodes with one bit flipped, which probes the
        // edges of the encoding space.
        let compressed = (0..=0xffffu32).filter(|x| x & 3 != 3);
        let random: Vec<u32> = (0..1_000_000).map(|_| rng.next_u32() | 3).collect();
        let mut flipped = vec![];
        for e in &entries {
            for inst in e.samples(&mut rng, 4) {
                let width = if inst & 3 == 3 { 32 } else { 16 };
                flipped.extend((0..width).map(|bit| inst ^ (1 << bit)));
            }
        }
        for inst in compressed.chain(random).chain(flipped) {
            let Some(name) = decoded_name(isa, inst) else {
                continue;
            };
            // riscv-opcodes leaves out HINT encodings, such as c.li with
            // rd=zero, so only the fixed bits of the encoding are compared.
            // Reserved encodings that must be illegal are tested in
            // tests/decode.rs.
            if !entries
                .iter()
                .any(|e| inst & e.mask == e.bits && e.is_named(name))
            {
                failures.add(
                    format_args!("{isa:?} {name}: not in riscv-opcodes"),
                    format_args!("{inst:#010x}"),
                );
            }
        }
    }
    failures.assert_none();
}
