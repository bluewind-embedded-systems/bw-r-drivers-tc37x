<img src="../.media/logo_bw.png" align="right" width="150" />  

## Troubleshooting

### Toolchain not found

```text
error: toolchain 'tricore' is not installable
```

If you see this error message, you need to install the `tricore` toolchain. 
Check the [Toolchain](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x/blob/main/doc/usage-guide.md#toolchain) section.

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
