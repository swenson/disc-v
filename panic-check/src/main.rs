// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Calls every public function and trait implementation of disc-v on inputs
//! the compiler cannot see, so that the program links only if none of them
//! can panic. See README.md.

#![no_std]
#![no_main]

use core::fmt::{self, Debug, Display, Write};
use core::hash::{Hash, Hasher};
use core::ptr::{addr_of, read_volatile, write_volatile};

use disc_v::{Decoder, Extension, Extensions, Instruction, Isa};

/// The program's input, read with volatile loads so that nothing is
/// constant-folded.
static mut INPUT: [u8; 256] = [0; 256];

/// Where output goes, written with volatile stores so that it is kept.
static mut OUTPUT: usize = 0;

fn input() -> [u8; 256] {
    // SAFETY: a volatile read of a plain array, which nothing else accesses
    // concurrently.
    unsafe { read_volatile(addr_of!(INPUT)) }
}

fn word(bytes: &[u8; 256], at: usize) -> u64 {
    let mut word = [0; 8];
    for (i, b) in word.iter_mut().enumerate() {
        *b = bytes[(at + i) % bytes.len()];
    }
    u64::from_le_bytes(word)
}

fn wide(bytes: &[u8; 256], at: usize) -> u128 {
    (u128::from(word(bytes, at)) << 64) | u128::from(word(bytes, at + 8))
}

fn output(value: usize) {
    // SAFETY: a volatile write of a plain integer.
    unsafe { write_volatile(&raw mut OUTPUT, value) }
}

/// Formatting output, kept with volatile stores.
struct Sink;

impl Write for Sink {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        output(s.len());
        Ok(())
    }
}

impl Hasher for Sink {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        output(bytes.len());
    }
}

fn show(value: &(impl Display + Debug)) {
    let _ = write!(Sink, "{value} {value:?}");
}

fn debug(value: &impl Debug) {
    let _ = write!(Sink, "{value:?}");
}

fn hash(value: &impl Hash) {
    value.hash(&mut Sink);
}

fn isa(n: u64) -> Isa {
    match n % 3 {
        0 => Isa::Rv32,
        1 => Isa::Rv64,
        _ => Isa::Rv128,
    }
}

fn instruction(ins: &Instruction) {
    show(ins);
    show(&ins.without_aliases());
    let _ = write!(Sink, "{}", ins.operands());
    debug(&ins.operands());
    hash(&ins.isa());
    output(ins.isa().xlen() as usize);
    output(ins.pc() as usize ^ ins.raw() as usize ^ ins.length());
    output(ins.mnemonic().len() ^ ins.is_illegal() as usize);
    output((*ins == ins.without_aliases()) as usize);
}

/// Every extension, including those that are not in the defaults.
fn every_extension() -> Extensions {
    use Extension::*;
    Extensions::DEFAULT | Extensions::from([E, Zcmp, Zcmt, Zclsd, Zfinx, Zdinx, Zhinx, Zhinxmin])
}

/// An extension chosen by `n`.
fn extension(n: u64) -> Option<Extension> {
    let count = every_extension().iter().count() as u64;
    every_extension().iter().nth((n % count.max(1)) as usize)
}

/// A set of extensions chosen by the bits of `mask`.
fn extensions(mask: u128) -> Extensions {
    every_extension()
        .iter()
        .enumerate()
        .filter(|(i, _)| (mask >> (i % 128)) & 1 == 1)
        .map(|(_, e)| e)
        .collect()
}

fn free_functions(bytes: &[u8; 256]) {
    let isa = isa(word(bytes, 0));
    let pc = word(bytes, 8);
    instruction(&disc_v::decode(isa, pc, word(bytes, 16)));
    let len = (word(bytes, 24) as usize) % bytes.len();
    if let Some(ins) = disc_v::decode_bytes(isa, pc, &bytes[..len]) {
        instruction(&ins);
    }
    let mut dis = disc_v::disassemble(isa, pc, &bytes[..len]);
    debug(&dis);
    for ins in dis.by_ref() {
        instruction(&ins);
    }
    output(dis.remainder().len() ^ dis.pc() as usize);
    output(disc_v::inst_length(word(bytes, 32) as u16).unwrap_or(0));
}

fn decoder(dec: Decoder, bytes: &[u8; 256]) {
    debug(&dec);
    hash(&dec);
    debug(&dec.isa());
    debug(&dec.extensions());
    hash(&dec.extensions());
    let pc = word(bytes, 40);
    instruction(&dec.decode(pc, word(bytes, 48)));
    let len = (word(bytes, 56) as usize) % bytes.len();
    if let Some(ins) = dec.decode_bytes(pc, &bytes[..len]) {
        instruction(&ins);
    }
    for ins in dec.disassemble(pc, &bytes[..len]) {
        instruction(&ins);
    }
}

fn decoders(bytes: &[u8; 256]) {
    for dec in [
        Decoder::new(isa(word(bytes, 64))),
        Decoder::RV32GC,
        Decoder::RV64GC,
        Decoder::RVA23U64,
        Decoder::RVA23S64,
    ] {
        decoder(dec, bytes);
    }
    let mask = wide(bytes, 72);
    match Decoder::with_only(isa(word(bytes, 88)), extensions(mask)) {
        Ok(dec) => decoder(dec, bytes),
        Err(conflict) => {
            show(&conflict);
            hash(&conflict);
            show(&conflict.extension());
            show(&conflict.conflicts_with());
        }
    }
    if let Some(ext) = extension(word(bytes, 96)) {
        let dec = Decoder::new(isa(word(bytes, 104)));
        match dec.with(ext) {
            Ok(dec) => decoder(dec, bytes),
            Err(conflict) => show(&conflict),
        }
        decoder(dec.try_with(ext), bytes);
        decoder(dec.without(ext), bytes);
    }
    // An ISA string from the input.
    let len = (word(bytes, 112) as usize) % 64;
    if let Some(Ok(march)) = bytes.get(128..128 + len).map(core::str::from_utf8) {
        match Decoder::from_march(march) {
            Ok(dec) => decoder(dec, bytes),
            // Not Debug: MarchError's Debug output escapes the extension
            // name with core's Debug for str, which has panic paths of its
            // own that the optimizer cannot remove.
            Err(err) => {
                let _ = write!(Sink, "{err}");
                hash(&err);
            }
        }
    }
}

fn extension_sets(bytes: &[u8; 256]) {
    let mask = wide(bytes, 120);
    let set = extensions(mask);
    if let Some(ext) = extension(word(bytes, 8)) {
        show(&ext);
        hash(&ext);
        output(ext.name().len());
        output(Extension::from_name(ext.name()).is_some() as usize);
        let others = [ext];
        for set in [
            set.with(ext),
            set.without(ext),
            set.union(Extensions::from(ext)),
            set | Extensions::from(others),
            Extensions::from(&others[..]),
            Extensions::of(&others),
            others.into_iter().collect(),
        ] {
            debug(&set);
            output(set.contains(ext) as usize);
        }
    }
    let len = (word(bytes, 16) as usize) % 64;
    if let Some(Ok(name)) = bytes.get(192..192 + len).map(core::str::from_utf8) {
        debug(&Extension::from_name(name));
    }
    for set in [
        Extensions::EMPTY,
        Extensions::DEFAULT,
        Extensions::default(),
        Extensions::GC,
        Extensions::RVA23U64,
        Extensions::RVA23S64,
    ] {
        debug(&set);
        output((set == Extensions::EMPTY) as usize);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {
        let bytes = input();
        free_functions(&bytes);
        decoders(&bytes);
        extension_sets(&bytes);
    }
}

#[cfg(not(feature = "diagnose"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe extern "C" {
        // Not defined anywhere: linking fails if a panic is reachable.
        fn disc_v_panic_is_reachable() -> !;
    }
    // SAFETY: never called, since the program does not link if it could be.
    unsafe { disc_v_panic_is_reachable() }
}

#[cfg(feature = "diagnose")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
