# gpio_driver

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

## Build 

### Build the whole project

```
./tools/build_for_target # build for target
./tools/build_for_host # build for host
```

### Build an example

```
./tools/build_for_target <example> # build for target
./tools/build_for_host <example> # build for host
```

## Run

### Available examples
- `bliky`to show gpio dirver basic usage
- `can_send` to use a tc37x-litekit to transmit/receive message from/to another device. 

### Run an example
The file `.cargo/config.toml` is already set up to launch all examples with
tricore-probe.

Make sure you have a TC37x board attached to yout PC through a debugger  and
launch an example:

```
./tools/run_on_target <example>
./tools/run_on_host <example>
```

## Test

```
./tools/set_build_profile host
cargo test -Ftracing --lib # run every test
cargo test -Ftracing --tests # run snapshot tests
```
