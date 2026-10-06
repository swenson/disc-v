# disc-v

A RISC-V disassembler in Rust.

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

The original C disassembler is MIT licensed (Copyright (c) 2016-2017 Michael
Clark, (c) 2017-2018 SiFive, Inc.). The Caliptra port and its changes are
licensed under Apache-2.0. disc-v is distributed under both licenses: see
[LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
