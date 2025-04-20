# Technical Context

## Environment

- **Language**: Rust 1.70+
- **Build System**: Cargo
- **Platform Support**: Windows, Linux, macOS
- **Game Version**: Arma 3 (Latest Stable)

## Core Dependencies

1. **pbo_tools**: Custom library for PBO file manipulation
   - Provides functions to read, extract, and process PBO files
   - Usage: `pbo_tools::extract_pbo(path, target_dir)`
   - **Behavior Note**: Applies extension filtering (`-filter` option) *before* performing internal file processing like renaming `config.bin` to `config.cpp`. This requires a workaround in `PboProcessor`.

2. **tokio**: Async runtime
   - Used for async file I/O and concurrent operations
   - Key features: `fs`, `task`, `sync`
   - Usage: `tokio::fs::read_dir`, `tokio::spawn`

3. **rayon**: Parallel computation library
   - Enables parallel processing of PBOs
   - Usage: `par_iter()` for parallel scanning and extraction

4. **serde**: Serialization/deserialization
   - Used for config files and storing file database records
   - Usage: `#[derive(Serialize, Deserialize)]` for model objects

5. **thiserror**: Error handling
   - Provides ergonomic error definitions
   - Usage: Custom `Error` enum with error variants

6. **log**: Logging facade
   - Used for consistent logging across components
   - Paired with `env_logger` for development
   - Usage: `log::info!`, `log::error!`

7. **tempfile**: (Dev dependency)
   - Used for creating temporary test directories
   - Usage: `tempfile::tempdir()` in tests

## File Database Implementation

- **Storage Format**: JSON files
- **Storage Location**: Configurable, defaults to user's home directory
- **Structure**:
  - One main index file
  - Records indexed by file path
  - Metadata includes hash, size, timestamp, source PBO
- **Operations**:
  - Query by file path
  - Query by source PBO
  - Update metadata
  - Purge old records

## Build Configuration

- **Features**:
  - `full`: All supported features
  - `minimal`: Core extraction only, no database
  - `progress`: Progress reporting (optional)
  
- **Dependencies Management**:
  - Pinned dependency versions for stability
  - Dev dependencies separated in Cargo.toml
  - Feature-gated optional dependencies

## Performance Considerations

1. **Memory Usage**:
   - File database loaded entirely into memory
   - Configurable maximum memory limit
   - Periodic flushing to disk

2. **CPU Utilization**:
   - Parallel extraction on multi-core systems
   - Configurable thread limit
   - Work stealing via rayon

3. **Disk I/O**:
   - Batch operations where possible
   - Async file operations
   - Buffer sizes optimized for PBO reading
   - **Post-Extraction Filtering**: The `PboProcessor` performs a second filtering pass after `pbo_tools` extracts to a temporary directory. This adds a slight overhead but ensures correct filtering according to user config, handling quirks like `config.bin` renaming. The impact is generally negligible compared to the extraction itself.

4. **Error Handling**:
   - Graceful degradation on errors
   - Detailed error reporting
   - Recovery mechanisms for interrupted operations

## Testing Strategy

1. **Unit Tests**: For core logic and algorithms
2. **Integration Tests**: For extraction workflow
3. **Property Tests**: For robustness against varied inputs
4. **Performance Tests**: For ensuring extraction speed
5. **Mock Tests**: For simulating file system and external dependencies 