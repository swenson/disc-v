// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! A decoder for a chosen base ISA and set of extensions.

use core::fmt;

use crate::{Disassembler, Extension, Extensions, Instruction, Isa, inst_length};

/// Pairs of extensions that give different meanings to the same encodings,
/// and so cannot be decoded together.
const CONFLICTS: &[(Extension, Extension)] = &[
    // Zcmp and Zcmt use the encodings of c.fsdsp.
    (Extension::Zcmp, Extension::Zcd),
    (Extension::Zcmt, Extension::Zcd),
    // Zclsd uses the encodings of c.flw, c.fsw, c.flwsp and c.fswsp.
    (Extension::Zclsd, Extension::Zcf),
    // Zfinx (and so Zdinx, Zhinx and Zhinxmin) uses F's encodings with
    // integer registers.
    (Extension::Zfinx, Extension::F),
];

/// Decodes instructions for a base ISA with a chosen set of extensions.
///
/// [`Decoder::new`] enables every supported extension that does not
/// conflict with another ([`Extensions::DEFAULT`]), as the free functions
/// such as [`decode`](fn@crate::decode) do. Presets such as
/// [`Decoder::RVA23U64`], [`Decoder::with_only`] and
/// [`Decoder::from_march`] choose the extensions of a particular target, so
/// that instructions from other extensions are shown as `.insn` (or without
/// aliases, such as `lpad`, that need them).
///
/// ```
/// use disc_v::{Decoder, Extension, Isa};
///
/// // czero.eqz a0,a1,a2
/// assert_eq!(Decoder::new(Isa::Rv64).decode(0, 0x0ec5d533).to_string(), "czero.eqz a0,a1,a2");
/// assert_eq!(Decoder::RV64GC.decode(0, 0x0ec5d533).to_string(), ".insn 4, 0x0ec5d533");
///
/// let dec = Decoder::RV64GC.with(Extension::Zicond).unwrap();
/// assert_eq!(dec.decode(0, 0x0ec5d533).to_string(), "czero.eqz a0,a1,a2");
///
/// let dec = Decoder::from_march("rv64gc_zicond").unwrap();
/// assert_eq!(dec, Decoder::RV64GC.with(Extension::Zicond).unwrap());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Decoder {
    isa: Isa,
    extensions: Extensions,
}

impl Decoder {
    /// RV32GC: RV32I with IMAFD, Zicsr, Zifencei and C.
    pub const RV32GC: Decoder = Decoder::unchecked(Isa::Rv32, Extensions::GC);

    /// RV64GC: RV64I with IMAFD, Zicsr, Zifencei and C.
    pub const RV64GC: Decoder = Decoder::unchecked(Isa::Rv64, Extensions::GC);

    /// The RVA23U64 profile, for user-mode code.
    pub const RVA23U64: Decoder = Decoder::unchecked(Isa::Rv64, Extensions::RVA23U64);

    /// The RVA23S64 profile, which adds the supervisor-mode and hypervisor
    /// instructions to RVA23U64.
    pub const RVA23S64: Decoder = Decoder::unchecked(Isa::Rv64, Extensions::RVA23S64);

    /// A decoder for `isa` with every supported extension that does not
    /// conflict with another ([`Extensions::DEFAULT`]).
    pub const fn new(isa: Isa) -> Decoder {
        Decoder::unchecked(isa, Extensions::DEFAULT)
    }

    const fn unchecked(isa: Isa, extensions: Extensions) -> Decoder {
        Decoder { isa, extensions }
    }

    /// A decoder for `isa` with exactly `extensions` (and the extensions
    /// they imply), or an error if two of them conflict.
    ///
    /// ```
    /// use disc_v::{Decoder, Extension, Isa};
    ///
    /// let dec = Decoder::with_only(Isa::Rv32, [Extension::M, Extension::C]).unwrap();
    /// assert_eq!(dec.decode(0, 0x02b50533).to_string(), "mul a0,a0,a1");
    /// assert_eq!(dec.decode(0, 0x00b57553).to_string(), ".insn 4, 0x00b57553"); // fadd.s
    /// ```
    pub fn with_only(isa: Isa, extensions: impl Into<Extensions>) -> Result<Decoder, Conflict> {
        let extensions = extensions.into();
        match find_conflict(extensions) {
            Some(conflict) => Err(conflict),
            None => Ok(Decoder { isa, extensions }),
        }
    }

    /// The decoder with `ext` (and the extensions it implies) enabled, or an
    /// error if it conflicts with an extension already enabled.
    pub fn with(self, ext: Extension) -> Result<Decoder, Conflict> {
        Decoder::with_only(self.isa, self.extensions.with(ext))
    }

    /// The decoder with `ext` (and the extensions it implies) enabled, or
    /// the decoder unchanged if `ext` conflicts with an extension already
    /// enabled.
    pub fn try_with(self, ext: Extension) -> Decoder {
        self.with(ext).unwrap_or(self)
    }

    /// The decoder with `ext` (and the extensions that imply it) disabled.
    pub fn without(self, ext: Extension) -> Decoder {
        Decoder {
            extensions: self.extensions.without(ext),
            ..self
        }
    }

    /// The base ISA.
    pub fn isa(&self) -> Isa {
        self.isa
    }

    /// The enabled extensions.
    pub fn extensions(&self) -> Extensions {
        self.extensions
    }

    /// Decodes the instruction `inst`, located at address `pc`. See
    /// [`decode`](fn@crate::decode).
    pub fn decode(&self, pc: u64, inst: u64) -> Instruction {
        crate::decode::decode(self.isa, self.extensions, pc, inst)
    }

    /// Decodes the instruction at the start of `bytes`, located at address
    /// `pc`. See [`decode_bytes`](crate::decode_bytes).
    pub fn decode_bytes(&self, pc: u64, bytes: &[u8]) -> Option<Instruction> {
        let first = u16::from_le_bytes([*bytes.first()?, *bytes.get(1)?]);
        let len = inst_length(first).unwrap_or(2);
        let mut word = [0; 8];
        word[..len].copy_from_slice(bytes.get(..len)?);
        Some(self.decode(pc, u64::from_le_bytes(word)))
    }

    /// Disassembles `bytes`, the first of which is at address `pc`. See
    /// [`disassemble`](crate::disassemble).
    pub fn disassemble<'a>(&self, pc: u64, bytes: &'a [u8]) -> Disassembler<'a> {
        Disassembler {
            decoder: *self,
            pc,
            bytes,
        }
    }

    /// A decoder for an ISA string, as passed to compilers with `-march`,
    /// such as `"rv64gc"` or `"rv32imac_zicsr_zba_zbb"`.
    ///
    /// Version numbers (`rv64i2p1`) are ignored, as are extensions that
    /// define no instructions (such as Zkt, Sstc or Zvl128b). Some
    /// extensions are decoded as a larger extension that contains them:
    /// Zmmul as M, Zaamo and Zalrsc as A, the Zve embedded vector extensions
    /// as V, and Sha as H. The Zce, Zk, Zkn and Zks bundles are
    /// expanded.
    ///
    /// Unknown supervisor- and machine-level extensions (names starting
    /// with `s`) are also ignored, since nearly all of them define no
    /// instructions, while unknown `z` and `x` extensions are errors.
    ///
    /// ```
    /// use disc_v::{Decoder, MarchError};
    ///
    /// assert_eq!(Decoder::from_march("rv64gc"), Ok(Decoder::RV64GC));
    /// assert_eq!(
    ///     Decoder::from_march("rv64gc_xtheadba"),
    ///     Err(MarchError::UnsupportedExtension("xtheadba")),
    /// );
    /// ```
    pub fn from_march(march: &str) -> Result<Decoder, MarchError<'_>> {
        let (isa, rest) = [
            ("rv32", Isa::Rv32),
            ("rv64", Isa::Rv64),
            ("rv128", Isa::Rv128),
        ]
        .into_iter()
        .find_map(|(prefix, isa)| {
            let head = march.get(..prefix.len())?;
            head.eq_ignore_ascii_case(prefix)
                .then(|| (isa, &march[prefix.len()..]))
        })
        .ok_or(MarchError::InvalidBase)?;
        let mut extensions = Extensions::EMPTY;
        for (i, token) in rest.split('_').enumerate() {
            let first = token.chars().next().map(|c| c.to_ascii_lowercase());
            match first {
                Some('z' | 's' | 'x') if i > 0 => {
                    extensions = extensions.union(multi_letter(token)?);
                }
                _ => {
                    if i == 0 && !matches!(first, Some('i' | 'g' | 'e')) {
                        return Err(MarchError::InvalidBase);
                    }
                    extensions = extensions.union(single_letters(token)?);
                }
            }
        }
        Decoder::with_only(isa, extensions).map_err(MarchError::Conflict)
    }
}

/// The extensions of a run of single-letter extensions, such as `imafdc`.
fn single_letters(token: &str) -> Result<Extensions, MarchError<'_>> {
    use Extension::*;
    let mut extensions = Extensions::EMPTY;
    let mut rest = token;
    while let Some(c) = rest.chars().next() {
        let (name, after) = rest.split_at(c.len_utf8());
        rest = strip_version(after);
        extensions = extensions.union(match c.to_ascii_lowercase() {
            'i' => Extensions::EMPTY,
            // The RV32E and RV64E bases.
            'e' => E.into(),
            'g' => Extensions::of(&[M, A, F, D, Zicsr, Zifencei]),
            'b' => Extensions::of(&[Zba, Zbb, Zbs]),
            c => match Extension::from_name(name) {
                Some(ext) if c.is_ascii_alphabetic() => ext.into(),
                _ => return Err(MarchError::UnsupportedExtension(name)),
            },
        });
    }
    Ok(extensions)
}

/// The extensions of a multi-letter extension such as `zba2p0`.
fn multi_letter(token: &str) -> Result<Extensions, MarchError<'_>> {
    use Extension::*;
    let name = token.strip_suffix(trailing_version(token)).unwrap_or(token);
    if let Some(ext) = Extension::from_name(name) {
        return Ok(ext.into());
    }
    let lower = |s: &str| name.eq_ignore_ascii_case(s);
    let has_prefix = |p: &str| {
        name.get(..p.len())
            .is_some_and(|h| h.eq_ignore_ascii_case(p))
    };
    Ok(if lower("zmmul") {
        M.into()
    } else if lower("zaamo") || lower("zalrsc") {
        A.into()
    } else if lower("zce") {
        Extensions::of(&[Zca, Zcb, Zcmp, Zcmt])
    } else if has_prefix("zve") {
        V.into()
    } else if lower("zkn") {
        Extensions::of(&[Zbkb, Zbkc, Zbkx, Zkne, Zknd, Zknh])
    } else if lower("zks") {
        Extensions::of(&[Zbkb, Zbkc, Zbkx, Zksed, Zksh])
    } else if lower("zk") {
        Extensions::of(&[Zbkb, Zbkc, Zbkx, Zkne, Zknd, Zknh])
    } else if lower("sha") {
        H.into()
    } else if lower("smctr") {
        Ssctr.into()
    } else if has_prefix("zvl") || NO_INSTRUCTIONS.iter().any(|n| lower(n)) {
        Extensions::EMPTY
    } else if has_prefix("s") {
        // Other supervisor- and machine-level extensions define no
        // instructions.
        Extensions::EMPTY
    } else {
        return Err(MarchError::UnsupportedExtension(name));
    })
}

/// Unprivileged extensions that define no instructions.
const NO_INSTRUCTIONS: &[&str] = &[
    "zicntr", "zihpm", "zkt", "zvkt", "zvfh", "zvfhmin", "ztso", "za64rs", "za128rs", "zama16b",
    "zic64b", "ziccamoa", "ziccif", "zicclsm", "ziccrse", "supm", "zkr",
];

/// `s` without the version number at its start, such as `2p1` in `2p1m`.
fn strip_version(s: &str) -> &str {
    &s[leading_version(s).len()..]
}

/// The version number at the start of `s`, in a run of single-letter
/// extensions: digits, optionally followed by `p` and more digits.
fn leading_version(s: &str) -> &str {
    let major = s.bytes().take_while(u8::is_ascii_digit).count();
    if major == 0 {
        return "";
    }
    let minor = match &s.as_bytes()[major..] {
        [b'p' | b'P', d, rest @ ..] if d.is_ascii_digit() => {
            2 + rest.iter().take_while(|b| b.is_ascii_digit()).count()
        }
        _ => 0,
    };
    &s[..major + minor]
}

/// The version number at the end of a multi-letter extension name, such as
/// `1p0` in `zba1p0`.
fn trailing_version(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut end = bytes.len();
    while end > 0 && bytes[end - 1].is_ascii_digit() {
        end -= 1;
    }
    if end == bytes.len() {
        return "";
    }
    if end > 1 && matches!(bytes[end - 1], b'p' | b'P') && bytes[end - 2].is_ascii_digit() {
        end -= 1;
        while end > 0 && bytes[end - 1].is_ascii_digit() {
            end -= 1;
        }
    }
    &s[end..]
}

/// Finds a pair of conflicting extensions in `extensions`.
fn find_conflict(extensions: Extensions) -> Option<Conflict> {
    CONFLICTS
        .iter()
        .find(|(a, b)| extensions.contains(*a) && extensions.contains(*b))
        .map(|&(extension, conflicts_with)| Conflict {
            extension,
            conflicts_with,
        })
}

/// Two extensions that give different meanings to the same encodings, and
/// so cannot be decoded together.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Conflict {
    extension: Extension,
    conflicts_with: Extension,
}

impl Conflict {
    /// One of the conflicting extensions.
    pub fn extension(&self) -> Extension {
        self.extension
    }

    /// The extension it conflicts with.
    pub fn conflicts_with(&self) -> Extension {
        self.conflicts_with
    }
}

impl fmt::Display for Conflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} conflicts with {}",
            self.extension, self.conflicts_with
        )
    }
}

impl core::error::Error for Conflict {}

/// An error parsing an ISA string with [`Decoder::from_march`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarchError<'a> {
    /// The string does not start with `rv32`, `rv64` or `rv128` followed by
    /// `i`, `e` or `g`.
    InvalidBase,
    /// The string names an extension that disc-v does not support.
    UnsupportedExtension(&'a str),
    /// The string names two extensions that conflict.
    Conflict(Conflict),
}

impl fmt::Display for MarchError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarchError::InvalidBase => {
                f.write_str("ISA string does not start with rv32, rv64 or rv128 and i, e or g")
            }
            MarchError::UnsupportedExtension(name) => write!(f, "unsupported extension {name}"),
            MarchError::Conflict(conflict) => conflict.fmt(f),
        }
    }
}

impl core::error::Error for MarchError<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_suffixes() {
        assert_eq!(leading_version("2p1m"), "2p1");
        assert_eq!(leading_version("2m"), "2");
        assert_eq!(leading_version("2pm"), "2");
        assert_eq!(leading_version("mc2"), "");
        assert_eq!(strip_version("2p1mc"), "mc");
        assert_eq!(strip_version("mc2"), "mc2");
        assert_eq!(trailing_version("zba1p0"), "1p0");
        assert_eq!(trailing_version("zba1"), "1");
        assert_eq!(trailing_version("zba"), "");
        assert_eq!(trailing_version("zvl128b"), "");
    }
}
