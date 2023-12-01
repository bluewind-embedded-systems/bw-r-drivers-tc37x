cp .cargo/config.toml.host .cargo/config.toml
cp rust-toolchain.toml.host rust-toolchain.toml

cargo run --example=canky --features=tracing