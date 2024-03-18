# bw-r-driver-tc37x

## Prerequisites

Make sure `tc37x` and `tc37x-rt-example` are available in the parent directory.

You should have a workspace with this setup:


- 📂 bw-r-driver-tc37x
   - 📄 `Cargo.toml`
   -  ` ...`
- 📂 tc37x --> [tc375-pac](https://github.com/Infineon/tc375-pac) 
   -  `Cargo.toml`
   -  `...`
- 📂 [tc37x-rt](https://github.com/bluewind-embedded-systems/bw-r-rt-example)
   -  `Cargo.toml`
   - `...`


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
