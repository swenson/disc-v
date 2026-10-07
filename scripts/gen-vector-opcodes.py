#!/usr/bin/env python3
# Copyright (c) 2026 Christopher Swenson
# Licensed under the Apache-2.0 license.
"""Generates src/opcodes/v.rs, the vector instruction tables, from a checkout
of https://github.com/riscv/riscv-opcodes.

Usage: scripts/gen-vector-opcodes.py <riscv-opcodes checkout> > src/opcodes/v.rs

Each instruction becomes a match/mask entry with an operand format string
(see `fmt` in src/opcodes/mod.rs). The operand order follows GNU objdump. The
output is formatted with rustfmt, which must be installed.
"""
import os, re, subprocess, sys

# The vector extensions disc-v supports, and the `Extension` each file's
# instructions need. Files that only $import from these (rv_zvknhb, rv_zvkn,
# rv_zvks) add no encodings; Zvknhb implies Zvknha.
FILES = {
    "rv_v": "V",
    "rv_zvbb": "Zvbb",
    "rv_zvbc": "Zvbc",
    "rv_zvkg": "Zvkg",
    "rv_zvkned": "Zvkned",
    "rv_zvknha": "Zvknha",
    "rv_zvksed": "Zvksed",
    "rv_zvksh": "Zvksh",
}

# Instructions whose multiplicand comes before vs2: vd, vs1/rs1, vs2.
MULTIPLY_ADD = """vmacc vnmsac vmadd vnmsub vwmaccu vwmacc vwmaccsu vwmaccus vfmacc vfnmacc
vfmsac vfnmsac vfmadd vfnmadd vfmsub vfnmsub vfwmacc vfwnmacc vfwmsac vfwnmsac""".split()

# Aliases objdump shows, as (instruction, alias, constraints, format).
ALIASES = [
    ("vxor.vi", "vnot.v", "ImmEq(-1)", "D,BM"),
    ("vrsub.vx", "vneg.v", "Rs1Eq(0)", "D,BM"),
    ("vwadd.vx", "vwcvt.x.x.v", "Rs1Eq(0)", "D,BM"),
    ("vwaddu.vx", "vwcvtu.x.x.v", "Rs1Eq(0)", "D,BM"),
    ("vnsrl.wx", "vncvt.x.x.w", "Rs1Eq(0)", "D,BM"),
    ("vmand.mm", "vmmv.m", "Rs2EqRs1", "D,B"),
    ("vmxor.mm", "vmclr.m", "Rs2EqRs1, RdEqRs1", "D"),
    ("vmxnor.mm", "vmset.m", "Rs2EqRs1, RdEqRs1", "D"),
    ("vmnand.mm", "vmnot.m", "Rs2EqRs1", "D,B"),
    ("vfsgnjn.vv", "vfneg.v", "Rs2EqRs1", "D,BM"),
    ("vfsgnjx.vv", "vfabs.v", "Rs2EqRs1", "D,BM"),
    ("vl1re8.v", "vl1r.v", "", "D,(1)"),
    ("vl2re8.v", "vl2r.v", "", "D,(1)"),
    ("vl4re8.v", "vl4r.v", "", "D,(1)"),
    ("vl8re8.v", "vl8r.v", "", "D,(1)"),
]

TABLES = {0x57: "OP_V", 0x77: "OP_VE", 0x07: "LOAD_FP", 0x27: "STORE_FP"}


def rustfmt(code):
    """Formats Rust code as `cargo fmt` does, so generated files pass
    `cargo fmt --check` like the rest of the crate."""
    return subprocess.run(["rustfmt", "--edition", "2024"], input=code, capture_output=True,
                          text=True, check=True).stdout

def parse(tokens):
    """Returns (name, bits, mask, args) for a riscv-opcodes line."""
    bits = mask = 0
    args = []
    for t in tokens[1:]:
        m = re.fullmatch(r"(\d+)(?:\.\.(\d+))?=(\w+)", t)
        if m:
            hi = int(m.group(1))
            lo = int(m.group(2) or hi)
            field = ((1 << (hi - lo + 1)) - 1) << lo
            bits |= (int(m.group(3), 0) << lo) & field
            mask |= field
        else:
            args.append(t)
    return tokens[0], bits, mask, args


def segment_name(name, fields):
    """The name of a load or store with `fields` fields (nf + 1), as in
    vlseg2e32.v for vle32.v."""
    m = re.fullmatch(r"v([ls])(e|se|uxei|oxei)(\d+)(ff)?\.v", name)
    kind = {"e": "seg{n}e", "se": "sseg{n}e", "uxei": "uxseg{n}ei", "oxei": "oxseg{n}ei"}
    return f"v{m.group(1)}{kind[m.group(2)].format(n=fields)}{m.group(3)}{m.group(4) or ''}.v"


def operands(name, bits, mask, args):
    """Returns (codec, format) for an instruction."""
    major = bits & 0x7F
    funct3 = (bits >> 12) & 7
    if name == "vsetvli":
        return "VsetVli", "0,1,T"
    if name == "vsetivli":
        return "VsetIvli", "0,7,T"
    if name == "vsetvl":
        return "R", "0,1,2"
    codec = "Vu6" if "zimm6hi" in args else "Vu" if "zimm5" in args else "V"
    vm = "M" if "vm" in args else ""
    if major in (0x07, 0x27):
        extra = ",2" if "rs2" in args else ",B" if "vs2" in args else ""
        return codec, f"D,(1){extra}{vm}"
    # In OPFVF (funct3=5) the scalar operand is a floating-point register, as
    # is the destination of vfmv.f.s (OPFVV, funct3=1).
    scalar = "4" if funct3 == 5 else "1"
    dest = ("3" if funct3 == 1 else "0") if "rd" in args else "D"
    src = "A" if "vs1" in args else scalar if "rs1" in args else "i" if (
        "simm5" in args or "zimm5" in args or "zimm6hi" in args) else None
    parts = [dest]
    if name.split(".")[0] in MULTIPLY_ADD:
        parts += [src, "B"]
    else:
        if "vs2" in args:
            parts.append("B")
        if src:
            parts.append(src)
    # The carry and merge forms (.vvm, .vxm, .vim, .vfm) have vm fixed at 0
    # and take v0 as an explicit operand.
    if "vm" not in args and mask >> 25 & 1 and not bits >> 25 & 1:
        parts.append("Z")
    return codec, ",".join(parts) + vm


root = sys.argv[1]
entries = []
for f, ext in FILES.items():
    for line in open(os.path.join(root, "extensions", f)):
        line = line.split("#", 1)[0].strip()
        if not line or line.startswith("$"):
            continue
        name, bits, mask, args = parse(line.split())
        if "nf" in args:
            args = [a for a in args if a != "nf"]
            for nf in range(8):
                n = name if nf == 0 else segment_name(name, nf + 1)
                entries.append((n, bits | nf << 29, mask | 7 << 29, args, ext))
        else:
            entries.append((name, bits, mask, args, ext))

aliases = {}
for base, alias, constraints, fmt in ALIASES:
    aliases.setdefault(base, []).append((alias, constraints, fmt))

tables = {t: [] for t in TABLES.values()}
for name, bits, mask, args, ext in entries:
    table = TABLES[bits & 0x7F]
    for other, obits, omask, *_ in tables[table]:
        assert (bits ^ obits) & mask & omask, f"{name} overlaps {other}"
    tables[table].append((name, bits, mask, args, ext))

rev = subprocess.run(["git", "-C", root, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()


def ident(name):
    return re.sub(r"[^A-Z0-9]", "_", name.upper())


out = [f"""// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! The vector extension (V), and the vector bit-manipulation (Zvbb, Zvbc)
//! and cryptography (Zvkg, Zvkned, Zvknha/b, Zvksed, Zvksh) extensions.
//!
//! Generated by scripts/gen-vector-opcodes.py from riscv-opcodes {rev}
//! (Copyright (c) 2022 RISC-V International, BSD-3-Clause).

use super::Constraint::*;
use super::{{Codec, MaskedOpcode, Opcode, Pseudo, masked}};
use crate::Extension;
"""]
for table, rows in tables.items():
    out.append(f"\npub(crate) static {table}: [MaskedOpcode; {len(rows)}] = [")
    for name, bits, mask, args, ext in rows:
        codec, fmt = operands(name, bits, mask, args)
        op = f'Opcode::new("{name}", Codec::{codec}, "{fmt}").requires(&[Extension::{ext}])'
        if name in aliases:
            ps = ", ".join(f"Pseudo::new(&{ident(a)}, &[{c}])" for a, c, _ in aliases[name])
            op = f"Opcode {{ pseudo: &[{ps}], ..{op} }}"
        out.append(f"    masked({bits:#010x}, {mask:#010x}, {op}),")
    out.append("];")
out.append("")
for base, alias, _, fmt in ALIASES:
    name, bits, mask, args, ext = next(e for e in entries if e[0] == base)
    codec = operands(name, bits, mask, args)[0]
    out.append(f'static {ident(alias)}: Opcode = '
               f'Opcode::new("{alias}", Codec::{codec}, "{fmt}").requires(&[Extension::{ext}]);')
sys.stdout.write(rustfmt("\n".join(out) + "\n"))
