@echo off
set RUST_LOG=info

REM Set configuration variables here - edit these instead of individual commands
set CONFIG_FILE=scan_config.json
set CACHE_DIR_SOURCE=./cache/pca
set CACHE_DIR_TARGET=./cache/pca_next
set OUTPUT_DIR=./reports
set FUZZY_THRESHOLD=0.8

echo Using configuration:
echo   Config file: %CONFIG_FILE%
echo   Source cache: %CACHE_DIR_SOURCE%
echo   Target cache: %CACHE_DIR_TARGET%
echo   Output directory: %OUTPUT_DIR%
echo   Fuzzy match threshold: %FUZZY_THRESHOLD%

echo Creating cache directory structure...
mkdir "%CACHE_DIR_TARGET%" 2>nul
mkdir "%OUTPUT_DIR%" 2>nul

echo Generating dependency report from cache...
cargo run --release --bin arma3tool_cli -- --config %CONFIG_FILE% report --cache-dir "%CACHE_DIR_TARGET%" --output-dir "%OUTPUT_DIR%"

echo Generating inheritance visualization...
cargo run --release --bin arma3tool_cli -- --config %CONFIG_FILE% graph --cache-dir "%CACHE_DIR_TARGET%" --output-dir "%OUTPUT_DIR%"

@REM echo Generating fuzzy search report...
@REM cargo run --release --bin arma3tool_cli -- --config %CONFIG_FILE% fuzzy-search --cache-dir "%CACHE_DIR_TARGET%" --output-dir "%OUTPUT_DIR%" --threshold %FUZZY_THRESHOLD%

@REM echo Generating comparison report...
@REM cargo run --release --bin arma3tool_cli -- --config %CONFIG_FILE% compare --cache-dir-a "%CACHE_DIR_SOURCE%" --cache-dir-b "%CACHE_DIR_TARGET%" --output-dir "%OUTPUT_DIR%"

echo Reports generated in %OUTPUT_DIR%