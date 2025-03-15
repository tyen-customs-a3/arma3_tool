@echo off
echo Scanning Arma 3 Game Data...
cargo run --release -- --config scan_config.json ScanGameData
echo Game Data Scan Complete!
pause 