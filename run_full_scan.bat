@echo off
echo Running Arma 3 Tool Full Scan...
cargo run --release -- --config scan_config.json run-all
echo Scan Complete!
pause 