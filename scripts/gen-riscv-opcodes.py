#!/usr/bin/env python3
# Copyright (c) 2026 Christopher Swenson
# Licensed under the Apache-2.0 license.
"""Generates tests/data/riscv-opcodes.txt from a checkout of
https://github.com/riscv/riscv-opcodes.

Usage: scripts/gen-riscv-opcodes.py <riscv-opcodes checkout> > tests/data/riscv-opcodes.txt

Each output line describes one instruction encoding:

    <isas> <name> <base> <match> <mask> <nonzero> <not-two> <extensions>

  isas     `32`, `64` or `32,64`: the base ISAs the encoding is valid for
  name     mnemonic, with riscv-opcodes' `.rv32`/`.rv64` suffixes removed
  base     for a pseudo-op, the instruction it is an alias of; otherwise `-`
  match    bits that must be set, in hex, under...
  mask     ...this mask
  nonzero  comma-separated field masks that must not be all zero, or `-`
           (for example `c.addi`'s rd and nonzero immediate)
  not-two  field mask that must not equal 2 (`c.lui`'s rd), or `-`
  extensions
           the extensions the instruction needs, joined with `+`, or `-` for
           the base ISA; from the riscv-opcodes file name (`rv_d_zfa` is
           `d+zfa`). An instruction that several extensions provide (by
           `$import`) lists them joined with `|` (`zbb|zbkb`).
"""
import csv, os, re, subprocess, sys

# The extensions disc-v supports.
FILES = """rv_i rv32_i rv64_i rv_m rv64_m rv_a rv64_a rv_f rv64_f rv_d rv64_d
rv_q rv64_q rv_c rv32_c rv64_c rv_c_d rv32_c_f rv_zicsr rv_zifencei rv_system
rv_s rv_sdext rv_zicntr rv_zba rv64_zba rv_zbb rv32_zbb rv64_zbb rv_zbc rv_zbs
rv32_zbs rv64_zbs rv_zicond rv_zawrs rv_zicbo
rv_zihintntl rv_c_zihintntl rv_zimop rv_zcmop rv_zicfiss rv_c_zicfiss
rv_zicfilp rv_zcb rv64_zcb
rv_zfh rv64_zfh rv_zfhmin rv_d_zfhmin rv_q_zfhmin rv_f_zfa rv_d_zfa rv32_d_zfa
rv_q_zfa rv64_q_zfa rv_zfh_zfa rv_v rv_zvbb rv_zvbc rv_zvkg rv_zvkned
rv_zvknha rv_zvksed rv_zvksh rv_h rv64_h rv_svinval rv_svinval_h rv_smrnmi
rv_ssctr rv_zalasr rv_zacas rv64_zacas rv_zabha rv_zabha_zacas rv_zfbfmin
rv_zvfbfmin rv_zvfbfwma rv_zbkb rv32_zbkb rv64_zbkb rv_zbkc rv_zbkx rv32_zknd rv64_zknd
rv32_zkne rv64_zkne rv_zknh rv32_zknh rv64_zknh rv_zksed rv_zksh rv_zcmp rv_zcmt""".split()

root = sys.argv[1]
ext_dir = os.path.join(root, 'extensions')
fields = {name: (int(hi), int(lo)) for name, hi, lo in csv.reader(open(os.path.join(root, 'arg_lut.csv')), skipinitialspace=True)}

def field_mask(name):
    hi, lo = fields[name]
    return ((1 << (hi - lo + 1)) - 1) << lo

def parse_line(tokens):
    """Returns (name, match, mask, nonzero masks, not-two mask)."""
    name, rest = tokens[0], tokens[1:]
    match = mask = 0
    nz, n2 = {}, 0
    for t in rest:
        m = re.fullmatch(r'(\d+)(?:\.\.(\d+))?=(\w+)', t)
        if m:
            hi = int(m.group(1)); lo = int(m.group(2) or hi)
            bits = ((1 << (hi - lo + 1)) - 1) << lo
            match |= (int(m.group(3), 0) << lo) & bits
            mask |= bits
            continue
        if t.endswith('_n0') or t.endswith('_n0_e') or t in ('rd_n2',):
            nz[t] = field_mask(t)
        if t == 'rd_n2':
            n2 = field_mask(t)
        nm = re.match(r'c_nz(u?imm\d+)', t)
        if nm:  # a nonzero immediate split across several fields
            nz[nm.group(1)] = nz.get(nm.group(1), 0) | field_mask(t)
    return name, match, mask, list(nz.values()), n2

def file_lines(f):
    for line in open(os.path.join(ext_dir, f)):
        line = line.split('#', 1)[0].strip()
        if line:
            yield line.split()


def find_def(f, name):
    for tokens in file_lines(f):
        if tokens[0] == name:
            return tokens
    raise KeyError(f'{name} not in {f}')

def segment_name(name, fields):
    """The name of a vector load or store with `fields` fields (nf + 1), as
    in vlseg2e32.v for vle32.v. riscv-opcodes leaves nf as an operand."""
    m = re.fullmatch(r'v([ls])(e|se|uxei|oxei)(\d+)(ff)?\.v', name)
    kind = {'e': 'seg{n}e', 'se': 'sseg{n}e', 'uxei': 'uxseg{n}ei', 'oxei': 'oxseg{n}ei'}
    return f"v{m.group(1)}{kind[m.group(2)].format(n=fields)}{m.group(3)}{m.group(4) or ''}.v"

# File-name parts that are the base ISA (or privileged instructions that every
# implementation has) rather than an extension.
BASE = {'i', 'system', 's', 'zicntr'}

def extensions(f, name):
    """The extensions an instruction from riscv-opcodes file `f` needs, with
    the extensions that also provide it by importing it."""
    parts = file_extensions(f, name)
    alts = sorted(ALTERNATIVES.get((f, name), set()) - set(parts))
    if alts:
        assert len(parts) == 1, (f, name)
        return '|'.join(parts + alts)
    return '+'.join(parts) or '-'

def file_extensions(f, name):
    """The extensions named by riscv-opcodes file `f`."""
    parts = f.split('_')[1:]
    if parts == ['zicbo']:  # The file holds three extensions.
        parts = ['zicboz' if name == 'cbo.zero' else 'zicbop' if name.startswith('prefetch')
                 else 'zicbom']
    parts = [p for p in parts if p not in BASE]
    # The compressed files are Zca, except rv_c_d (Zcd) and rv32_c_f (Zcf).
    if parts == ['c', 'd']:
        return ['zcd']
    if parts == ['c', 'f']:
        return ['zcf']
    return ['zca' if p == 'c' else p for p in parts]


def strip_suffix(name):
    return re.sub(r'[._]rv(32|64)$', '', name)

# (file, instruction) -> the extensions of the files that also provide it,
# by $import or by a $pseudo_op of the same name (as rv64_zbkb does for
# rv64_zbb's rev8).
ALTERNATIVES = {}
for f in FILES:
    for tokens in file_lines(f):
        if tokens[0] in ('$import', '$pseudo_op'):
            src, name = tokens[1].split('::')
            if tokens[0] == '$pseudo_op' and strip_suffix(tokens[2]) != strip_suffix(name):
                continue
            ALTERNATIVES.setdefault((src, name), set()).update(file_extensions(f, name))

rev = subprocess.run(['git', '-C', root, 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()
print(f'# Generated by scripts/gen-riscv-opcodes.py from riscv-opcodes {rev}.')
print('# riscv-opcodes is Copyright (c) 2022 RISC-V International, BSD-3-Clause.')
print('# isas name base match mask nonzero not-two extensions')
for f in FILES:
    isas = {'rv': '32,64', 'rv32': '32', 'rv64': '64'}[f.split('_')[0]]
    for tokens in file_lines(f):
        base = '-'
        source = f
        if tokens[0] == '$import':
            source, name = tokens[1].split('::')
            tokens = find_def(source, name)
        elif tokens[0] == '$pseudo_op':
            base = strip_suffix(tokens[1].split('::')[1])
            tokens = tokens[2:]
        name, match, mask, nz, n2 = parse_line(tokens)
        nzs = ','.join(f'{m:x}' for m in nz) or '-'
        n2s = f'{n2:x}' if n2 else '-'
        variants = [(strip_suffix(name), base, match, mask)]
        if 'nf' in tokens:
            # Each value of nf is a separately named segment instruction.
            nf_mask = 7 << 29
            variants = [(name, base, match, mask | nf_mask)] + [
                (segment_name(name, nf + 1), name, match | nf << 29, mask | nf_mask)
                for nf in range(1, 8)]
        exts = extensions(source, name)
        for n, b, m, k in variants:
            print(f"{isas} {n} {b} {m:x} {k:x} {nzs} {n2s} {exts}")
