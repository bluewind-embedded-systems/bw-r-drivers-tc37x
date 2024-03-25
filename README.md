# Rust TC37x drivers

## Getting started

### Toolchain

Make sure you have a working toolchain. Check [Rust
Development Platform for Infineon AURIX](https://hightec-rt.com/en/rust) website to get the toolchain.

Once it is installed it should appear as a rustup toolchain:

```shell
rustup toolchain list
```

The expected output should be something like this, it may vary depending on your system:

```text
stable-x86_64-unknown-linux-gnu (default)       /home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu
tricore /opt/HighTec/toolchains/rust/v0.2.0/
```

If `tricore` does not appear in this list, but the toolchain is installed
somewhere in your system, you can [teach rustup about
it](https://rust-lang.github.io/rustup/concepts/toolchains.html#custom-toolchains).
For instance, if the toolchain is installed in
`/opt/HighTec/toolchains/rust/v0.2.0/` you can add an alias for that toolchain:

```shell
rustup toolchain link tricore /opt/HighTec/toolchains/rust/v0.2.0/
```

Check again with `rustup toolchain list` and you should have `tricore` in the output.

Currently, this crate is tested with `tricore-htc-none-v0.2.0`.

If the toolchains is ready, check that you can build the examples (see below).

### Examples

[Here you can find some
examples](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples).
They are meant to be standalone and to be used as a boilerplate for your new
project. 

Try a simple example:

```shell
git clone https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples.git
cd bw-r-drivers-tc37x-examples
cd blinky
cargo +tricore build --target=tc162-htc-none
```

This will build the `blinky` example and generate an ELF file in
`./target/tc162-htc-none/debug/blinky.elf` directory.

You can use the `tricore-probe` tool to flash the binary to the target hardware:

```shell
tricore-probe ./target/tc162-htc-none/debug/blinky.elf
```


#### Make your own executable

You can copy one example directory (e.g. `blinky`) to a new directory:

```shell
cd bw-r-drivers-tc37x-examples
cp -r blinky my-example
cd my-example
```

Now edit `Cargo.toml` and change the package name:

```toml
[package]
name = "my-example"
```

### Drivers

This crate contains many low level drivers for the `TC37x` microcontroller:

- [gpio](gpio)
- [can](can)
- [adc](adc)

Refer to the documentation you find here and the examples to understand how to
use the different drivers.

## Development utilities

### Tracing

This crate has a `tracing` feature, which let you trace all side effects to the
peripherals registers:

- read
- write
- load-modify-store

This is particularly useful to write automatic tests for low level drivers,
because you don't need a real Aurix hardware to run the tests. You can use the
`tracing` feature to test your application or other higher level drivers built
on top of this crate.

Take a look at `tests/gpio.rs` for some simple examples of tests.

### Logging

This crate uses the `defmt` logging framework.
More details about `defmt` can be found [here](https://defmt.ferrous-systems.com/).

Logging is disabled by default. To enable it, you need to add the `log_with_defmt` feature to your `Cargo.toml`:

```toml
[dependencies]
bw-r-drivers-tc37x = { version = "*", features = ["log_with_defmt"] }
```

`defmt` requires a probe to be connected to the target hardware to receive the log messages.

Check `tricore-probe` [documentation](https://github.com/veecle/tricore-probe) for more details on how to use it.

