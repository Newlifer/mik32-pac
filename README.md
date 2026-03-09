# mik32-pac - PAC for Mikron Mik32 Amur (К1948ВК018) microcontoller

This is a [Peripheral Access Crate](https://docs.rust-embedded.org/book/start/registers.html) for the Mikron Mik32 Amur (К1948ВК018) RISC-V microcontroller.

# How to update
Two ways: justfile

```sh
# Getting tools
just get-tools
# Build PAC
just build
```

Or manual:
```sh
# Getting tools
cargo install svd2rust
cargo install form
rustup component add rustfmt

# Build PAC
svd2rust -i ./misc/mik32v2.svd --target riscv
form -i ./lib.rs -o ./src/
cargo fmt
rm lib.rs
rm build.rs
```

# License
The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by Mikron under a MIT licence.
