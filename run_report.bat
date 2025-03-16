@echo off
set RUST_LOG=info
cargo run --release --bin arma3_tool -- --config scan_config.json report 