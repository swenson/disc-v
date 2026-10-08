# panic-check

Checks that disc-v cannot panic: a `no_std` program that calls every public
function and trait implementation of disc-v on inputs the compiler cannot
see, built with optimizations and link-time optimization. Its panic handler
calls a function that does not exist, so if any panic is reachable, the link
fails with an undefined symbol `disc_v_panic_is_reachable`.

```sh
cd panic-check
cargo build --release                                  # riscv32imc
cargo build --release --target thumbv7em-none-eabi
```

To find the panics, build with the `diagnose` feature, which links a panic
handler, and list the call sites with the disc-v source lines they come
from (this needs RISC-V binutils):

```sh
cargo build --release --features diagnose
python3 -I ../scripts/panic-sites.py \
    target/riscv32imc-unknown-none-elf/release/disc-v-panic-check
```

`../scripts/panic-check.sh` does both for RISC-V and Arm at optimization
levels 2, 3, `s` and `z`, as CI does with the stable compiler and the
minimum supported version (choose the compiler with `RUSTUP_TOOLCHAIN`).

The check covers those build configurations, not every build:

- Debug builds keep overflow checks and debug assertions.
- At optimization level 1, core's `Debug` for structs keeps an assertion.
- `MarchError`'s `Debug` output escapes the extension name with core's
  `Debug` for `str`, which has panic paths the optimizer cannot remove, so
  the program uses its `Display` output instead.
- A new compiler can fail to remove a check that an older one removed; the
  fix is usually `get` instead of indexing, or masking an index.

When adding to disc-v's public API, call it from `src/main.rs`.
