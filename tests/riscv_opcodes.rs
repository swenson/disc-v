// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Checks the decoder against the official encodings in
//! [riscv-opcodes](https://github.com/riscv/riscv-opcodes), as extracted into
//! `tests/data/riscv-opcodes.txt` by `scripts/gen-riscv-opcodes.py`.
//!
//! The opcodes are compared without aliases, since riscv-opcodes names the
//! underlying instructions (and compressed instructions by their `c.` names).

pub mod common;

use common::{Entry, Failures, Rng, embedded, entries, inx, rve, with_resolving_conflicts};
use disc_v::{Decoder, Extension, Extensions, Isa};

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
        // Zcmp reserves register lists below 4, and cm.mvsa01 into the same
        // s-register twice.
        (_, "cm.push" | "cm.pop" | "cm.popret" | "cm.popretz") => (inst >> 4) & 0xf < 4,
        (_, "cm.mvsa01") => (inst >> 7) & 7 == (inst >> 2) & 7,
        _ => false,
    }
}

/// The riscv-opcodes name for disc-v's mnemonic `name`: riscv-opcodes has
/// only cm.jalt, which the specification calls cm.jt for indices below 32.
fn riscv_opcodes_name(name: &'static str) -> &'static str {
    match name {
        "cm.jt" => "cm.jalt",
        name => name,
    }
}

/// The mnemonic of `inst` without aliases, or `None` if it is illegal.
fn decoded_name(dec: Decoder, inst: u32) -> Option<&'static str> {
    let ins = dec.decode(0, inst as u64).without_aliases();
    (!ins.is_illegal()).then(|| riscv_opcodes_name(ins.mnemonic()))
}

/// The riscv-opcodes entries, with its errors corrected: rv32_zilsd names
/// its store pseudo-op `ld`.
fn entries_fixed() -> Vec<Entry> {
    let mut entries = entries();
    for e in &mut entries {
        if e.name == "ld" && e.bits & 0x7f == 0x23 {
            e.name = "sd";
            e.base = Some("sd");
        }
    }
    entries
}

#[test]
fn every_riscv_opcodes_encoding_decodes_to_its_name() {
    let entries = entries_fixed();
    let mut rng = Rng::new(1);
    let mut failures = Failures::default();
    for isa in [Isa::Rv32, Isa::Rv64] {
        for e in entries
            .iter()
            .filter(|e| e.applies_to(isa) && !NOT_IN_SPEC.contains(&(isa, e.name)))
        {
            // A decoder with the extensions the entry needs, including any
            // that conflict with the defaults (such as Zcmp).
            let dec = e.extensions.iter().fold(Decoder::new(isa), |dec, group| {
                let ext = Extension::from_name(group[0]).unwrap();
                if group
                    .iter()
                    .any(|n| dec.extensions().contains(Extension::from_name(n).unwrap()))
                {
                    dec
                } else {
                    with_resolving_conflicts(dec, ext)
                }
            });
            for inst in e.samples(&mut rng, 64) {
                if reserved_by_spec(isa, e.name, inst) {
                    continue;
                }
                match decoded_name(dec, inst) {
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
    let entries = entries_fixed();
    let mut rng = Rng::new(2);
    let mut failures = Failures::default();
    let configs = [Extensions::DEFAULT, embedded(), rve(), inx()];
    for (isa, exts) in [Isa::Rv32, Isa::Rv64]
        .into_iter()
        .flat_map(|i| configs.map(|c| (i, c)))
    {
        let dec = Decoder::with_only(isa, exts).unwrap();
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
            let Some(name) = decoded_name(dec, inst) else {
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
