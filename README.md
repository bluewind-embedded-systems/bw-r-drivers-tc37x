# bw-r-driver-tc37x

## Prerequisites

Make sure `tc37x` and `tc37x-rt` are available in the parent directory.

You should have a workspace with this setup:

```txt
├── tc37x-driver
│   ├── Cargo.toml
│   ...
├── tc37x
│   ├── Cargo.toml
│    ...
└── tc37x-rt
    ├── Cargo.toml
    ...
```
:warning: reference repository for 'tc37x' is here [https://git.bwlocal.it/bw-rust/third-parties/ifx/tc37x](https://git.bwlocal.it/bw-rust/third-parties/ifx/tc37x). 

## Build & run with scripts 

You can run an example with `tricore-probe`.


On linux: 
- Build project : 
```
./tools/build_on_target 
```

- Build example (`can_send`) can_send: 
```
./tools/run_on_target can_send 
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

### Available examples
- `bliky`to show gpio dirver basic usage
- `can_send` to use a tc37x-litekit to send message to another device. 

