<img src="../.media/logo_bw.png" align="right" width="150" />  

# Usage guide 

## Getting started

This crate provides low level drivers for the `TC37x` microcontroller family.

To get familiar with the drivers, you can start with the examples.

### Examples

[Here you can find some examples](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples).

They are meant to be standalone and to be used as a boilerplate for your new
project. 

Try a simple example:

```shell
git clone https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples.git
cd bw-r-drivers-tc37x-examples
cd blinky
cargo +tricore build --target=tc162-htc-none
```

If the `tricore` toolchain is available in your system, the command above should 
build the `blinky` example and generate an ELF file in
`./target/tc162-htc-none/debug/blinky.elf` directory.

If it fails with an error like `error: toolchain 'tricore' is not installable`,
you need to install the `tricore` toolchain. Check the [Toolchain](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x/blob/main/doc/usage-guide.md#toolchain) section.

You can use the `tricore-probe` tool to flash the binary to the target hardware:

```shell
tricore-probe ./target/tc162-htc-none/debug/blinky.elf
```

If `tricore-probe` is not available in your system, you can install it by following
the instructions in the [tricore-probe repository](https://github.com/veecle/tricore-probe).

Alternatively, you can use other tools like 
[Infineon MemTool](https://www.infineon.com/cms/en/tools/aurix-tools/free-tools/infineon/)
or
[Universal Debug Engine](https://www.pls-mc.com/products/universal-debug-engine/).

### Make your own executable

You can copy one example directory (e.g. `blinky`) to a new directory:

```shell
cp -r bw-r-drivers-tc37x-examples/blinky my-rust-project
cd my-rust-project
```

Now edit `Cargo.toml` and change the package name:

```toml
[package]
name = "my-rust-project"
```

### Drivers

This crate contains many low level drivers for the `TC37x` microcontroller:

- [gpio](gpio)
- [can](can)

Refer to the documentation you find here and the examples to understand how to use the different drivers.

### Safety

This crate uses `unsafe` code to access the hardware registers. The drivers are
written in a way that should prevent undefined behavior, but it is not
guaranteed. If you find any issue, please report it.

All peripherals are `Send` and `Sync`, so you are able to share them between threads
or interrupt handlers, but you need to be careful with the synchronization.

Currently, **there is no ownership system in place to prevent you from using the
same peripheral in different parts of your code**. You need to be careful with
this, because it can lead to undefined behavior.

## Prerequisites

### Toolchain

Check [Hightec - Rust Development Platform for Infineon AURIX](https://hightec-rt.com/en/rust) website to get the toolchain.

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

### tricore-probe

We recommend using the `tricore-probe` tool to flash the binary to the target hardware.

`tricore-probe` embedded programs just like native ones, so you can use `cargo run` to flash
the binary to the target hardware and also see the logs.

You can set `tricore-probe` as the default runner for your project by adding the following
lines to your `.cargo/config.toml` file:

```toml
[target.tc162-htc-none]
runner = "tricore-probe"
```

