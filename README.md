# disc-v

[![CI](https://github.com/swenson/disc-v/actions/workflows/ci.yml/badge.svg)](https://github.com/swenson/disc-v/actions/workflows/ci.yml)

A RISC-V disassembler in Rust. It is `no_std`, does not allocate, and its
output follows GNU objdump's (see [Output format](#output-format)).

It is meant to be embedded in various Rust tools: debuggers, emulators, etc.

```rust
use disc_v::{disassemble, Isa};

let code = [0x13, 0x05, 0x10, 0x00, 0x82, 0x80];
for ins in disassemble(Isa::Rv64, 0x1000, &code) {
    println!("{:x}: {ins}", ins.pc());
}
// 1000: li a0,1
// 1004: ret
```

Single instructions can be decoded from bytes with `decode_bytes`, or from
an integer with `decode`. An `Instruction` has the mnemonic, operands,
address, length and raw encoding, and `without_aliases()` shows it as
encoded (`c.li a0,1` rather than `li a0,1`), like `objdump -M no-aliases`.

The minimum supported Rust version is 1.85.1.

## Choosing extensions

`decode`, `decode_bytes` and `disassemble` decode every supported extension
that does not conflict with another. To decode for a particular target, so
that other extensions' instructions are shown as `.insn` (and their aliases
are not used), use a `Decoder`:

```rust
use disc_v::{Decoder, Extension, Isa};

// Presets.
let dec = Decoder::RVA23U64;
let dec = Decoder::RV64GC;

// From an ISA string, as passed to compilers with -march.
let dec = Decoder::from_march("rv32imac_zicsr_zba_zbb").unwrap();

// By adding and removing extensions. Implied extensions follow: D implies F,
// and removing F removes D.
let dec = Decoder::RV64GC.with(Extension::Zicond).unwrap().without(Extension::D);
let dec = Decoder::with_only(Isa::Rv32, [Extension::M, Extension::C]).unwrap();

for ins in dec.disassemble(0x1000, &[0x05, 0x45]) {
    println!("{ins}");
}
```

Some extensions give the same encodings different meanings, so they cannot
be enabled together; the defaults leave out the one that conflicts. For
example, Zcmp and Zcmt (compressed push/pop and table jumps, common in
size-optimized embedded code) reuse the encodings of Zcd, the compressed
double-precision loads and stores that C includes when D is enabled. `with`
returns an error for a conflicting extension, and `try_with` leaves the
decoder unchanged instead:

```rust
use disc_v::{Decoder, Extension};

assert!(Decoder::RV32GC.with(Extension::Zcmp).is_err());
assert_eq!(Decoder::RV32GC.try_with(Extension::Zcmp), Decoder::RV32GC);

// Removing Zcd keeps the other compressed instructions (Zca, Zcf).
let dec = Decoder::RV32GC.without(Extension::Zcd).with(Extension::Zcmp).unwrap();
assert_eq!(dec.decode(0, 0xb862).to_string(), "cm.push {ra,s0-s1},-16");

let dec = Decoder::from_march("rv32imac_zcmp_zcmt").unwrap();
```

RV32E and RV64E, which have only registers x0-x15, are the E extension:
`Decoder::from_march("rv32emc")`, or `with(Extension::E)`. Instructions that
use x16-x31 are then shown as `.insn`.

Zfinx, Zdinx, Zhinx and Zhinxmin keep floating-point values in the integer
registers. They use F's encodings, so they conflict with F:
`Decoder::from_march("rv32imc_zicsr_zdinx")` decodes `fadd.d a0,a0,a2`
rather than `fadd.d fa0,fa0,fa2`.

## Output format

The text is what `objdump -D -b binary` prints for the same bytes: the same
mnemonics, aliases (`li`, `ret`, `csrr`, ...) and operand syntax. Immediates
are decimal (`li a0,-1107`), while shift amounts and upper immediates are
hex (`slli a0,a0,0x3`, `lui a0,0xbad`).

Branch and jump targets are absolute addresses, always written with `0x`:
`beq a0,a1,0x1008`. When objdump disassembles an ELF file it instead writes
the target as bare hex followed by a symbol, as in `beq a0,a1,1008 <foo+0x8>`.
disc-v has no symbol information, so it uses objdump's raw-binary form,
which is unambiguous.

A few encodings are decoded differently from objdump where binutils and the
ISA specification disagree; see
[Differences from the specification](#differences-from-the-specification).

## Supported instructions

- RV32I, RV64I and RV128I; RV32E and RV64E (as the E extension)
- M, A, F, D, Q and C
- Zicsr and Zifencei
- Zba, Zbb, Zbc and Zbs
- Zicond
- Zawrs
- Zicbom, Zicboz and Zicbop; Zihintntl and Zihintpause
- Zimop and Zcmop, with the Zicfiss and Zicfilp instructions
- Zcb
- Zfh and Zfhmin
- Zfinx, Zdinx, Zhinx and Zhinxmin, which conflict with F and so are not
  decoded by default
- Zfa
- V (vectors)
- Zvbb and Zvbc; Zvkg, Zvkned, Zvknha, Zvknhb, Zvksed and Zvksh
- H (hypervisor)
- Zalasr, Zacas and Zabha
- Zfbfmin, Zvfbfmin and Zvfbfwma (BFloat16)
- Zbkb, Zbkc and Zbkx; Zknd, Zkne, Zknh, Zksed and Zksh (scalar cryptography)
- Zca, Zcf and Zcd (the parts of C); Zcmp and Zcmt, which conflict with Zcd and
  so are not decoded by default
- Zilsd (RV32 load and store pairs); Zclsd, which conflicts with Zcf and so is
  not decoded by default
- `ecall`, `ebreak`, `sret`, `mret`, `dret`, `wfi` and `sfence.vma`; Svinval,
  Smrnmi (`mnret`) and Ssctr (`sctrclr`)

48- and 64-bit encodings are recognized (so disassembly stays in sync) but
shown as `.insn`, as objdump does.

## Testing

Besides unit tests, the decoder is checked against two references:

- `tests/riscv_opcodes.rs` checks it against the encodings in
  [riscv-opcodes](https://github.com/riscv/riscv-opcodes), in both
  directions: every listed instruction decodes to its name, and every
  encoding disc-v accepts is listed. The data is in
  `tests/data/riscv-opcodes.txt`, generated by `scripts/gen-riscv-opcodes.py`.
- `tests/objdump.rs` compares the output with GNU objdump (after rewriting
  objdump's `1008 <foo+0x8>` targets as `0x1008`) for every
  compressed encoding, random 32-bit encodings, and random encodings of every
  instruction in riscv-opcodes. It needs a RISC-V binutils
  (`riscv64-elf-objdump`, `riscv64-unknown-elf-objdump` or
  `riscv64-linux-gnu-objdump`) and is skipped if none is found; set
  `DISC_V_REQUIRE_BINUTILS=1` to make that an error, as CI does.

CSR names come from riscv-opcodes, via `scripts/gen-csr-names.py`, and so do
the vector instruction tables in `src/opcodes/v.rs`, via
`scripts/gen-vector-opcodes.py`, which encodes objdump's operand order. The
generators format their output with `rustfmt`, so generated code is formatted
and linted like the rest of the crate.

### Differences from the specification

Where binutils and the ISA specification disagree on whether an encoding is
valid, disc-v follows the specification, with the exceptions below. The
differences from binutils, such as RV32 shift amounts of 32 or more
(reserved, but decoded by objdump), are listed in `known_difference` in
`tests/objdump.rs`. Where riscv-opcodes disagrees with the specification,
disc-v follows the specification, and the differences are listed in
`NOT_IN_SPEC` in `tests/riscv_opcodes.rs`.

disc-v decodes a few encodings that the specification reserves, where they
have plausible uses:

- `aes64ks1i` with a round number above 10. The AES key schedule has uses
  besides AES itself, and objdump decodes these too.

## Origins

disc-v is built on the disassembler from the
[Caliptra MCU emulator](https://github.com/chipsalliance/caliptra-mcu-sw)
(`emulator/app/src/dis.rs`, imported at commit
`20f7fcff1b51d69b03d0b352685fc7360536ad9a`). The Caliptra authors ported it
to Rust and added the bit-manipulation extensions (Zba, Zbb, Zbc, Zbs).

That code is itself a port of Michael Clark and SiFive's C
[riscv-disassembler](https://github.com/michaeljclark/riscv-disassembler).

The name is a nod to Oxide Computer's
[disc-v](https://github.com/oxidecomputer/disc-v), an earlier Rust port of
the same C disassembler by Wladimir J. van der Laan and Adam H. Leventhal.
That repository has no license, so this crate does not use any of its code.

## License

Copyright (c) 2026 Christopher Swenson.

disc-v is licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE)).

It is derived from Michael Clark and SiFive's riscv-disassembler, which is
MIT licensed (Copyright (c) 2016-2017 Michael Clark, (c) 2017-2018 SiFive,
Inc.), by way of the Caliptra port, which is Apache-2.0 licensed. The
original MIT copyright and permission notice is kept in [NOTICE](NOTICE),
and source files derived from that code say so in their headers.

The test data and CSR names derived from riscv-opcodes are Copyright (c) 2022
RISC-V International, under the BSD-3-Clause license
([tests/data/LICENSE-riscv-opcodes](tests/data/LICENSE-riscv-opcodes)).
