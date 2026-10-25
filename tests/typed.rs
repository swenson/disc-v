// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! The typed view of an instruction: its [`Mnemonic`] and operand fields.
//!
//! Each case states the instruction's objdump text too, so the encoding is
//! the one the case claims to be.

use disc_v::{Decoder, Extension, Isa, Mnemonic, RoundingMode, decode};

#[track_caller]
fn at(isa: Isa, pc: u64, inst: u32, text: &str) -> disc_v::Instruction {
    let ins = decode(isa, pc, inst as u64);
    assert_eq!(ins.to_string(), text, "{inst:#010x}");
    ins
}

#[track_caller]
fn rv64(inst: u32, text: &str) -> disc_v::Instruction {
    at(Isa::Rv64, 0x1000, inst, text)
}

/// The mnemonic shown, and the one encoded, are typed as their names are.
#[test]
fn the_mnemonic_is_typed() {
    let li = rv64(0x4505, "li a0,1");
    assert_eq!(li.id(), Mnemonic::Li);
    assert_eq!(li.without_aliases().id(), Mnemonic::CLi);
    assert_eq!(li.without_aliases().id().name(), "c.li");
    let fcvt = rv64(0xd231_f553, "fcvt.d.lu fa0,gp");
    assert_eq!(fcvt.id(), Mnemonic::FcvtDLu);
    assert_eq!(Mnemonic::FcvtDLu.to_string(), "fcvt.d.lu");
    let illegal = at(Isa::Rv32, 0x1000, 0x0911_003b, ".insn 4, 0x0911003b");
    assert_eq!(illegal.id(), Mnemonic::Illegal);
    assert!(illegal.is_illegal());
    let parcel = rv64(0xffff_ffff, ".2byte 0xffff");
    assert_eq!(parcel.id(), Mnemonic::ReservedParcel);
    assert!(parcel.is_illegal());
    for ins in [li, fcvt, illegal, parcel] {
        assert_eq!(ins.id().name(), ins.mnemonic());
    }
}

/// A compressed instruction's registers are the ones it executes with: the
/// 3-bit fields name `x8` to `x15`, and the registers the encoding implies are
/// filled in.
#[test]
fn compressed_registers_are_as_executed() {
    let ld = rv64(0x6500, "ld s0,8(a0)");
    assert_eq!((ld.rd(), ld.rs1(), ld.imm()), (8, 10, 8));
    let ldsp = rv64(0x60a2, "ld ra,8(sp)");
    assert_eq!((ldsp.rd(), ldsp.rs1(), ldsp.imm()), (1, 2, 8));
    let jalr = rv64(0x9782, "jalr a5");
    assert_eq!(jalr.without_aliases().id(), Mnemonic::CJalr);
    assert_eq!((jalr.rd(), jalr.rs1()), (1, 15));
    let spn = rv64(0x0200, "addi s0,sp,256");
    assert_eq!((spn.rd(), spn.rs1(), spn.imm()), (8, 2, 256));
}

/// An upper immediate is the value written, and a PC-relative one names its
/// target.
#[test]
fn immediates_are_as_used() {
    let lui = rv64(0x8421_17b7, "lui a5,0x84211");
    assert_eq!(lui.imm(), 0x8421_1000_u32 as i32);
    let c_lui = rv64(0x77fd, "lui a5,0xfffff");
    assert_eq!(c_lui.imm(), -0x1000);
    let auipc = rv64(0x0000_1517, "auipc a0,0x1");
    assert_eq!((auipc.imm(), auipc.target()), (0x1000, 0x2000));
    let beq = rv64(0xfe05_0ee3, "beqz a0,0xffc");
    assert_eq!(
        (beq.rs1(), beq.rs2(), beq.imm(), beq.target()),
        (10, 0, -4, 0xffc)
    );
    let srai = rv64(0x43f5_5513, "srai a0,a0,0x3f");
    assert_eq!(srai.imm(), 63);
    let sd = rv64(0xfe11_3c23, "sd ra,-8(sp)");
    assert_eq!((sd.rd(), sd.rs1(), sd.rs2(), sd.imm()), (0, 2, 1, -8));
    // RV32 wraps the target to its 32-bit addresses.
    let j = at(Isa::Rv32, 0, 0xffdf_f06f, "j 0xfffffffc");
    assert_eq!(j.target(), 0xffff_fffc);
}

/// A CSR instruction's immediate is the CSR number, and its `zimm` is in rs1.
#[test]
fn csr_fields() {
    let frflags = rv64(0x0010_2573, "frflags a0");
    assert_eq!(frflags.without_aliases().id(), Mnemonic::Csrrs);
    assert_eq!((frflags.rd(), frflags.rs1(), frflags.imm()), (10, 0, 1));
    let fsrmi = rv64(0x0021_d073, "fsrmi zero,3");
    assert_eq!(fsrmi.without_aliases().id(), Mnemonic::Csrrwi);
    assert_eq!((fsrmi.rs1(), fsrmi.imm()), (3, 2));
}

/// The rounding mode is typed, and absent where there is no field.
#[test]
fn rounding_modes() {
    let rtz = rv64(0xc205_1553, "fcvt.w.d a0,fa0,rtz");
    assert_eq!(rtz.rounding_mode(), Some(RoundingMode::Rtz));
    let dynamic = rv64(0x02b5_7553, "fadd.d fa0,fa0,fa1");
    assert_eq!(dynamic.rounding_mode(), Some(RoundingMode::Dyn));
    let fmadd = rv64(0x6ac5_f543, "fmadd.d fa0,fa1,fa2,fa3");
    assert_eq!(
        (fmadd.rd(), fmadd.rs1(), fmadd.rs2(), fmadd.rs3()),
        (10, 11, 12, 13)
    );
    assert_eq!(fmadd.rounding_mode(), Some(RoundingMode::Dyn));
    let add = rv64(0x00b5_0533, "add a0,a0,a1");
    assert_eq!(add.rounding_mode(), None);
}

/// Atomic ordering and fence sets.
#[test]
fn ordering_and_fences() {
    let amo = rv64(0x06b6_252f, "amoadd.w.aqrl a0,a1,(a2)");
    assert_eq!(amo.id(), Mnemonic::AmoaddW);
    assert!(amo.aq() && amo.rl());
    assert_eq!((amo.rd(), amo.rs1(), amo.rs2()), (10, 12, 11));
    let plain = rv64(0x00b6_252f, "amoadd.w a0,a1,(a2)");
    assert!(!plain.aq() && !plain.rl());
    let fence = rv64(0x0330_000f, "fence rw,rw");
    assert_eq!((fence.pred(), fence.succ()), (3, 3));
    let all = rv64(0x0ff0_000f, "fence iorw,iorw");
    assert_eq!((all.pred(), all.succ()), (15, 15));
}

/// A vector instruction states whether `v0` masks it.
#[cfg(feature = "vector")]
#[test]
fn vector_masking() {
    let masked = rv64(0x0021_80d7, "vadd.vv v1,v2,v3,v0.t");
    assert_eq!(masked.id(), Mnemonic::VaddVv);
    assert_eq!((masked.rd(), masked.rs1(), masked.rs2()), (1, 3, 2));
    assert!(masked.is_masked());
    assert!(!rv64(0x0221_80d7, "vadd.vv v1,v2,v3").is_masked());
}

/// With Zfinx the floating-point operands are integer registers.
#[test]
fn zfinx_operands_are_integer_registers() {
    let f = rv64(0x00b5_7553, "fadd.s fa0,fa0,fa1");
    assert!(!f.fp_in_x_registers());
    let zfinx = Decoder::with_only(Isa::Rv64, [Extension::Zfinx]).unwrap();
    let x = zfinx.decode(0x1000, 0x00b5_7553);
    assert_eq!(x.to_string(), "fadd.s a0,a0,a1");
    assert!(x.fp_in_x_registers());
    assert_eq!((x.rd(), x.rs1(), x.rs2()), (10, 10, 11));
}
