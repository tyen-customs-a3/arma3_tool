@echo off
set RUST_LOG=info
cargo run --release --bin arma3tool_cli -- compare --cache-dir-a ./cache/pca --cache-dir-b ./cache/pca_next --output-dir ./reports