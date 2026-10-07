// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Scalar cryptography: bit manipulation for cryptography (Zbkb, Zbkx; Zbkc
//! is Zbc's `clmul` and `clmulh`), AES (Zknd, Zkne), SHA-2 (Zknh), SM4
//! (Zksed) and SM3 (Zksh).
//!
//! Zbkb also provides several Zbb instructions, such as `rol` and `rev8`;
//! those are in [`b`](super::b).

use super::{Codec, Opcode, fmt};
use crate::Extension;

pub(crate) static PACK: Opcode =
    Opcode::new("pack", Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbkb]);
pub(crate) static PACKH: Opcode =
    Opcode::new("packh", Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbkb]);
pub(crate) static PACKW: Opcode = Opcode::new("packw", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zbkb])
    .rv64();
pub(crate) static BREV8: Opcode =
    Opcode::new("brev8", Codec::R, fmt::RD_RS1).requires(&[Extension::Zbkb]);
pub(crate) static ZIP: Opcode = Opcode::new("zip", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zbkb])
    .rv32_only();
pub(crate) static UNZIP: Opcode = Opcode::new("unzip", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zbkb])
    .rv32_only();
pub(crate) static XPERM4: Opcode =
    Opcode::new("xperm4", Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbkx]);
pub(crate) static XPERM8: Opcode =
    Opcode::new("xperm8", Codec::R, fmt::RD_RS1_RS2).requires(&[Extension::Zbkx]);

pub(crate) static AES32ESI: Opcode = Opcode::new("aes32esi", Codec::RBs, fmt::RD_RS1_RS2_BS)
    .requires(&[Extension::Zkne])
    .rv32_only();
pub(crate) static AES32ESMI: Opcode = Opcode::new("aes32esmi", Codec::RBs, fmt::RD_RS1_RS2_BS)
    .requires(&[Extension::Zkne])
    .rv32_only();
pub(crate) static AES32DSI: Opcode = Opcode::new("aes32dsi", Codec::RBs, fmt::RD_RS1_RS2_BS)
    .requires(&[Extension::Zknd])
    .rv32_only();
pub(crate) static AES32DSMI: Opcode = Opcode::new("aes32dsmi", Codec::RBs, fmt::RD_RS1_RS2_BS)
    .requires(&[Extension::Zknd])
    .rv32_only();
pub(crate) static AES64ES: Opcode = Opcode::new("aes64es", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zkne])
    .rv64_only();
pub(crate) static AES64ESM: Opcode = Opcode::new("aes64esm", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zkne])
    .rv64_only();
pub(crate) static AES64DS: Opcode = Opcode::new("aes64ds", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknd])
    .rv64_only();
pub(crate) static AES64DSM: Opcode = Opcode::new("aes64dsm", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknd])
    .rv64_only();
pub(crate) static AES64IM: Opcode = Opcode::new("aes64im", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zknd])
    .rv64_only();
/// The specification reserves round numbers above 10, but they are decoded
/// (as objdump does), since the key schedule has uses besides AES itself.
pub(crate) static AES64KS1I: Opcode = Opcode::new("aes64ks1i", Codec::Rnum, fmt::RD_RS1_SHAMT)
    .requires_any(&[Extension::Zknd, Extension::Zkne])
    .rv64_only();
pub(crate) static AES64KS2: Opcode = Opcode::new("aes64ks2", Codec::R, fmt::RD_RS1_RS2)
    .requires_any(&[Extension::Zknd, Extension::Zkne])
    .rv64_only();

pub(crate) static SHA256SUM0: Opcode =
    Opcode::new("sha256sum0", Codec::R, fmt::RD_RS1).requires(&[Extension::Zknh]);
pub(crate) static SHA256SUM1: Opcode =
    Opcode::new("sha256sum1", Codec::R, fmt::RD_RS1).requires(&[Extension::Zknh]);
pub(crate) static SHA256SIG0: Opcode =
    Opcode::new("sha256sig0", Codec::R, fmt::RD_RS1).requires(&[Extension::Zknh]);
pub(crate) static SHA256SIG1: Opcode =
    Opcode::new("sha256sig1", Codec::R, fmt::RD_RS1).requires(&[Extension::Zknh]);
pub(crate) static SHA512SUM0R: Opcode = Opcode::new("sha512sum0r", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknh])
    .rv32_only();
pub(crate) static SHA512SUM1R: Opcode = Opcode::new("sha512sum1r", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknh])
    .rv32_only();
pub(crate) static SHA512SIG0L: Opcode = Opcode::new("sha512sig0l", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknh])
    .rv32_only();
pub(crate) static SHA512SIG0H: Opcode = Opcode::new("sha512sig0h", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknh])
    .rv32_only();
pub(crate) static SHA512SIG1L: Opcode = Opcode::new("sha512sig1l", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknh])
    .rv32_only();
pub(crate) static SHA512SIG1H: Opcode = Opcode::new("sha512sig1h", Codec::R, fmt::RD_RS1_RS2)
    .requires(&[Extension::Zknh])
    .rv32_only();
pub(crate) static SHA512SUM0: Opcode = Opcode::new("sha512sum0", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zknh])
    .rv64_only();
pub(crate) static SHA512SUM1: Opcode = Opcode::new("sha512sum1", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zknh])
    .rv64_only();
pub(crate) static SHA512SIG0: Opcode = Opcode::new("sha512sig0", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zknh])
    .rv64_only();
pub(crate) static SHA512SIG1: Opcode = Opcode::new("sha512sig1", Codec::R, fmt::RD_RS1)
    .requires(&[Extension::Zknh])
    .rv64_only();

pub(crate) static SM4ED: Opcode =
    Opcode::new("sm4ed", Codec::RBs, fmt::RD_RS1_RS2_BS).requires(&[Extension::Zksed]);
pub(crate) static SM4KS: Opcode =
    Opcode::new("sm4ks", Codec::RBs, fmt::RD_RS1_RS2_BS).requires(&[Extension::Zksed]);
pub(crate) static SM3P0: Opcode =
    Opcode::new("sm3p0", Codec::R, fmt::RD_RS1).requires(&[Extension::Zksh]);
pub(crate) static SM3P1: Opcode =
    Opcode::new("sm3p1", Codec::R, fmt::RD_RS1).requires(&[Extension::Zksh]);
