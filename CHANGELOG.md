# Changelog

All notable changes to this project are documented in this file. The format
is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Zicond (`czero.eqz`, `czero.nez`).
- Zawrs (`wrs.nto`, `wrs.sto`).
- Zicbom, Zicboz and Zicbop (`cbo.*`, and `prefetch.*` for the `ori` hints).
- Zihintntl (`ntl.*`, for the `add` and `c.add` hints) and Zihintpause
  (`pause`).

### Changed

- On RV32 and RV64, the MISC-MEM encodings that were RV128's `lq` are now
  decoded as `cbo.*` where valid.

## [0.1.0]

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

[Unreleased]: https://github.com/swenson/disc-v/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/swenson/disc-v/releases/tag/v0.1.0
