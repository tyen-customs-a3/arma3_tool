@echo off
set RUST_LOG=info
cargo run --release --bin arma3tool_ui -- -d cache/arma3.db