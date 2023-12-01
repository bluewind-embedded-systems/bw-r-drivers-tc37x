
cp .cargo/config.toml.target .cargo/config.toml
cp rust-toolchain.toml.target rust-toolchain.toml

export DEFMT_LOG="trace"
cargo build --example=canky --features=example