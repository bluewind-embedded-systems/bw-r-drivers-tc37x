# Rust TC37x drivers

## Getting started

This crate provides low level drivers for the `TC37x` microcontroller family.

To get familiar with the drivers, you can start with the examples.

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

If the `tricore` toolchain is available in your system, the command above should 
build the `blinky` example and generate an ELF file in
`./target/tc162-htc-none/debug/blinky.elf` directory.

If it fails with an error like `error: toolchain 'tricore' is not installable`,
you need to install the `tricore` toolchain. Check the [Toolchain](#toolchain)
section below.

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

Refer to the documentation you find here and the examples to understand how to
use the different drivers.

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

Check [Rust
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

Take a look at [`tests/gpio.rs`](./tests/gpio.rs) for some simple examples of tests.

### Logging

This crate uses the `defmt` logging framework.
More details about `defmt` can be found [here](https://defmt.ferrous-systems.com/).

Logging is disabled by default. To enable it, you need to add the `log_with_defmt` feature to your `Cargo.toml`:

```toml
[dependencies]
bw-r-drivers-tc37x = { version = "*", features = ["log_with_defmt"] }
```

`defmt` requires a probe to be connected to the target hardware to receive the log messages.

Check [tricore-probe](https://github.com/veecle/tricore-probe) 
and [defmt](https://defmt.ferrous-systems.com/) for more details on how to use it.

### Testing

This crate contains some tests that do not require a real Aurix hardware to run.
You don't even need to have the `tricore` toolchain installed to run these tests.
You can run them with the following command:

```shell
cargo test -Ftracing
```

### Continuous Integration

This crate uses GitHub Actions to run the tests on every push to the repository.

You can check the status of the tests by clicking on the "Actions" tab in the
GitHub repository.

Currently, the tests are run on the `x86_64-unknown-linux-gnu` target with
the `tracing` feature enabled and the official Rust toolchain.

Documentation is also built and published to GitHub Pages on every push to the
`main` branch.

## Troubleshooting

### Toolchain not found

```text
error: toolchain 'tricore' is not installable
```

If you see this error message, you need to install the `tricore` toolchain. 
Check the [Toolchain](#toolchain) section above.

### Could not find tricore in arch

```text
error[E0433]: failed to resolve: could not find `tricore` in `arch`
   --> /path/to/pac/src/common.rs:343:25
    |
343 |             core::arch::tricore::intrinsics::__ldmst(self.ptr as *mut u32, res.data, res.mask);
    |                         ^^^^^^^ could not find `tricore` in `arch`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `tc37x` (lib) due to previous error
```

If you see this error message, you need to select the correct target for your 
project.

This is usually done by setting the `--target=tc162-htc-none` option when
using `cargo build` or `cargo run`.

To make this configuration permament, you can add a `.cargo/config.toml`
file in your project with the following content:

```toml
[build]
target = "tc162-htc-none"
```

### Can't find crate for std

```text
error[E0463]: can't find crate for `std`
  |
  = note: the `tc162-htc-none` target may not support the standard library
```

```text
error: cannot find attribute `test` in this scope
```

```text
error[E0463]: can't find crate for `test`
```

If you see any of these error messages, you are probably trying to run tests meant to be
run on the host machine, but you are using the `tc162-htc-none` target.

This happens when there is a `config.toml` file in the `.cargo` directory of your project.
To run tests on the host machine, you need to remove the `config.toml` file or
change the target to the host machine. You can use the `x86_64-unknown-linux-gnu`
target. Check `rustc -vV` output to see which is your host triple.
For instance, if you are using a Linux machine, you should be able to run:

```shell
cargo +tricore test --target=x86_64-unknown-linux-gnu -Ftracing
```