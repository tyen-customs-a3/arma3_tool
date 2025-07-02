@echo off
echo Starting Arma 3 gamedata parsing with SIMPLE parser...

:: Change to the parser code directory (where the Cargo.toml is located)
cd /d "%~dp0"

:: Create output directory structure
mkdir ".\parser_output_simple" 2>nul
mkdir ".\parser_output_simple\failing_files" 2>nul

:: Use simple parser
set PARSER_TYPE=simple

:: Run the batch parser with the game data folder
cargo run --release --bin batch_check -- ^
  --input-dir "E:\pca\git\rs\arma3_tool\cache\pcanext_2.1.0-a2\gamedata\x" ^
  --output-dir ".\parser_output_simple\failing_files" ^
  --report-path ".\parser_output_simple\report.json" ^
  --diagnostic-path ".\parser_output_simple\diagnostics.log" ^
  --file-extensions "hpp,cpp,h,c" ^
  --parser-type %PARSER_TYPE% ^
  --parallel ^
  --copy-failed-files

echo Parsing complete. Check parser_output_simple folder for results.
pause 