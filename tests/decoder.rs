// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Choosing extensions with `Decoder`.

pub mod common;

use common::{Failures, Rng, entries};
use disc_v::{Decoder, Extension, Extensions, Isa, MarchError, decode};

#[track_caller]
fn check(dec: Decoder, inst: u32, expected: &str) {
    assert_eq!(
        dec.decode(0, inst as u64).to_string(),
        expected,
        "{dec:?} {inst:#010x}"
    );
}

#[test]
fn free_functions_use_the_default_extensions() {
    let mut rng = Rng::new(4);
    for isa in [Isa::Rv32, Isa::Rv64, Isa::Rv128] {
        let dec = Decoder::new(isa);
        assert_eq!(dec.extensions(), Extensions::DEFAULT);
        for _ in 0..100_000 {
            let inst = rng.next_u32() as u64;
            assert_eq!(dec.decode(0x1000, inst), decode(isa, 0x1000, inst));
        }
    }
}

#[test]
fn presets() {
    let czero = 0x0ec5d533; // czero.eqz a0,a1,a2
    check(Decoder::RV64GC, czero, ".insn 4, 0x0ec5d533");
    check(Decoder::RVA23U64, czero, "czero.eqz a0,a1,a2");
    check(Decoder::RV64GC, 0x02b50553, "fadd.d fa0,fa0,fa1,rne");
    check(Decoder::RV32GC, 0x00053503, ".insn 4, 0x00053503"); // ld is RV64-only

    // RVA23U64 has Zfhmin but not Zfh, and no Zifencei; RVA23S64 adds
    // Zifencei, Svinval and H.
    check(Decoder::RVA23U64, 0x00051507, "flh fa0,0(a0)");
    check(Decoder::RVA23U64, 0x04b57553, ".insn 4, 0x04b57553"); // fadd.h
    check(Decoder::RVA23U64, 0x0000100f, ".insn 4, 0x100f"); // fence.i
    check(Decoder::RVA23S64, 0x0000100f, "fence.i");
    check(Decoder::RVA23U64, 0x22b50073, ".insn 4, 0x22b50073"); // hfence.vvma
    check(Decoder::RVA23S64, 0x22b50073, "hfence.vvma a0,a1");
}

#[test]
fn aliases_need_their_extensions() {
    let lpad = 0x00012017; // auipc zero,0x12
    check(Decoder::RVA23U64, lpad, "auipc zero,0x12");
    check(
        Decoder::RVA23U64.with(Extension::Zicfilp).unwrap(),
        lpad,
        "lpad 0x12",
    );

    let pause = 0x0100000f; // fence w,0
    check(Decoder::RV64GC, pause, "fence w,unknown");
    check(Decoder::RVA23U64, pause, "pause");

    let prefetch = 0x02156013; // ori zero,a0,33
    check(Decoder::RV64GC, prefetch, "ori zero,a0,33");
    check(Decoder::RVA23U64, prefetch, "prefetch.r 32(a0)");

    let ntl = 0x00200033; // add zero,zero,sp
    check(Decoder::RV64GC, ntl, "add zero,zero,sp");
    check(Decoder::RVA23U64, ntl, "ntl.p1");

    let sspush = 0xce104073; // mop.rr.7 zero,zero,ra
    check(Decoder::RVA23U64, sspush, "mop.rr.7 zero,zero,ra");
    check(
        Decoder::RVA23U64.with(Extension::Zicfiss).unwrap(),
        sspush,
        "sspush ra",
    );
}

#[test]
fn with_without_and_implications() {
    let dec = Decoder::RV64GC.without(Extension::F);
    assert!(!dec.extensions().contains(Extension::D));
    check(dec, 0x00053507, ".insn 4, 0x00053507"); // fld
    check(dec, 0x2108, ".insn 2, 0x2108"); // c.fld

    let dec = Decoder::RV64GC.with(Extension::Zfh).unwrap();
    assert!(dec.extensions().contains(Extension::Zfhmin));
    check(dec, 0x04b57553, "fadd.h fa0,fa0,fa1");

    // Nothing conflicts yet, so try_with always adds the extension.
    assert_eq!(
        Decoder::RV64GC.try_with(Extension::Zba),
        Decoder::RV64GC.with(Extension::Zba).unwrap()
    );
}

#[test]
fn with_only() {
    let dec = Decoder::with_only(Isa::Rv32, [Extension::M, Extension::C]).unwrap();
    assert_eq!(dec.isa(), Isa::Rv32);
    check(dec, 0x02b50533, "mul a0,a0,a1");
    check(dec, 0x4505, "li a0,1");
    check(dec, 0x00b57553, ".insn 4, 0x00b57553"); // fadd.s
    check(dec, 0x30002573, ".insn 4, 0x30002573"); // csrr: no Zicsr

    let dec = Decoder::with_only(Isa::Rv64, Extensions::EMPTY).unwrap();
    check(dec, 0x00a50513, "addi a0,a0,10");
    check(dec, 0x4505, ".insn 2, 0x4505");
}

#[test]
fn from_march() {
    let gc = Decoder::RV64GC;
    assert_eq!(Decoder::from_march("rv64gc"), Ok(gc));
    assert_eq!(Decoder::from_march("RV64GC"), Ok(gc));
    assert_eq!(Decoder::from_march("rv64imafdc_zicsr_zifencei"), Ok(gc));
    assert_eq!(
        Decoder::from_march("rv64i2p1_m2p0_a_f_d_c_zicsr2p0_zifencei"),
        Ok(gc)
    );
    assert_eq!(Decoder::from_march("rv32gc"), Ok(Decoder::RV32GC));
    assert_eq!(
        Decoder::from_march("rv64gcb_zicond"),
        Ok(Decoder::RV64GC
            .with(Extension::Zba)
            .and_then(|d| d.with(Extension::Zbb))
            .and_then(|d| d.with(Extension::Zbs))
            .and_then(|d| d.with(Extension::Zicond))
            .unwrap()),
    );
    // Extensions that define no instructions are ignored, and subsets are
    // decoded as the extension containing them.
    assert_eq!(
        Decoder::from_march("rv64gc_zicntr_zkt_sstc_zvl128b"),
        Ok(gc)
    );
    assert_eq!(
        Decoder::from_march("rv32i_zmmul_zca").map(|d| d.extensions()),
        Ok(Extensions::from([Extension::M, Extension::C])),
    );
    assert_eq!(
        Decoder::from_march("rv64gc_zve64d").map(|d| d.extensions()),
        Ok(Extensions::GC.with(Extension::V)),
    );

    assert_eq!(Decoder::from_march("x86"), Err(MarchError::InvalidBase));
    assert_eq!(Decoder::from_march("rv64m"), Err(MarchError::InvalidBase));
    assert_eq!(
        Decoder::from_march("rv32e"),
        Err(MarchError::UnsupportedExtension("e"))
    );
    assert_eq!(
        Decoder::from_march("rv64gc_zbkb"),
        Err(MarchError::UnsupportedExtension("zbkb"))
    );
    assert_eq!(
        Decoder::from_march("rv64gcp"),
        Err(MarchError::UnsupportedExtension("p"))
    );
}

#[test]
fn disassemble_uses_the_decoder() {
    let code = [0x33, 0xd5, 0xc5, 0x0e, 0x05, 0x45]; // czero.eqz a0,a1,a2; li a0,1
    let text: Vec<String> = Decoder::RV64GC
        .disassemble(0, &code)
        .map(|i| i.to_string())
        .collect();
    assert_eq!(text, [".insn 4, 0x0ec5d533", "li a0,1"]);
}

/// Every instruction in riscv-opcodes is illegal (or another instruction)
/// when any extension it needs is disabled.
#[test]
fn instructions_need_their_extensions() {
    let entries = entries();
    let mut rng = Rng::new(5);
    let mut failures = Failures::default();
    for isa in [Isa::Rv32, Isa::Rv64] {
        for e in entries
            .iter()
            .filter(|e| e.applies_to(isa) && e.base.is_none())
        {
            for name in &e.extensions {
                let ext = Extension::from_name(name).unwrap_or_else(|| panic!("unknown {name}"));
                let dec = Decoder::new(isa).without(ext);
                for inst in e.samples(&mut rng, 16) {
                    let ins = dec.decode(0, inst as u64).without_aliases();
                    if !ins.is_illegal() && ins.mnemonic() == e.name {
                        failures.add(
                            format_args!("{isa:?} {} decoded without {name}", e.name),
                            format_args!("{inst:#010x}"),
                        );
                    }
                }
            }
        }
    }
    failures.assert_none();
}

/// The test above covers every extension that defines instructions of its
/// own. The rest only define aliases, which `aliases_need_their_extensions`
/// covers, or (Zvknhb) share another extension's instructions.
#[test]
fn riscv_opcodes_covers_every_extension() {
    use Extension::*;
    let aliases_only = [Zicbop, Zihintntl, Zihintpause, Zicfilp, Zvknhb];
    let named: Vec<&str> = entries()
        .iter()
        .filter(|e| e.base.is_none())
        .flat_map(|e| e.extensions.clone())
        .collect();
    for ext in Extensions::DEFAULT.iter() {
        assert_eq!(
            named.contains(&ext.name()),
            !aliases_only.contains(&ext),
            "riscv-opcodes instructions for {ext}"
        );
    }
}
