#!/usr/bin/env python3
# Copyright (c) 2026 Christopher Swenson
# Licensed under the Apache-2.0 license.

"""Lists the calls to panic functions in a program built by panic-check with
the `diagnose` feature, each with the innermost disc-v source line it comes
from (through inlining).

Usage: panic-sites.py ELF

Uses the RISC-V binutils objdump and addr2line, found as for the objdump
test (or with the prefix in DISC_V_BINUTILS_PREFIX). Exits with status 1 if
there are any calls.
"""

import collections
import os
import re
import shutil
import subprocess
import sys

# core's panic entry points, and the functions that report a failed index,
# slice or unwrap (mangled with the v0 or legacy scheme).
PANIC = re.compile(
    r"core(::|\d+)(panicking|str\d*slice_error_fail|slice\d+index|"
    r"option\d+(unwrap|expect)_failed|result\d+unwrap_failed)"
)
SRC = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "src") + os.sep


def function_name(symbol):
    """The last path segment of a mangled symbol, such as
    `panic_bounds_check`."""
    symbol = re.sub(r"17h[0-9a-f]{16}E$", "", symbol)  # legacy hash
    # The length-prefixed identifier that ends the symbol.
    for m in re.finditer(r"\d+", symbol):
        rest = symbol[m.end():]
        for start in range(m.start(), m.end()):
            if int(symbol[start:m.end()]) == len(rest) and rest:
                return rest
    return symbol


def binutils_prefix():
    candidates = [os.environ["DISC_V_BINUTILS_PREFIX"]] if "DISC_V_BINUTILS_PREFIX" in os.environ \
        else ["riscv64-elf-", "riscv64-unknown-elf-", "riscv64-linux-gnu-"]
    for prefix in candidates:
        if shutil.which(prefix + "objdump") and shutil.which(prefix + "addr2line"):
            return prefix
    sys.exit("RISC-V binutils not found")


def main():
    elf = sys.argv[1]
    prefix = binutils_prefix()
    dis = subprocess.run([prefix + "objdump", "-d", "--no-show-raw-insn", elf],
                         capture_output=True, text=True, check=True).stdout
    # Calls look like "  332d2:  jalr  132(ra) # 34352 <_ZN4core3str16slice_error_fail...>".
    function, sites = None, []
    for line in dis.splitlines():
        m = re.match(r"[0-9a-f]+ <(.*)>:", line)
        if m:
            function = m.group(1)
            continue
        m = re.match(r"\s*([0-9a-f]+):\s.*# [0-9a-f]+ <(.*)>", line)
        # Calls between core's panic functions are not of interest.
        if m and PANIC.search(m.group(2)) and not PANIC.search(function or ""):
            sites.append((m.group(1), m.group(2)))
    if not sites:
        print("no panics")
        return

    # With -a, each address is followed by its frames, innermost first, as
    # function and file:line pairs.
    lines = subprocess.run([prefix + "addr2line", "-e", elf, "-a", "-i", "-f", "-C"]
                           + [a for a, _ in sites], capture_output=True, text=True,
                           check=True).stdout.splitlines()
    frames, current = {}, None
    for line in lines:
        if re.fullmatch(r"0x[0-9a-f]+", line):
            current = int(line, 16)
            frames[current] = []
        else:
            frames[current].append(line)

    counts = collections.Counter()
    for address, target in sites:
        stack = frames[int(address, 16)]
        where = "outside disc-v: " + (stack[-2] if stack else "?")
        for function, location in zip(stack[0::2], stack[1::2]):
            location = re.sub(r" \(discriminator \d+\)", "", location)
            if location.startswith(SRC):
                where = location[len(SRC) - len("src/"):]
                break
            # Optimized code can lose the line, but keep the function.
            if re.match(r"<?disc_v::", function):
                where = function
                break
        kind = function_name(target)
        counts[(where, kind)] += 1
    for (where, kind), n in sorted(counts.items()):
        print(f"{n:3}  {where}  ({kind})")
    print(f"{sum(counts.values())} calls to panic functions")
    sys.exit(1)


if __name__ == "__main__":
    main()
