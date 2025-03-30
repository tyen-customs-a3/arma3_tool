@echo off
set RUST_LOG=info

if not exist "cache/pca_next/arma3.db" (
    echo Error: Database file not found at cache\pca_next\arma3.db
    echo Please run the scanner first to generate the database
    pause
    exit /b 1
)

cargo run --release --bin arma3tool_web -- -d cache/pca_next/arma3.db 