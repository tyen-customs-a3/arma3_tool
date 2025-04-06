@echo off
echo Building the CSV exporter...
cargo build --bin export_csv

echo Running the test...
cargo test --test export_csv_test -- --nocapture

if %ERRORLEVEL% == 0 (
    echo All tests passed successfully!
) else (
    echo Tests failed with error code %ERRORLEVEL%
)

echo.
echo Creating a sample export with test data...
cargo run --bin export_csv -- --database "D:\pca\git\dep\rs\arma3_tool\cache\pca_next\arma3.db" --output "class_export.csv"

if %ERRORLEVEL% == 0 (
    echo Sample export completed successfully!
    echo Output written to class_export.csv
) else (
    echo Sample export failed with error code %ERRORLEVEL%
) 