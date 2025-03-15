@echo off
echo Generating Arma 3 Reports...
cargo run --release -- --config scan_config.json generate-reports --scan-game-data --scan-missions --skip-extraction
echo Report Generation Complete!
pause 