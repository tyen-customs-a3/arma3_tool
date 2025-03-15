@echo off
echo Scanning Arma 3 Missions...
cargo run --release -- --config scan_config.json ScanMissions
echo Mission Scan Complete!
pause 