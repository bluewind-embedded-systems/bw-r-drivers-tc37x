# tc37x-simple-driver

## Prerequisites

Make sure `tc37x-pac` and `tc37x-rt` are available in the parent directory.

You should have a workspace with this setup:

```txt
├── tc37x-driver
│   ├── Cargo.toml
│   ...
├── tc37x-pac
    ├── Cargo.toml
    ...
```

## Examples

You can run an example with `tricore-probe`.

The file `.cargo/config.toml` is already set up to launch all examples with
tricore-probe. So you can just use `cargo run` to run examples on target.

Make sure you have a TC37x board attached to yout PC through a debugger  and
launch an example:

```sh
cargo run --example=blinky --features=example
```
