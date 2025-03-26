@echo off
set RUST_LOG=info
echo Creating cache directory structure...
mkdir "./cache/pca_next" 2>nul

echo Processing data to create cache file...
cargo run --release --bin arma3tool_cli -- --config scan_config.json process --cache-dir "./cache/pca_next"

echo Generating report from cache...
cargo run --release --bin arma3tool_cli -- --config scan_config.json report --cache-dir "./cache/pca_next"
