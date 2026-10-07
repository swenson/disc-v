// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Extensions, and sets of them.

use core::fmt;

/// A RISC-V extension that defines instructions disc-v can decode.
///
/// Extensions that define no instructions, such as Zkt or Sstc, are not
/// listed; [`Decoder::from_march`](crate::Decoder::from_march) accepts and
/// ignores them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Extension {
    /// Integer multiplication and division.
    M,
    /// Atomic instructions.
    A,
    /// Single-precision floating point.
    F,
    /// Double-precision floating point. Implies F.
    D,
    /// Quad-precision floating point. Implies D.
    Q,
    /// Compressed instructions.
    C,
    /// Vectors.
    V,
    /// The hypervisor extension.
    H,
    /// Control and status register instructions.
    Zicsr,
    /// `fence.i`.
    Zifencei,
    /// Integer conditional operations (`czero.*`).
    Zicond,
    /// Wait-on-reservation-set (`wrs.*`).
    Zawrs,
    /// Cache-block management (`cbo.clean`, `cbo.flush`, `cbo.inval`).
    Zicbom,
    /// Cache-block zero (`cbo.zero`).
    Zicboz,
    /// Cache-block prefetch hints (`prefetch.*`).
    Zicbop,
    /// Non-temporal locality hints (`ntl.*`).
    Zihintntl,
    /// The `pause` hint.
    Zihintpause,
    /// May-be-operations (`mop.r.N`, `mop.rr.N`).
    Zimop,
    /// Compressed may-be-operations (`c.mop.N`). Implies C.
    Zcmop,
    /// Shadow stacks (`sspush`, `sspopchk`, `ssrdp`, `ssamoswap.*`).
    /// Implies Zimop.
    Zicfiss,
    /// Landing pads (`lpad`).
    Zicfilp,
    /// Address generation (`sh1add`, ...).
    Zba,
    /// Basic bit manipulation (`andn`, `clz`, ...).
    Zbb,
    /// Carry-less multiplication (`clmul`, ...).
    Zbc,
    /// Single-bit instructions (`bset`, ...).
    Zbs,
    /// Simple compressed instructions (`c.lbu`, `c.mul`, ...). Implies C.
    Zcb,
    /// Half-precision floating point. Implies Zfhmin.
    Zfh,
    /// Minimal half-precision floating point: loads, stores and
    /// conversions. Implies F.
    Zfhmin,
    /// Additional floating-point instructions (`fli`, `fround`, ...).
    /// Implies F.
    Zfa,
    /// Vector bit manipulation. Implies V.
    Zvbb,
    /// Vector carry-less multiplication. Implies V.
    Zvbc,
    /// Vector GCM/GMAC. Implies V.
    Zvkg,
    /// Vector AES. Implies V.
    Zvkned,
    /// Vector SHA-256. Implies V.
    Zvknha,
    /// Vector SHA-256 and SHA-512. Implies Zvknha, whose instructions it
    /// shares.
    Zvknhb,
    /// Vector SM4. Implies V.
    Zvksed,
    /// Vector SM3. Implies V.
    Zvksh,
    /// Fine-grained address-translation cache invalidation (`sinval.vma`,
    /// ...).
    Svinval,
    /// Debug mode (`dret`).
    Sdext,
    /// Resumable non-maskable interrupts (`mnret`).
    Smrnmi,
    /// Control-transfer records (`sctrclr`).
    Ssctr,
}

impl Extension {
    /// Every extension, in declaration order.
    const ALL: [Extension; 41] = {
        use Extension::*;
        [
            M,
            A,
            F,
            D,
            Q,
            C,
            V,
            H,
            Zicsr,
            Zifencei,
            Zicond,
            Zawrs,
            Zicbom,
            Zicboz,
            Zicbop,
            Zihintntl,
            Zihintpause,
            Zimop,
            Zcmop,
            Zicfiss,
            Zicfilp,
            Zba,
            Zbb,
            Zbc,
            Zbs,
            Zcb,
            Zfh,
            Zfhmin,
            Zfa,
            Zvbb,
            Zvbc,
            Zvkg,
            Zvkned,
            Zvknha,
            Zvknhb,
            Zvksed,
            Zvksh,
            Svinval,
            Sdext,
            Smrnmi,
            Ssctr,
        ]
    };

    /// The extension's name in an ISA string, such as `"zba"` or `"m"`.
    pub const fn name(self) -> &'static str {
        use Extension::*;
        match self {
            M => "m",
            A => "a",
            F => "f",
            D => "d",
            Q => "q",
            C => "c",
            V => "v",
            H => "h",
            Zicsr => "zicsr",
            Zifencei => "zifencei",
            Zicond => "zicond",
            Zawrs => "zawrs",
            Zicbom => "zicbom",
            Zicboz => "zicboz",
            Zicbop => "zicbop",
            Zihintntl => "zihintntl",
            Zihintpause => "zihintpause",
            Zimop => "zimop",
            Zcmop => "zcmop",
            Zicfiss => "zicfiss",
            Zicfilp => "zicfilp",
            Zba => "zba",
            Zbb => "zbb",
            Zbc => "zbc",
            Zbs => "zbs",
            Zcb => "zcb",
            Zfh => "zfh",
            Zfhmin => "zfhmin",
            Zfa => "zfa",
            Zvbb => "zvbb",
            Zvbc => "zvbc",
            Zvkg => "zvkg",
            Zvkned => "zvkned",
            Zvknha => "zvknha",
            Zvknhb => "zvknhb",
            Zvksed => "zvksed",
            Zvksh => "zvksh",
            Svinval => "svinval",
            Sdext => "sdext",
            Smrnmi => "smrnmi",
            Ssctr => "ssctr",
        }
    }

    /// The extension named `name` in an ISA string, ignoring case.
    pub fn from_name(name: &str) -> Option<Extension> {
        Extension::ALL
            .into_iter()
            .find(|e| e.name().eq_ignore_ascii_case(name))
    }

    /// The extensions this one implies.
    const fn implies(self) -> &'static [Extension] {
        use Extension::*;
        match self {
            D | Zfhmin | Zfa => &[F],
            Q => &[D],
            Zfh => &[Zfhmin],
            Zcb | Zcmop => &[C],
            Zicfiss => &[Zimop],
            Zvbb | Zvbc | Zvkg | Zvkned | Zvknha | Zvksed | Zvksh => &[V],
            Zvknhb => &[Zvknha],
            _ => &[],
        }
    }

    const fn bit(self) -> u128 {
        1 << self as u32
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A set of [`Extension`]s.
///
/// Adding an extension also adds the extensions it implies (D implies F),
/// and removing one also removes the extensions that imply it (removing F
/// removes D).
///
/// ```
/// use disc_v::{Extension, Extensions};
///
/// let exts = Extensions::GC.with(Extension::Zba).without(Extension::D);
/// assert!(exts.contains(Extension::F));
/// assert!(!exts.contains(Extension::Q));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Extensions(u128);

impl Extensions {
    /// No extensions: only the base integer ISA.
    pub const EMPTY: Extensions = Extensions(0);

    /// Every supported extension that does not conflict with another. This
    /// is what [`decode`](fn@crate::decode) and the other free functions use.
    pub const DEFAULT: Extensions = Extensions::of(&Extension::ALL);

    /// G and C: IMAFD, Zicsr, Zifencei and C, as in RV32GC and RV64GC.
    pub const GC: Extensions = {
        use Extension::*;
        Extensions::of(&[M, A, F, D, C, Zicsr, Zifencei])
    };

    /// The extensions of the RVA23U64 profile that define instructions.
    pub const RVA23U64: Extensions = {
        use Extension::*;
        Extensions::of(&[
            M,
            A,
            F,
            D,
            C,
            V,
            Zicsr,
            Zicond,
            Zawrs,
            Zicbom,
            Zicboz,
            Zicbop,
            Zihintntl,
            Zihintpause,
            Zimop,
            Zcmop,
            Zba,
            Zbb,
            Zbs,
            Zcb,
            Zfhmin,
            Zfa,
            Zvbb,
        ])
    };

    /// The extensions of the RVA23S64 profile that define instructions:
    /// RVA23U64, Zifencei, Svinval and H.
    pub const RVA23S64: Extensions = {
        use Extension::*;
        Extensions::RVA23U64.union(Extensions::of(&[Zifencei, Svinval, H]))
    };

    /// The extensions in `exts`, and the extensions they imply.
    pub const fn of(exts: &[Extension]) -> Extensions {
        let mut set = Extensions::EMPTY;
        let mut i = 0;
        while i < exts.len() {
            set = set.with(exts[i]);
            i += 1;
        }
        set
    }

    /// Whether the set contains `ext`.
    pub const fn contains(self, ext: Extension) -> bool {
        self.0 & ext.bit() != 0
    }

    /// The set with `ext` and the extensions it implies added.
    pub const fn with(self, ext: Extension) -> Extensions {
        if self.contains(ext) {
            return self;
        }
        let mut set = Extensions(self.0 | ext.bit());
        let implied = ext.implies();
        let mut i = 0;
        while i < implied.len() {
            set = set.with(implied[i]);
            i += 1;
        }
        set
    }

    /// The set with `ext` and the extensions that imply it removed.
    pub const fn without(self, ext: Extension) -> Extensions {
        let mut set = Extensions(self.0 & !ext.bit());
        let mut i = 0;
        while i < Extension::ALL.len() {
            let other = Extension::ALL[i];
            let implied = other.implies();
            let mut j = 0;
            while j < implied.len() {
                if implied[j] as u32 == ext as u32 && set.contains(other) {
                    set = set.without(other);
                }
                j += 1;
            }
            i += 1;
        }
        set
    }

    /// The extensions in either set.
    pub const fn union(self, other: Extensions) -> Extensions {
        Extensions(self.0 | other.0)
    }

    /// Whether the set contains every extension in `exts`.
    pub(crate) fn contains_all(self, exts: &[Extension]) -> bool {
        exts.iter().all(|&e| self.contains(e))
    }

    /// The extensions in the set, in declaration order.
    pub fn iter(self) -> impl Iterator<Item = Extension> {
        Extension::ALL
            .into_iter()
            .filter(move |&e| self.contains(e))
    }
}

impl Default for Extensions {
    /// [`Extensions::DEFAULT`].
    fn default() -> Self {
        Extensions::DEFAULT
    }
}

impl fmt::Debug for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl From<Extension> for Extensions {
    fn from(ext: Extension) -> Self {
        Extensions::EMPTY.with(ext)
    }
}

impl<const N: usize> From<[Extension; N]> for Extensions {
    fn from(exts: [Extension; N]) -> Self {
        Extensions::of(&exts)
    }
}

impl From<&[Extension]> for Extensions {
    fn from(exts: &[Extension]) -> Self {
        Extensions::of(exts)
    }
}

impl FromIterator<Extension> for Extensions {
    fn from_iter<I: IntoIterator<Item = Extension>>(iter: I) -> Self {
        iter.into_iter().fold(Extensions::EMPTY, Extensions::with)
    }
}

impl core::ops::BitOr for Extensions {
    type Output = Extensions;

    fn bitor(self, other: Extensions) -> Extensions {
        self.union(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for ext in Extension::ALL {
            assert_eq!(Extension::from_name(ext.name()), Some(ext));
        }
        assert_eq!(Extension::from_name("ZBA"), Some(Extension::Zba));
        assert_eq!(Extension::from_name("zbkb"), None);
    }

    #[test]
    fn all_lists_every_extension_once() {
        let mut bits = 0u128;
        for (i, ext) in Extension::ALL.into_iter().enumerate() {
            assert_eq!(ext as usize, i);
            bits |= ext.bit();
        }
        assert_eq!(bits.count_ones() as usize, Extension::ALL.len());
    }

    #[test]
    fn implications() {
        use Extension::*;
        let set = Extensions::EMPTY.with(Q);
        assert_eq!(set, Extensions::of(&[F, D, Q]));
        assert_eq!(set.without(F), Extensions::EMPTY);
        assert_eq!(set.without(Q), Extensions::of(&[F, D]));
        assert!(Extensions::from(Zvknhb).contains(V));
    }
}
