# Changelog

All notable changes to this project are documented in this file. The format
is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Decoder`, which decodes for a chosen base ISA and set of extensions, so
  that instructions (and aliases) of other extensions are not decoded. It has
  presets (`RV32GC`, `RV64GC`, `RVA23U64`, `RVA23S64`), `with_only`, `with`,
  `try_with` and `without`, and `from_march` for ISA strings such as
  `rv64gc_zba`. The free functions are unchanged and decode every supported
  extension, as before.
- Zalasr (`lb.aq`, ..., `sd.rl`), Zacas (`amocas.w`, `.d`, `.q`) and Zabha
  (byte and halfword atomics, and `amocas.b`/`.h`).
- BFloat16: Zfbfmin (`fcvt.bf16.s`, `fcvt.s.bf16`), Zvfbfmin and Zvfbfwma.
- Scalar cryptography: Zbkb, Zbkc, Zbkx, Zknd, Zkne, Zknh, Zksed and Zksh.
  `Decoder::from_march` expands the Zk, Zkn and Zks bundles. `aes64ks1i`
  is decoded with round numbers above 10, which the specification
  reserves, as objdump does.
- Zcmp (`cm.push`, `cm.pop`, `cm.popret`, `cm.popretz`, `cm.mvsa01`,
  `cm.mva01s`) and Zcmt (`cm.jt`, `cm.jalt`). They conflict with Zcd, so
  they are not in the defaults; enable them with a `Decoder`, for example
  `Decoder::from_march("rv32imac_zcmp_zcmt")`.
- RV32E and RV64E, as the E extension: instructions that use x16-x31 are
  illegal. `Decoder::from_march` accepts `rv32e` and `rv64e`.
- Zilsd (`ld` and `sd` on RV32, with even-odd register pairs), in the
  defaults, and Zclsd (`c.ld`, `c.sd`, `c.ldsp`, `c.sdsp` on RV32), which
  conflicts with Zcf and so is not.
- Zfinx, Zdinx, Zhinx and Zhinxmin: floating point in the integer registers
  (`fadd.s a0,a0,a1`), with even-odd register pairs for double precision on
  RV32. They conflict with F, so they are not in the defaults.
- The parts of C as extensions: Zca, Zcf and Zcd. C is a bundle of Zca, and
  Zcf and Zcd when F and D are enabled.
- `Extension` and `Extensions`, a set of extensions with presets (`DEFAULT`,
  `GC`, `RVA23U64`, `RVA23S64`) that follows implications between extensions.

### Changed

- disc-v's functions do not panic in optimized builds: the remaining
  bounds and string-boundary checks that could not be optimized away are
  gone, and `panic-check/` checks this in CI.

### Fixed

- `Decoder::from_march` no longer panics on ISA strings with non-ASCII
  characters after the base, and reads version numbers in a run of
  single-letter extensions correctly (`rv32imc2` is M and C, and
  `rv32imac2p0` is accepted).

## [0.1.1] - 2026-10-06

### Added

- Zicond (`czero.eqz`, `czero.nez`).
- Zawrs (`wrs.nto`, `wrs.sto`).
- Zicbom, Zicboz and Zicbop (`cbo.*`, and `prefetch.*` for the `ori` hints).
- Zihintntl (`ntl.*`, for the `add` and `c.add` hints) and Zihintpause
  (`pause`).
- Zimop and Zcmop (`mop.r.N`, `mop.rr.N`, `c.mop.N`), with the Zicfiss
  (`sspush`, `sspopchk`, `ssrdp`, `ssamoswap.w`/`.d`) and Zicfilp (`lpad`)
  instructions.
- Zcb (`c.lbu`, `c.lhu`, `c.lh`, `c.sb`, `c.sh`, `c.zext.*`, `c.sext.*`,
  `c.not`, `c.mul`).
- Zfh and Zfhmin (half-precision floating point).
- Zfa (`fli`, `fminm`, `fmaxm`, `fround`, `froundnx`, `fleq`, `fltq`,
  `fcvtmod.w.d`, `fmvh.x.*`, `fmvp.*.x`) for each floating-point format.
- V (vectors), including segment loads and stores and objdump's aliases
  (`vnot.v`, `vneg.v`, `vmmv.m`, ...). The tables are generated from
  riscv-opcodes by `scripts/gen-vector-opcodes.py`.
- Vector bit manipulation (Zvbb, Zvbc) and cryptography (Zvkg, Zvkned,
  Zvknha, Zvknhb, Zvksed, Zvksh).
- H (`hlv.*`, `hlvx.*`, `hsv.*`, `hfence.vvma`, `hfence.gvma`), Svinval
  (`sinval.vma`, `sfence.w.inval`, `sfence.inval.ir`, `hinval.*`), Smrnmi
  (`mnret`) and Ssctr (`sctrclr`).

### Changed

- On RV32 and RV64, the MISC-MEM encodings that were RV128's `lq` are now
  decoded as `cbo.*` where valid.
- `c.lui` with an odd destination below `x16` and a zero immediate, previously
  reserved, is decoded as `c.mop.N`.

## [0.1.0] - 2026-10-06

Initial release.

### Added

- Decoding of RV32, RV64 and RV128 instructions in the I, M, A, F, D, Q and C
  extensions, Zicsr, Zifencei, Zba, Zbb, Zbc and Zbs, with `decode`,
  `decode_bytes`, and the `disassemble` iterator over a byte slice.
- Output in GNU objdump's syntax, including its aliases;
  `Instruction::without_aliases` shows instructions as encoded, like
  `objdump -M no-aliases`.
- `no_std` support with no allocation; the minimum supported Rust version is
  1.85.1.
- Tests against the encodings in riscv-opcodes and against GNU objdump.

[Unreleased]: https://github.com/swenson/disc-v/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/swenson/disc-v/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/swenson/disc-v/releases/tag/v0.1.0
