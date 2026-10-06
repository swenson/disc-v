//! Helpers shared by the integration tests.

use std::collections::BTreeMap;
use std::fmt::Display;

/// A small deterministic xorshift generator, so failures are reproducible.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1)
    }

    pub fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 32) as u32
    }
}

/// Collects test failures, grouped by kind so that a systematic bug shows
/// up once with a count and an example.
#[derive(Default)]
pub struct Failures(BTreeMap<String, (usize, String)>);

impl Failures {
    pub fn add(&mut self, kind: impl Display, example: impl Display) {
        self.0
            .entry(kind.to_string())
            .or_insert((0, example.to_string()))
            .0 += 1;
    }

    pub fn assert_none(self) {
        if self.0.is_empty() {
            return;
        }
        let total: usize = self.0.values().map(|(n, _)| n).sum();
        let mut msg = format!("{total} failures of {} kinds:\n", self.0.len());
        for (kind, (count, example)) in &self.0 {
            msg += &format!("{count:>7} x {kind}\n          e.g. {example}\n");
        }
        panic!("{msg}");
    }
}

/// Instruction encodings for every entry in riscv-opcodes: up to `per_entry`
/// random encodings that satisfy each entry's constraints.
pub fn riscv_opcodes_samples(rng: &mut Rng, per_entry: usize) -> Vec<u32> {
    entries()
        .iter()
        .flat_map(|e| e.samples(rng, per_entry))
        .collect()
}

/// An instruction encoding from `tests/data/riscv-opcodes.txt`.
pub struct Entry {
    pub rv32: bool,
    pub rv64: bool,
    pub name: &'static str,
    /// For a pseudo-op, the instruction it is an alias of.
    pub base: Option<&'static str>,
    pub bits: u32,
    pub mask: u32,
    pub nonzero: Vec<u32>,
    pub not_two: u32,
}

impl Entry {
    pub fn applies_to(&self, isa: disc_v::Isa) -> bool {
        match isa {
            disc_v::Isa::Rv32 => self.rv32,
            disc_v::Isa::Rv64 => self.rv64,
            disc_v::Isa::Rv128 => false,
        }
    }

    pub fn matches(&self, inst: u32) -> bool {
        inst & self.mask == self.bits
            && self.nonzero.iter().all(|&m| inst & m != 0)
            && (self.not_two == 0 || (inst & self.not_two) >> self.not_two.trailing_zeros() != 2)
    }

    pub fn is_named(&self, name: &str) -> bool {
        self.name == name || self.base == Some(name)
    }

    /// Up to `n` random encodings that match.
    pub fn samples(&self, rng: &mut Rng, n: usize) -> Vec<u32> {
        let width = if self.bits & 3 == 3 { u32::MAX } else { 0xffff };
        (0..n * 8)
            .map(|_| (rng.next_u32() & !self.mask & width) | self.bits)
            .filter(|&x| self.matches(x))
            .take(n)
            .collect()
    }
}

pub fn entries() -> Vec<Entry> {
    let hex = |s: &str| u32::from_str_radix(s, 16).unwrap();
    include_str!("../data/riscv-opcodes.txt")
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| {
            let f: Vec<&'static str> = l.split(' ').collect();
            Entry {
                rv32: f[0].contains("32"),
                rv64: f[0].contains("64"),
                name: f[1],
                base: Some(f[2]).filter(|&b| b != "-"),
                bits: hex(f[3]),
                mask: hex(f[4]),
                nonzero: match f[5] {
                    "-" => vec![],
                    s => s.split(',').map(hex).collect(),
                },
                not_two: if f[6] == "-" { 0 } else { hex(f[6]) },
            }
        })
        .collect()
}
