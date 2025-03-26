@echo off
set RUST_LOG=info
cargo run --release --bin arma3tool_cli -- fuzzy-search --cache-dir ./cache/pca_next --output-dir ./reports --threshold 0.8