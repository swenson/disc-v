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
fn shared_instructions() {
    // Zbkb provides rol and zext.h (as pack with rs2=zero) but not clz,
    // which only Zbb has.
    let zbkb = Decoder::with_only(Isa::Rv32, [Extension::Zbkb]).unwrap();
    check(zbkb, 0x60c59533, "rol a0,a1,a2");
    check(zbkb, 0x0805c533, "zext.h a0,a1");
    check(zbkb, 0x60059513, ".insn 4, 0x60059513"); // clz
    // Zbb provides zext.h but not pack.
    let zbb = Decoder::with_only(Isa::Rv32, [Extension::Zbb]).unwrap();
    check(zbb, 0x0805c533, "zext.h a0,a1");
    check(zbb, 0x08c5c533, ".insn 4, 0x08c5c533"); // pack
}

#[test]
fn conflicts() {
    use Extension::*;
    let push = 0xb862; // cm.push {ra,s0-s1},-16; c.fsdsp with Zcd

    // RV32GC has C and D, so Zcd, which Zcmp and Zcmt conflict with.
    let gc = Decoder::RV32GC;
    assert!(gc.extensions().contains(Zcd));
    let conflict = gc.with(Zcmp).unwrap_err();
    assert_eq!(
        (conflict.extension(), conflict.conflicts_with()),
        (Zcmp, Zcd)
    );
    assert_eq!(gc.try_with(Zcmp), gc);
    assert_eq!(gc.try_with(Zcmt), gc);
    check(gc, push, "fsd fs8,48(sp)");

    // Without Zcd (which also removes the C bundle but keeps Zca and D),
    // Zcmp is allowed.
    let dec = gc.without(Zcd).with(Zcmp).unwrap();
    assert!(dec.extensions().contains(D) && dec.extensions().contains(Zca));
    assert!(!dec.extensions().contains(C));
    check(dec, push, "cm.push {ra,s0-s1},-16");
    check(dec, 0x2108, ".insn 2, 0x2108"); // c.fld needs Zcd
    check(dec, 0x4505, "li a0,1");

    // The defaults leave out Zcmp and Zcmt.
    assert!(!Extensions::DEFAULT.contains(Zcmp));
    check(Decoder::new(Isa::Rv32), push, "fsd fs8,48(sp)");
    assert!(Decoder::with_only(Isa::Rv32, [C, D, Zcmp]).is_err());
}

#[test]
fn push_pop_and_table_jumps() {
    let rv32 = Decoder::from_march("rv32i_zca_zcmp_zcmt").unwrap();
    let rv64 = Decoder::from_march("rv64i_zca_zcmp_zcmt").unwrap();
    // The stack adjustment depends on the register list and XLEN.
    check(rv32, 0xb862, "cm.push {ra,s0-s1},-16");
    check(rv64, 0xb862, "cm.push {ra,s0-s1},-32");
    check(rv32, 0xba56, "cm.pop {ra,s0},32");
    check(rv32, 0xbe4e, "cm.popret {ra},64");
    check(rv32, 0xbc6a, "cm.popretz {ra,s0-s1},48");
    check(rv64, 0xbc6a, "cm.popretz {ra,s0-s1},64");
    check(rv32, 0xb802, ".insn 2, 0xb802"); // reserved register list
    check(rv32, 0xac26, "cm.mvsa01 s0,s1");
    check(rv32, 0xac62, "cm.mva01s s0,s0");
    check(rv32, 0xac22, ".insn 2, 0xac22"); // cm.mvsa01 s0,s0 is reserved
    check(rv32, 0xa07e, "cm.jt 31");
    check(rv32, 0xa082, "cm.jalt 32");
}

#[test]
fn compressed_bundle() {
    use Extension::*;
    let exts = Extensions::from([C, F, D]);
    assert!(exts.contains(Zca) && exts.contains(Zcf) && exts.contains(Zcd));
    // Removing D keeps C (and Zcf); adding it back brings Zcd back.
    let no_d = exts.without(D);
    assert!(no_d.contains(C) && no_d.contains(Zcf) && !no_d.contains(Zcd));
    assert!(no_d.with(D).contains(Zcd));
    // Removing C removes every compressed extension.
    let no_c = Extensions::from([C, F, Zcb]).without(C);
    assert!(!no_c.contains(Zca) && !no_c.contains(Zcf) && !no_c.contains(Zcb));
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
        Ok(Extensions::from([Extension::M, Extension::Zca])),
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
        Decoder::from_march("rv64gc_zilsd"),
        Err(MarchError::UnsupportedExtension("zilsd"))
    );
    // Zcmp conflicts with Zcd, which C brings in with D.
    assert_eq!(
        Decoder::from_march("rv32gc_zcmp").map_err(|e| e.to_string()),
        Err("zcmp conflicts with zcd".to_string()),
    );
    assert_eq!(
        Decoder::from_march("rv32imac_zcmp").map(|d| d.extensions()),
        Ok(Extensions::from([
            Extension::M,
            Extension::A,
            Extension::C,
            Extension::Zcmp
        ])),
    );
    assert_eq!(
        Decoder::from_march("rv32imaf_zce").map(|d| d.extensions()),
        Ok(Extensions::from([
            Extension::M,
            Extension::A,
            Extension::F,
            Extension::Zca,
            Extension::Zcb,
            Extension::Zcmp,
            Extension::Zcmt
        ])),
    );
    // Scalar cryptography bundles are expanded.
    assert_eq!(
        Decoder::from_march("rv64gc_zkn").map(|d| d.extensions()),
        Ok(Extensions::GC
            | Extensions::from([
                Extension::Zbkb,
                Extension::Zbkc,
                Extension::Zbkx,
                Extension::Zkne,
                Extension::Zknd,
                Extension::Zknh,
            ])),
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
/// when any extension it needs is disabled. An instruction that several
/// extensions provide needs all of them disabled.
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
            for group in &e.extensions {
                let dec = group.iter().fold(Decoder::new(isa), |dec, name| {
                    let ext =
                        Extension::from_name(name).unwrap_or_else(|| panic!("unknown {name}"));
                    dec.without(ext)
                });
                for inst in e.samples(&mut rng, 16) {
                    let ins = dec.decode(0, inst as u64).without_aliases();
                    if !ins.is_illegal() && ins.mnemonic() == e.name {
                        failures.add(
                            format_args!("{isa:?} {} decoded without {}", e.name, group.join("|")),
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
    // C is a bundle of Zca and Zcd.
    let aliases_only = [C, Zicbop, Zihintntl, Zihintpause, Zicfilp, Zvknhb];
    let named: Vec<&str> = entries()
        .iter()
        .filter(|e| e.base.is_none())
        .flat_map(|e| e.extensions.concat())
        .collect();
    for ext in Extensions::DEFAULT.iter().chain([Zcmp, Zcmt]) {
        assert_eq!(
            named.contains(&ext.name()),
            !aliases_only.contains(&ext),
            "riscv-opcodes instructions for {ext}"
        );
    }
}
