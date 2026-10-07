// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Decoding and formatting of specific instructions, and the public API.

use disc_v::{Isa, decode, decode_bytes, disassemble};

#[track_caller]
fn check(isa: Isa, inst: u32, expected: &str) {
    assert_eq!(
        decode(isa, 4, inst as u64).to_string(),
        expected,
        "{isa:?} {inst:#010x}"
    );
}

/// The test cases from the Caliptra emulator's disassembler, which cover
/// the bit-manipulation extensions. The expected text is GNU objdump's.
#[test]
fn caliptra_cases() {
    let cases = [
        (Isa::Rv32, 0x10018193, "addi gp,gp,256"),
        (Isa::Rv32, 0x0911003b, ".insn 4, 0x0911003b"),
        (Isa::Rv64, 0x0911003b, "add.uw zero,sp,a7"),
        (Isa::Rv64, 0x0800003b, "zext.w zero,zero"),
        (Isa::Rv32, 0x40007033, "andn zero,zero,zero"),
        (Isa::Rv32, 0x488595b3, "bclr a1,a1,s0"),
        (Isa::Rv32, 0x49d51513, "bclri a0,a0,0x1d"),
        (Isa::Rv64, 0x49d51513, "bclri a0,a0,0x1d"),
        (Isa::Rv32, 0x48855533, "bext a0,a0,s0"),
        (Isa::Rv32, 0x48265613, "bexti a2,a2,0x2"),
        (Isa::Rv64, 0x48265613, "bexti a2,a2,0x2"),
        (Isa::Rv32, 0x68261633, "binv a2,a2,sp"),
        (Isa::Rv32, 0x68261613, "binvi a2,a2,0x2"),
        (Isa::Rv64, 0x68261613, "binvi a2,a2,0x2"),
        (Isa::Rv32, 0x28c69633, "bset a2,a3,a2"),
        (Isa::Rv32, 0x28b01a93, "bseti s5,zero,0xb"),
        (Isa::Rv32, 0x28b01a93, "bseti s5,zero,0xb"),
        (Isa::Rv32, 0x0ac69633, "clmul a2,a3,a2"),
        (Isa::Rv32, 0x0ac63633, "clmulh a2,a2,a2"),
        (Isa::Rv32, 0x0ac62633, "clmulr a2,a2,a2"),
        (Isa::Rv32, 0x60001613, "clz a2,zero"),
        (Isa::Rv64, 0x6000161b, "clzw a2,zero"),
        (Isa::Rv32, 0x60201613, "cpop a2,zero"),
        (Isa::Rv64, 0x6020161b, "cpopw a2,zero"),
        (Isa::Rv32, 0x60101613, "ctz a2,zero"),
        (Isa::Rv64, 0x6010161b, "ctzw a2,zero"),
        (Isa::Rv32, 0x0a106633, "max a2,zero,ra"),
        (Isa::Rv32, 0x0a107633, "maxu a2,zero,ra"),
        (Isa::Rv32, 0x0a104633, "min a2,zero,ra"),
        (Isa::Rv32, 0x0a105633, "minu a2,zero,ra"),
        (Isa::Rv32, 0x28755613, "orc.b a2,a0"),
        (Isa::Rv32, 0x40006033, "orn zero,zero,zero"),
        (Isa::Rv32, 0x69855613, "rev8 a2,a0"),
        (Isa::Rv64, 0x6b855613, "rev8 a2,a0"),
        (Isa::Rv32, 0x60851633, "rol a2,a0,s0"),
        (Isa::Rv64, 0x6085163b, "rolw a2,a0,s0"),
        (Isa::Rv32, 0x60005633, "ror a2,zero,zero"),
        (Isa::Rv32, 0x61115613, "rori a2,sp,0x11"),
        (Isa::Rv64, 0x63115613, "rori a2,sp,0x31"),
        (Isa::Rv64, 0x6011561b, "roriw a2,sp,0x1"),
        (Isa::Rv64, 0x6085563b, "rorw a2,a0,s0"),
        (Isa::Rv32, 0x60411613, "sext.b a2,sp"),
        (Isa::Rv32, 0x60511613, "sext.h a2,sp"),
        (Isa::Rv32, 0x216b25b3, "sh1add a1,s6,s6"),
        (Isa::Rv64, 0x216a25bb, "sh1add.uw a1,s4,s6"),
        (Isa::Rv32, 0x20a5c533, "sh2add a0,a1,a0"),
        (Isa::Rv64, 0x216445bb, "sh2add.uw a1,s0,s6"),
        (Isa::Rv32, 0x20d56533, "sh3add a0,a0,a3"),
        (Isa::Rv64, 0x20d5653b, "sh3add.uw a0,a0,a3"),
        (Isa::Rv64, 0x0bd5151b, "slli.uw a0,a0,0x3d"),
        (Isa::Rv32, 0x40004033, "xnor zero,zero,zero"),
        (Isa::Rv32, 0x08004033, "zext.h zero,zero"),
        (Isa::Rv64, 0x0800403b, "zext.h zero,zero"),
    ];
    for (isa, inst, expected) in cases {
        check(isa, inst, expected);
    }
}

#[test]
fn aliases() {
    check(Isa::Rv64, 0x00000013, "nop");
    check(Isa::Rv64, 0x00a00513, "li a0,10");
    check(Isa::Rv64, 0x00008067, "ret");
    check(Isa::Rv64, 0x00450067, "jr 4(a0)");
    check(Isa::Rv64, 0x000500e7, "jalr a0");
    check(Isa::Rv64, 0x008000ef, "jal 0xc");
    check(Isa::Rv64, 0x00a5d463, "bge a1,a0,0xc");
    check(Isa::Rv64, 0xc0002573, "rdcycle a0");
    check(Isa::Rv64, 0x30002573, "csrr a0,mstatus");
    check(Isa::Rv64, 0x30051073, "csrw mstatus,a0");
    check(Isa::Rv64, 0x0ff57513, "zext.b a0,a0");
    check(Isa::Rv64, 0x22b58553, "fmv.d fa0,fa1");
    check(Isa::Rv64, 0x8330000f, "fence.tso");
    check(Isa::Rv64, 0x12050073, "sfence.vma a0");
}

#[test]
fn compressed() {
    check(Isa::Rv64, 0x4505, "li a0,1");
    check(Isa::Rv64, 0x8082, "ret");
    check(Isa::Rv64, 0x1141, "addi sp,sp,-16");
    check(Isa::Rv64, 0xe406, "sd ra,8(sp)");
    check(Isa::Rv32, 0x2001, "jal 0x4");
    // On RV64 this is c.addiw with rd=zero, which is reserved.
    check(Isa::Rv64, 0x2001, ".insn 2, 0x2001");
    check(Isa::Rv64, 0x0000, "unimp");
    check(Isa::Rv64, 0x0004, ".insn 2, 0x0004");
    // HINTs are shown as encoded.
    check(Isa::Rv64, 0x4005, "c.li zero,1");
    check(Isa::Rv64, 0x0005, "c.nop 1");
}

#[test]
fn operand_formats() {
    check(Isa::Rv64, 0x12345537, "lui a0,0x12345");
    check(Isa::Rv64, 0x00012517, "auipc a0,0x12");
    check(Isa::Rv64, 0x00351513, "slli a0,a0,0x3");
    check(Isa::Rv64, 0x02b50553, "fadd.d fa0,fa0,fa1,rne");
    check(Isa::Rv64, 0x02b57553, "fadd.d fa0,fa0,fa1");
    check(Isa::Rv64, 0x420585d3, "fcvt.d.s fa1,fa1");
    check(Isa::Rv64, 0x060525af, "amoadd.w.aqrl a1,zero,(a0)");
    check(Isa::Rv64, 0x100525af, "lr.w a1,(a0)");
    check(Isa::Rv64, 0x0330000f, "fence rw,rw");
}

/// Encodings that the specification reserves.
#[test]
fn reserved_encodings_are_illegal() {
    let cases = [
        (Isa::Rv64, 0x0004),     // c.addi4spn with a zero immediate
        (Isa::Rv64, 0x6101),     // c.addi16sp with a zero immediate
        (Isa::Rv64, 0x6201),     // c.lui with a zero immediate (and an even rd)
        (Isa::Rv64, 0x4002),     // c.lwsp with rd=zero
        (Isa::Rv64, 0x6002),     // c.ldsp with rd=zero
        (Isa::Rv64, 0x8002),     // c.jr with rs1=zero
        (Isa::Rv32, 0x9c21),     // c.addw, which is RV64-only
        (Isa::Rv32, 0x1082),     // c.slli with a shift amount of 32
        (Isa::Rv64, 0x101525af), // lr.w with rs2 nonzero
        (Isa::Rv32, 0x02051513), // slli with a shift amount of 32
        (Isa::Rv32, 0x00053503), // ld, which is RV64-only
        (Isa::Rv64, 0x00a54023), // sq, which is RV128-only
        (Isa::Rv64, 0x0000007b), // custom-3, which RV128 uses
    ];
    for (isa, inst) in cases {
        let ins = decode(isa, 0, inst);
        assert!(ins.is_illegal(), "{isa:?} {inst:#x} decoded as {ins}");
        assert!(ins.to_string().starts_with(".insn "), "{ins}");
    }
}

#[test]
fn illegal_encodings_are_shown_as_insn() {
    // The value is padded to a whole number of 16-bit parcels, as objdump does.
    check(Isa::Rv32, 0x0000007b, ".insn 4, 0x007b");
    check(Isa::Rv32, 0x0911003b, ".insn 4, 0x0911003b");
    check(Isa::Rv64, 0x6101, ".insn 2, 0x6101");
}

#[test]
fn cache_management_and_hints() {
    check(Isa::Rv64, 0x0015200f, "cbo.clean (a0)");
    check(Isa::Rv64, 0x0045200f, "cbo.zero (a0)");
    check(Isa::Rv64, 0x02156013, "prefetch.r 32(a0)");
    check(Isa::Rv64, 0x00256013, "ori zero,a0,2");
    check(Isa::Rv64, 0x00200033, "ntl.p1");
    check(Isa::Rv64, 0x900a, "ntl.p1");
    check(Isa::Rv64, 0x0100000f, "pause");
    // On RV128, this encoding is lq rather than cbo.inval.
    check(Isa::Rv128, 0x0005200f, "lq zero,0(a0)");
}

#[test]
fn vector() {
    check(Isa::Rv64, 0x022180d7, "vadd.vv v1,v2,v3");
    check(Isa::Rv64, 0x002180d7, "vadd.vv v1,v2,v3,v0.t");
    check(Isa::Rv64, 0xb62560d7, "vmacc.vx v1,a0,v2");
    check(Isa::Rv64, 0x402180d7, "vadc.vvm v1,v2,v3,v0");
    check(Isa::Rv64, 0x0d05f557, "vsetvli a0,a1,e32,m1,ta,ma");
    check(Isa::Rv64, 0x0045f557, "vsetvli a0,a1,4"); // reserved LMUL
    check(Isa::Rv64, 0x22056087, "vlseg2e32.v v1,(a0)");
    check(Isa::Rv64, 0x2e2fb0d7, "vnot.v v1,v2");
    check(Isa::Rv32, 0x02056087, "vle32.v v1,(a0)");
}

#[test]
fn vector_bit_manipulation_and_crypto() {
    check(Isa::Rv64, 0x062180d7, "vandn.vv v1,v2,v3");
    check(Isa::Rv64, 0x562fb0d7, "vror.vi v1,v2,63"); // 6-bit immediate
    check(Isa::Rv64, 0x4a2620d7, "vclz.v v1,v2");
    check(Isa::Rv64, 0x322560d7, "vclmul.vx v1,v2,a0");
    check(Isa::Rv64, 0xa220a0f7, "vaesdf.vv v1,v2");
    check(Isa::Rv64, 0xb221a0f7, "vghsh.vv v1,v2,v3");
    check(Isa::Rv64, 0x8a22a0f7, "vaeskf1.vi v1,v2,5");
    check(Isa::Rv64, 0xba21a0f7, "vsha2ch.vv v1,v2,v3");
    check(Isa::Rv64, 0xae23a0f7, "vsm3c.vi v1,v2,7");
}

#[test]
fn half_precision() {
    check(Isa::Rv64, 0x00051507, "flh fa0,0(a0)");
    check(Isa::Rv64, 0x04b57553, "fadd.h fa0,fa0,fa1");
    check(Isa::Rv64, 0x24b58553, "fmv.h fa0,fa1");
    check(Isa::Rv64, 0x40258553, "fcvt.s.h fa0,fa1");
    check(Isa::Rv64, 0xe4058553, "fmv.x.h a0,fa1");
}

#[test]
fn zfa() {
    check(Isa::Rv64, 0xf0100553, "fli.s fa0,-0x1p+0");
    check(Isa::Rv64, 0xf0108553, "fli.s fa0,min");
    check(Isa::Rv64, 0xf0180553, "fli.s fa0,0x1p+0");
    check(Isa::Rv64, 0xf01f8553, "fli.s fa0,nan");
    check(Isa::Rv64, 0x40458553, "fround.s fa0,fa1,rne");
    check(Isa::Rv64, 0xc2859553, "fcvtmod.w.d a0,fa1,rtz");
    check(Isa::Rv32, 0xe2158553, "fmvh.x.d a0,fa1");
    check(Isa::Rv64, 0xe2158553, ".insn 4, 0xe2158553"); // fmvh.x.d is RV32-only
}

#[test]
fn zcb() {
    check(Isa::Rv64, 0x85a8, "lhu a0,2(a1)");
    check(Isa::Rv64, 0x8da8, "sh a0,2(a1)");
    check(Isa::Rv64, 0x9d61, "zext.b a0,a0");
    check(Isa::Rv64, 0x9d71, "zext.w a0,a0");
    check(Isa::Rv32, 0x9d71, ".insn 2, 0x9d71"); // c.zext.w is RV64-only
    check(Isa::Rv64, 0x9d4d, "mul a0,a0,a1");
    assert_eq!(
        decode(Isa::Rv64, 0, 0x9d75).without_aliases().to_string(),
        "c.not a0"
    );
}

#[test]
fn may_be_operations() {
    check(Isa::Rv64, 0x81c5c573, "mop.r.0 a0,a1");
    check(Isa::Rv64, 0x82c5c573, "mop.rr.0 a0,a1,a2");
    check(Isa::Rv64, 0xcdc0c073, "sspopchk ra");
    check(Isa::Rv64, 0xcdc04573, "ssrdp a0");
    check(Isa::Rv64, 0xce104073, "sspush ra");
    check(Isa::Rv64, 0xcdc5c073, "mop.r.28 zero,a1");
    check(Isa::Rv64, 0x6081, "sspush ra");
    check(Isa::Rv64, 0x6181, "c.mop.3");
    check(Isa::Rv64, 0x00012017, "lpad 0x12");
    check(Isa::Rv64, 0x4cb5352f, "ssamoswap.d.aq a0,a1,(a0)");
    assert_eq!(
        decode(Isa::Rv64, 0, 0x6081).without_aliases().to_string(),
        "c.mop.1"
    );
}

#[test]
fn hypervisor_and_privileged() {
    check(Isa::Rv64, 0x22b50073, "hfence.vvma a0,a1");
    check(Isa::Rv64, 0x22050073, "hfence.vvma a0");
    check(Isa::Rv64, 0x62000073, "hfence.gvma");
    check(Isa::Rv64, 0x6005c573, "hlv.b a0,(a1)");
    check(Isa::Rv64, 0x6435c573, "hlvx.hu a0,(a1)");
    check(Isa::Rv64, 0x6eb54073, "hsv.d a1,(a0)");
    check(Isa::Rv32, 0x6eb54073, ".insn 4, 0x6eb54073"); // hsv.d is RV64-only
    check(Isa::Rv64, 0x16000073, "sinval.vma zero,zero");
    check(Isa::Rv64, 0x18100073, "sfence.inval.ir");
    check(Isa::Rv64, 0x70200073, "mnret");
    check(Isa::Rv64, 0x10400073, "sctrclr");
}

#[test]
fn atomics_extensions() {
    // Zalasr: the name's ordering is shown with the other ordering bit.
    check(Isa::Rv64, 0x3405852f, "lb.aq a0,(a1)");
    check(Isa::Rv64, 0x3605a52f, "lw.aqrl a0,(a1)");
    check(Isa::Rv64, 0x3aa5802f, "sb.rl a0,(a1)");
    check(Isa::Rv64, 0x3ea5b02f, "sd.aqrl a0,(a1)");
    check(Isa::Rv32, 0x3ea5b02f, ".insn 4, 0x3ea5b02f"); // sd.rl is RV64-only
    // Zabha and Zacas.
    check(Isa::Rv64, 0x00c5852f, "amoadd.b a0,a2,(a1)");
    check(Isa::Rv64, 0x0ec5952f, "amoswap.h.aqrl a0,a2,(a1)");
    check(Isa::Rv64, 0x28c5a52f, "amocas.w a0,a2,(a1)");
    check(Isa::Rv64, 0x28c5852f, "amocas.b a0,a2,(a1)");
    // On RV32, amocas.d works on register pairs, so odd registers are reserved.
    check(Isa::Rv32, 0x2cc5b52f, "amocas.d.aq a0,a2,(a1)");
    check(Isa::Rv32, 0x28d5b52f, ".insn 4, 0x28d5b52f");
    check(Isa::Rv64, 0x28d5b52f, "amocas.d a0,a3,(a1)");
}

#[test]
fn bfloat16() {
    check(Isa::Rv64, 0x4485f553, "fcvt.bf16.s fa0,fa1");
    check(Isa::Rv64, 0x40658553, "fcvt.s.bf16 fa0,fa1");
    check(Isa::Rv64, 0xee2550d7, "vfwmaccbf16.vf v1,fa0,v2");
    check(Isa::Rv64, 0x4a2e90d7, "vfncvtbf16.f.f.w v1,v2");
}

#[test]
fn scalar_cryptography() {
    check(Isa::Rv32, 0x08c5c533, "pack a0,a1,a2");
    check(Isa::Rv32, 0x0805c533, "zext.h a0,a1");
    check(Isa::Rv32, 0x08c5f533, "packh a0,a1,a2");
    check(Isa::Rv32, 0x6875d513, "brev8 a0,a1");
    check(Isa::Rv32, 0x08f59513, "zip a0,a1");
    check(Isa::Rv32, 0x28c5a533, "xperm4 a0,a1,a2");
    check(Isa::Rv32, 0xe2c58533, "aes32esi a0,a1,a2,0x3");
    check(Isa::Rv32, 0x70c58533, "sm4ed a0,a1,a2,0x1");
    check(Isa::Rv32, 0x10059513, "sha256sum0 a0,a1");
    check(Isa::Rv32, 0x50c58533, "sha512sum0r a0,a1,a2");
    check(Isa::Rv64, 0x32c58533, "aes64es a0,a1,a2");
    check(Isa::Rv64, 0x31a59513, "aes64ks1i a0,a1,0xa");
    // Round numbers above 10 are reserved, but decoded.
    check(Isa::Rv64, 0x31b59513, "aes64ks1i a0,a1,0xb");
    check(Isa::Rv64, 0x08c5c53b, "packw a0,a1,a2");
}

#[test]
fn rv128() {
    check(Isa::Rv128, 0x0005200f, "lq zero,0(a0)");
    check(Isa::Rv128, 0x00053503, "ld a0,0(a0)");
    check(Isa::Rv128, 0x2108, "lq a0,0(a0)");
}

#[test]
fn branch_targets_wrap_to_xlen() {
    let j_back = 0xffdff06f; // j -4
    assert_eq!(decode(Isa::Rv32, 0, j_back).to_string(), "j 0xfffffffc");
    assert_eq!(
        decode(Isa::Rv64, 0, j_back).to_string(),
        "j 0xfffffffffffffffc"
    );
}

#[test]
fn accessors() {
    let ins = decode(Isa::Rv64, 0x1000, 0x00b50463); // beq a0,a1,8
    assert_eq!(ins.mnemonic(), "beq");
    assert_eq!(ins.operands().to_string(), "a0,a1,0x1008");
    assert_eq!(ins.pc(), 0x1000);
    assert_eq!(ins.raw(), 0x00b50463);
    assert_eq!(ins.length(), 4);
    assert_eq!(ins.isa(), Isa::Rv64);
    assert!(!ins.is_illegal());

    let li = decode(Isa::Rv64, 0, 0x4505);
    assert_eq!(li.to_string(), "li a0,1");
    assert_eq!(li.without_aliases().to_string(), "c.li a0,1");
    assert_eq!(li.without_aliases().mnemonic(), "c.li");
    assert_eq!(
        decode(Isa::Rv64, 0, 0x00a00513)
            .without_aliases()
            .to_string(),
        "addi a0,zero,10"
    );

    let amo = decode(Isa::Rv64, 0, 0x060525af);
    assert_eq!(amo.mnemonic(), "amoadd.w");
    assert_eq!(amo.operands().to_string(), "a1,zero,(a0)");
}

#[test]
fn bits_beyond_the_instruction_are_ignored() {
    assert_eq!(
        decode(Isa::Rv64, 0, 0xdead_beef_0000_0013).to_string(),
        "nop"
    );
    assert_eq!(decode(Isa::Rv64, 0, 0xdead_beef_dead_4505).raw(), 0x4505);
}

#[test]
fn long_encodings() {
    // 48-bit and 64-bit encodings are recognized but not decoded.
    let ins = decode_bytes(Isa::Rv64, 0, &[0x1f, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!((ins.length(), ins.is_illegal()), (6, true));
    assert_eq!(ins.to_string(), ".insn 6, 0x001f");
    let ins = decode_bytes(Isa::Rv64, 0, &[0x3f, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!((ins.length(), ins.is_illegal()), (8, true));
    assert_eq!(ins.to_string(), ".insn 8, 0x003f");
    // Reserved lengths are shown two bytes at a time.
    let ins = decode_bytes(Isa::Rv64, 0, &[0x7f, 0, 0, 0]).unwrap();
    assert_eq!((ins.length(), ins.is_illegal()), (2, true));
    assert_eq!(ins.to_string(), ".2byte 0x007f");
}

#[test]
fn decode_bytes_needs_the_whole_instruction() {
    assert_eq!(decode_bytes(Isa::Rv64, 0, &[]), None);
    assert_eq!(decode_bytes(Isa::Rv64, 0, &[0x05]), None);
    assert_eq!(decode_bytes(Isa::Rv64, 0, &[0x13, 0x05, 0x10]), None);
    assert_eq!(
        decode_bytes(Isa::Rv64, 0, &[0x05, 0x45])
            .unwrap()
            .to_string(),
        "li a0,1"
    );
}

#[test]
fn disassemble_iterates_over_mixed_lengths() {
    let code = [
        0x13, 0x05, 0x10, 0x00, // li a0,1
        0x82, 0x80, // ret
        0x1f, 0, 0, 0, 0, 0, // a 48-bit instruction
        0x13, 0x05, // the start of a truncated instruction
    ];
    let mut iter = disassemble(Isa::Rv64, 0x100, &code);
    let listing: Vec<(u64, String)> = iter.by_ref().map(|i| (i.pc(), i.to_string())).collect();
    assert_eq!(
        listing,
        [
            (0x100, "li a0,1".to_string()),
            (0x104, "ret".to_string()),
            (0x106, ".insn 6, 0x001f".to_string())
        ]
    );
    assert_eq!(iter.remainder(), [0x13, 0x05]);
    assert_eq!(iter.pc(), 0x10c);
}
