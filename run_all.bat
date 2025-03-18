@echo off
set RUST_LOG=info
cargo run --release --bin arma3tool_cli -- --config scan_config.json all 