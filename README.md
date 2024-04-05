<img src=".media/logo_bw.png" align="right" width="150" />

# Rust TC37x drivers

# About Bluewind

[Bluewind](https://www.bluewind.it/) is an independent engineering company that
provides world-class products, engineering and software solutions in the
domains of electronics, safety critical applications, and connected devices.

Bluewind is a strategic partner who creates value in the whole product
innovation cycle, taking part to product strategy stage, and providing
electronics and software design, certifications consultancy, and production.

# Table of Contents

- [Getting started and Code Examples](#getting-started)
- [Usage Guide][usage-guide]
- [Code Documenation][bw-r-drivers-tc37x-documentation]
- [Development utilities][dev-utils]
- [Troubleshooting][troubleshooting]


# AURIX™ Rust Startup Ecosystem

The AURIX™ Rust Startup Ecosystem is a collaborative effort involving
[Veecle][veecle], [Infineon][infineon], [HighTec][hightec] and
[Bluewind][bluewind] aimed at supporting Rust on Infineon's AURIX™ architecture
for automotive and industrial applications. The primary objective is to empower
    customers to seamlessly integrate Rust tasks alongside existing C
    implementations for evaluation and pre-development purposes.

<p align="center">
  <img src="./.github/ecosystem.png" alt="AURIX Rust Startup Ecosystem" width="75%"/>
</p>

The AURIX™ Rust Startup Ecosystem consists of:

* A [Peripheral Access Crate][tc375-pac] (PAC) from Infineon.
* [Low-level drivers][bw-r-drivers-tc37x-github] from Bluewind, fully written
  in Rust.
* A precompiled version of [PXROS-HR][pxros], an ASIL-D RTOS written in C,
  developed by HighTec.
* Rust [PXROS-HR bindings][pxros-rust-bindings] developed jointly by Veecle and
  HighTec.
* A Rust runtime from Veecle, named [veecle-pxros], which
  seamlessly integrates with PXROS-HR, providing a native Rust experience. This
  runtime also supports asynchronous execution where feasible.
* A curated set of examples by Veecle and Bluewind, covering bare metal driver
  examples, driver instances employing PXROS-HR, and connectivity application
  demonstrations.

For compiling Rust for AURIX™, HighTec offers a combined package of their Rust
and C/C++ compiler, accessible [here][hightec-development-platform].

Finally, to facilitate flashing and debugging on AURIX Veecle is maintaining
the [tricore-probe].

For additional information visit:
* https://www.veecle.io/aurix
* https://www.bluewind.it/rust
* https://hightec-rt.com/en/rust

## Getting started

To get familiar with the drivers, you can start with the examples
[here](bw-r-drivers-tc37x-examples).

They are meant to be standalone and to be used as a boilerplate for your new
project. 

**Try a simple example**:

```shell
git clone https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples.git
cd bw-r-drivers-tc37x-examples
cd blinky
cargo +tricore build --target=tc162-htc-none
```

Check the [Toolchain](toolchain) guide for additional information.

[usage-guide]: doc/usage-guide.md
[toolchain]: doc/usage-guide.md#toolchain
[dev-utils]: doc/development_utilities.md
[troubleshooting]: doc/troubleshooting.md
[bw-r-drivers-tc37x-documentation]: https://bluewind-embedded-systems.github.io/bw-r-drivers-tc37x/
[veecle]: https://www.veecle.io/
[infineon]: https://www.infineon.com/
[hightec]: https://hightec-rt.com
[bluewind]: https://www.bluewind.it
[pxros]: https://hightec-rt.com/en/products/real-time-os
[tc375-pac]: https://github.com/Infineon/tc375-pac
[bw-r-drivers-tc37x-github]: https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x
[pxros-rust-bindings]: https://github.com/hightec-rt/pxros
[veecle-pxros]: https://github.com/veecle/veecle-pxros
[hightec-development-platform]: https://hightec-rt.com/en/products/development-platform
[tricore-probe]: https://github.com/veecle/tricore-probe
[bw-r-drivers-tc37x-examples]: https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x-examples