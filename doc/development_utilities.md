<img src="../.media/logo_bw.png" align="right" width="150" />  

# Development utilities

## Tracing

This crate has a `tracing` feature, which let you trace all side effects to the peripherals registers:

- read
- write
- load-modify-store

This is particularly useful to write automatic tests for low level drivers, because you don't need a real Aurix hardware to run the tests. 

You can use the `tracing` feature to test your application or other higher level drivers built on top of this crate.

Take a look at [`tests/gpio.rs`](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x/tests/gpio.rs) for some simple examples of tests.

## Logging

This crate uses the `defmt` logging framework.
More details about `defmt` can be found [here](https://defmt.ferrous-systems.com/).

Logging is disabled by default. To enable it, you need to add the `log_with_defmt` feature to your `Cargo.toml`:

```toml
[dependencies]
bw-r-drivers-tc37x = { version = "*", features = ["log_with_defmt"] }
```

`defmt` requires a probe to be connected to the target hardware to receive the log messages.

Check [tricore-probe](https://github.com/veecle/tricore-probe) and [defmt](https://defmt.ferrous-systems.com/) for more details on how to use it.

### Testing

This crate contains some tests that do not require a real Aurix hardware to run.
You don't even need to have the `tricore` toolchain installed to run these tests.You can run them with the following command:

```shell
cargo test -Ftracing
```

## Continuous Integration

This crate uses GitHub Actions to run the tests on every push to the repository.

You can check the status of the tests by clicking on the "Actions" tab in the GitHub repository.

Currently, the tests are run on the x86_64-unknown-linux-gnu` target with the `tracing` feature enabled and the official Rust toolchain.

Documentation is also built and published to GitHub Pages on every push to the `main` branch.

