#!/usr/bin/env bash
# Copyright (c) 2026 Christopher Swenson
# Licensed under the Apache-2.0 license.

# Builds panic-check (see panic-check/README.md) for each target and
# optimization level, and lists the panics for any build that can panic.
# Choose the compiler with RUSTUP_TOOLCHAIN, such as RUSTUP_TOOLCHAIN=1.85.1.

set -u
cd "$(dirname "$0")/../panic-check"

# opt-level 1 is left out: it keeps a panic in core's Debug for structs.
targets=(riscv32imc-unknown-none-elf thumbv7em-none-eabi)
opt_levels=(2 3 s z)

failed=()
for target in "${targets[@]}"; do
    for opt in "${opt_levels[@]}"; do
        echo "== $target, opt-level $opt"
        if ! CARGO_PROFILE_RELEASE_OPT_LEVEL=$opt cargo build --quiet --release --target "$target"; then
            failed+=("$target opt-level $opt")
            # The panic sites are listed from the RISC-V build.
            if [ "$target" = riscv32imc-unknown-none-elf ]; then
                CARGO_PROFILE_RELEASE_OPT_LEVEL=$opt cargo build --quiet --release \
                    --target "$target" --features diagnose &&
                    python3 -I ../scripts/panic-sites.py \
                        "target/$target/release/disc-v-panic-check"
            fi
        fi
    done
done

if [ ${#failed[@]} -gt 0 ]; then
    printf 'disc-v can panic: %s\n' "${failed[@]}"
    exit 1
fi
echo "no panics"
