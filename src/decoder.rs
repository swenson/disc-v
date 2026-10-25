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
    // CORE-V and T-Head use the same custom-0 encodings.
    (Extension::XCVmem, Extension::XTheadBa),
    (Extension::XCVmem, Extension::XTheadBb),
    (Extension::XCVmem, Extension::XTheadBs),
    (Extension::XCVmem, Extension::XTheadCmo),
    (Extension::XCVmem, Extension::XTheadCondMov),
    (Extension::XCVmem, Extension::XTheadFmv),
    (Extension::XCVmem, Extension::XTheadInt),
    (Extension::XCVmem, Extension::XTheadMac),
    (Extension::XCVmem, Extension::XTheadMemIdx),
    (Extension::XCVmem, Extension::XTheadMemPair),
    (Extension::XCVmem, Extension::XTheadSync),
    (Extension::XCVbi, Extension::XTheadFMemIdx),
    (Extension::XCVelw, Extension::XTheadBb),
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
    ///     Decoder::from_march("rv64gc_xventanacondops"),
    ///     Err(MarchError::UnsupportedExtension("xventanacondops")),
    /// );
    /// ```
    pub fn from_march(march: &str) -> Result<Decoder, MarchError<'_>> {
        let mut first_unsupported = None;
        let (isa, extensions) = parse_march(march, &mut |name| {
            first_unsupported.get_or_insert(name);
        })?;
        if let Some(name) = first_unsupported {
            return Err(MarchError::UnsupportedExtension(name));
        }
        Decoder::with_only(isa, extensions).map_err(MarchError::Conflict)
    }

    /// A decoder for the extensions of an ISA string that disc-v supports,
    /// calling `unsupported` with the name of each extension it names that
    /// disc-v does not, instead of refusing the string as
    /// [`from_march`](Self::from_march) does.
    ///
    /// The string is read as [`from_march`](Self::from_march) reads it; the
    /// unsupported extensions are left out, so the decoder decodes everything
    /// else the string declares. A string that names no valid base, or two
    /// extensions that conflict, is still an error.
    ///
    /// ```
    /// use disc_v::Decoder;
    ///
    /// let mut unsupported = Vec::new();
    /// let dec = Decoder::from_march_reporting("rv64gc_xventanacondops_zba", |name| {
    ///     unsupported.push(name)
    /// })
    /// .unwrap();
    /// assert_eq!(unsupported, ["xventanacondops"]);
    /// assert_eq!(dec, Decoder::RV64GC.with(disc_v::Extension::Zba).unwrap());
    /// ```
    pub fn from_march_reporting<'a>(
        march: &'a str,
        mut unsupported: impl FnMut(&'a str),
    ) -> Result<Decoder, MarchError<'a>> {
        let (isa, extensions) = parse_march(march, &mut unsupported)?;
        Decoder::with_only(isa, extensions).map_err(MarchError::Conflict)
    }
}

/// The base ISA and the extensions an ISA string names, calling `unsupported`
/// with each extension name disc-v does not support.
fn parse_march<'a>(
    march: &'a str,
    unsupported: &mut dyn FnMut(&'a str),
) -> Result<(Isa, Extensions), MarchError<'a>> {
    let (isa, rest) = [
        ("rv32", Isa::Rv32),
        ("rv64", Isa::Rv64),
        ("rv128", Isa::Rv128),
    ]
    .into_iter()
    .find_map(|(prefix, isa)| {
        let (head, rest) = march.split_at_checked(prefix.len())?;
        head.eq_ignore_ascii_case(prefix).then_some((isa, rest))
    })
    .ok_or(MarchError::InvalidBase)?;
    let mut extensions = Extensions::EMPTY;
    // Not split('_'): its searcher has panic paths that the optimizer
    // does not always remove (see panic-check/).
    for (i, token) in rest.split(['_']).enumerate() {
        let first = token.chars().next().map(|c| c.to_ascii_lowercase());
        match first {
            Some('z' | 's' | 'x') if i > 0 => {
                extensions = extensions.union(multi_letter(token, unsupported));
            }
            _ => {
                if i == 0 && !matches!(first, Some('i' | 'g' | 'e')) {
                    return Err(MarchError::InvalidBase);
                }
                extensions = extensions.union(single_letters(token, unsupported));
            }
        }
    }
    Ok((isa, extensions))
}

/// The extensions of a run of single-letter extensions, such as `imafdc`,
/// calling `unsupported` with each letter disc-v does not support.
fn single_letters<'a>(token: &'a str, unsupported: &mut dyn FnMut(&'a str)) -> Extensions {
    use Extension::*;
    let mut extensions = Extensions::EMPTY;
    let mut rest = token;
    while let Some(c) = rest.chars().next() {
        let Some((name, after)) = rest.split_at_checked(c.len_utf8()) else {
            break;
        };
        rest = strip_version(after);
        extensions = extensions.union(match c.to_ascii_lowercase() {
            'i' => Extensions::EMPTY,
            // The RV32E and RV64E bases.
            'e' => E.into(),
            'g' => Extensions::of(&[M, A, F, D, Zicsr, Zifencei]),
            'b' => Extensions::of(&[Zba, Zbb, Zbs]),
            c => match Extension::from_name(name) {
                Some(ext) if c.is_ascii_alphabetic() => ext.into(),
                _ => {
                    unsupported(name);
                    Extensions::EMPTY
                }
            },
        });
    }
    extensions
}

/// The extensions of a multi-letter extension such as `zba2p0`, or none
/// after calling `unsupported` with its name when disc-v does not support it.
fn multi_letter<'a>(token: &'a str, unsupported: &mut dyn FnMut(&'a str)) -> Extensions {
    use Extension::*;
    let name = token.strip_suffix(trailing_version(token)).unwrap_or(token);
    if let Some(ext) = Extension::from_name(name) {
        return ext.into();
    }
    let lower = |s: &str| name.eq_ignore_ascii_case(s);
    let has_prefix = |p: &str| {
        name.get(..p.len())
            .is_some_and(|h| h.eq_ignore_ascii_case(p))
    };
    if lower("zmmul") {
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
        unsupported(name);
        Extensions::EMPTY
    }
}

/// Unprivileged extensions that define no instructions.
const NO_INSTRUCTIONS: &[&str] = &[
    "zicntr", "zihpm", "zkt", "zvkt", "zvfh", "zvfhmin", "ztso", "za64rs", "za128rs", "zama16b",
    "zic64b", "ziccamoa", "ziccif", "zicclsm", "ziccrse", "supm", "zkr",
];

/// `s` without the version number at its start, such as `2p1` in `2p1m`.
fn strip_version(s: &str) -> &str {
    s.strip_prefix(leading_version(s)).unwrap_or(s)
}

/// The version number at the start of `s`, in a run of single-letter
/// extensions: digits, optionally followed by `p` and more digits.
fn leading_version(s: &str) -> &str {
    fn skip_digits(s: &str) -> &str {
        s.trim_start_matches(|c: char| c.is_ascii_digit())
    }
    let after_major = skip_digits(s);
    if after_major.len() == s.len() {
        return "";
    }
    let after = match after_major.strip_prefix(['p', 'P']) {
        Some(minor) if minor.starts_with(|c: char| c.is_ascii_digit()) => skip_digits(minor),
        _ => after_major,
    };
    s.strip_suffix(after).unwrap_or("")
}

/// The version number at the end of a multi-letter extension name, such as
/// `1p0` in `zba1p0`.
fn trailing_version(s: &str) -> &str {
    fn skip_digits(s: &str) -> &str {
        s.trim_end_matches(|c: char| c.is_ascii_digit())
    }
    let before_minor = skip_digits(s);
    if before_minor.len() == s.len() {
        return "";
    }
    let before = match before_minor.strip_suffix(['p', 'P']) {
        Some(major) if major.ends_with(|c: char| c.is_ascii_digit()) => skip_digits(major),
        _ => before_minor,
    };
    s.strip_prefix(before).unwrap_or("")
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

    /// The conflicts between vendor extensions are exactly the pairs whose
    /// encodings overlap in the same base ISA, and no extension's encodings
    /// overlap each other.
    #[test]
    #[cfg(all(feature = "vendor-thead", feature = "vendor-corev"))]
    fn vendor_conflicts_are_the_overlapping_encodings() {
        use crate::opcodes::{MaskedOpcode, xcv, xthead};
        let tables: [&[MaskedOpcode]; 2] = [&xthead::CUSTOM_0, &xcv::CUSTOM];
        let entries = || tables.into_iter().flatten();
        let vendor = |e: &MaskedOpcode| *e.op.requires.iter().find(|x| x.is_vendor()).unwrap();
        let conflicts =
            |a: Extension, b: Extension| CONFLICTS.contains(&(a, b)) || CONFLICTS.contains(&(b, a));
        for (i, a) in entries().enumerate() {
            for b in entries().skip(i + 1) {
                let overlap =
                    (a.bits ^ b.bits) & a.mask & b.mask == 0 && a.op.isas & b.op.isas != 0;
                if !overlap {
                    continue;
                }
                let (va, vb) = (vendor(a), vendor(b));
                assert_ne!(va, vb, "{} and {} overlap", a.op.mnemonic, b.op.mnemonic);
                assert!(
                    conflicts(va, vb),
                    "{} and {} overlap",
                    a.op.mnemonic,
                    b.op.mnemonic
                );
            }
        }
        for &(a, b) in CONFLICTS.iter().filter(|(a, _)| a.is_vendor()) {
            assert!(
                entries().any(|x| vendor(x) == a
                    && entries().any(|y| vendor(y) == b
                        && (x.bits ^ y.bits) & x.mask & y.mask == 0
                        && x.op.isas & y.op.isas != 0)),
                "{a} and {b} do not overlap"
            );
        }
    }
}
