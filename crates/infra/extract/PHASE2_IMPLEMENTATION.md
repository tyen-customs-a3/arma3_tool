# Phase 2 Implementation: Binary Config Conversion

## Summary

This implementation adds binary config file detection and conversion functionality to the PBO extraction system.

## Changes Made

### 1. Updated PboExtractor Trait (`pbo_interface.rs`)
- Added `convert_binary_configs` method to the trait
- Implemented the method in `HemttPboExtractor`
- Added helper method `try_convert_binary_config` for actual conversion logic

### 2. Binary Config Detection
- Detects files named `config.bin` (case-insensitive)
- Checks for raP signature (`\0raP`) to identify binary configs
- Provides helper functions:
  - `is_binary_config()` - Checks if a filename is a binary config
  - `get_converted_config_name()` - Converts `config.bin` -> `config.cpp`

### 3. Conversion Process
- When user requests `.cpp` files, the system now:
  1. Extracts `.bin` files as well
  2. Detects binary configs after extraction
  3. Attempts conversion using the raP signature check
  4. Currently creates placeholder files (actual debinarize not yet available)
  5. Handles errors gracefully without failing the entire extraction

### 4. Updated Processor (`processor.rs`)
- Integrated binary conversion into the extraction workflow
- Removed old hacky config.bin renaming logic
- Now properly handles conversion results and updates file lists

### 5. Error Handling
- Conversion errors are logged as warnings
- Failed conversions don't stop the extraction process
- Original binary files are preserved if conversion fails

## Current Limitations

1. **No Actual Debinarize**: The current implementation detects binary configs but cannot actually convert them. It creates placeholder files indicating conversion is needed.

2. **HEMTT Integration**: The HEMTT crates in use don't provide debinarize functionality. Future integration would require:
   - Adding hemtt-config with debinarize support
   - Or calling an external debinarize tool
   - Or implementing raP format parsing

## Testing

Added comprehensive tests:
- `test_binary_config_detection` - Verifies config.bin detection
- `test_converted_config_name` - Tests filename conversion
- `test_binary_config_conversion` - Tests the conversion process
- `test_non_binary_file_not_converted` - Ensures text files aren't converted

## Future Improvements

1. **Actual Debinarize Implementation**:
   ```rust
   // When HEMTT provides debinarize:
   use hemtt_config::debinarize;
   
   let config = debinarize::from_bytes(&binary_data)?;
   fs::write(output_path, config.to_string())?;
   ```

2. **External Tool Integration**:
   ```rust
   // Call external debinarize tool
   Command::new("debinarize")
       .arg(binary_path)
       .arg("-o")
       .arg(output_path)
       .output()?;
   ```

3. **More Binary Formats**:
   - Support for other binarized files (rvmat, etc.)
   - Configurable binary file patterns

## Usage Example

```rust
let extractor = HemttPboExtractor::new(30);
let extracted_files = vec![PathBuf::from("config.bin")];

// Convert binary configs
let conversions = extractor.convert_binary_configs(&extracted_files, output_dir).await?;

for (original, converted) in conversions {
    println!("Converted: {} -> {}", original.display(), converted.display());
}
```