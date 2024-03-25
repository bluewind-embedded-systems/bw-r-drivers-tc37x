# Rust TC37x drivers

## Getting started

### Toolchain

Make sure you have a working toolchain. Check [Rust
Development Platform for Infineon AURIX](https://hightec-rt.com/en/rust) website to get the toolchain.

Once it is installed it should appear as a rustup toolchain:

```
rustup toolchain list
```

The expected output should be something like this:

```
stable-x86_64-unknown-linux-gnu (default)
tricore
tricore-htc-none-v0.2.0
```

If `tricore` does not appear in this list, but the toolchain is installed
somewhere in your system, you can [teach rustup about
it](https://rust-lang.github.io/rustup/concepts/toolchains.html#custom-toolchains).
For instance, if the toolchain is installed in
`/opt/HighTec/toolchains/rust/v0.2.0/`:

```
rustup toolchain link tricore /opt/HighTec/toolchains/rust/v0.2.0/
```

Check again with `rustup toolchain list` and you should have `tricore` in the output.

### Examples

[Here you can find some
examples](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples).
They are meant to be standalone and to be used as a boilerplate for your new
project. You can copy one example directory (e.g. `blinky`) to a new directory:

```
git clone https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples.git
cd bw-r-drivers-tc37x-examples
cp -r blinky my-example
cd my-example
```

Now edit `Cargo.toml` and change the package name:

```
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
