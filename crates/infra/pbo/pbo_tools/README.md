# PBO Tools RS

A Rust library and CLI tool for working with PBO (Packed Binary Object) files. This toolkit provides native Rust functionality for listing, extracting, and managing PBO files using HEMTT's PBO crate, with robust error handling and async operations.

## Features

- **Native Rust Implementation**: No external dependencies required
- **Async Operations**: Modern async/await support for better performance
- **Cross-Platform**: Works on Linux, macOS, and Windows
- List contents of PBO files with detailed metadata
- Extract files with pattern filtering support
- PBO properties and validation
- Comprehensive error handling and logging
- Configurable timeouts and retry mechanisms

## Prerequisites

- Rust toolchain installed (1.70+)
- No external tools required - pure Rust implementation!

## Installation

To install the CLI tool:

```bash
cargo install pbo_tools
```

## Usage

### Command Line Interface

List contents of a PBO file:
```bash
pbo_tools list path/to/file.pbo
```

Extract files from a PBO:
```bash
pbo_tools extract path/to/file.pbo output/dir
```

Show PBO properties and metadata:
```bash
pbo_tools properties path/to/file.pbo
```

Validate PBO integrity:
```bash
pbo_tools validate path/to/file.pbo
```

Options:
- `--verbose` - Enable verbose output with detailed file information
- `--filter` - Extract specific files (supports glob patterns like *.cpp)
- `--timeout` - Set operation timeout in seconds

### Library Usage

Basic usage with async operations:
```rust
use pbo_tools::core::PboApi;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PboApi::new();
    
    // List contents
    let pbo_path = Path::new("mission.pbo");
    let files = api.list_contents(&pbo_path).await?;
    println!("Files in PBO: {} files", files.len());
    
    // Extract specific files with pattern filtering
    let output_dir = Path::new("output");
    api.extract_filtered(&pbo_path, "*.cpp", &output_dir).await?;
    
    // Get PBO properties
    let properties = api.get_properties(&pbo_path).await?;
    println!("PBO Version: {:?}", properties.version);
    
    // Validate PBO integrity
    let validation = api.validate_pbo(&pbo_path).await?;
    println!("PBO is valid: {}", validation.is_valid());
    
    Ok(())
}
```

Using the PBO operations trait directly:
```rust
use pbo_tools::ops::{HemttPboOperations, PboOperations};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops = HemttPboOperations::new();
    
    // List contents
    let pbo_path = Path::new("mission.pbo");
    let files = ops.list_contents(&pbo_path).await?;
    
    // Extract all files
    let output_dir = Path::new("output");
    ops.extract_all(&pbo_path, &output_dir).await?;
    
    Ok(())
}
```

## Project Structure

- `src/cli` - Command-line interface implementation
- `src/core` - Core API and PBO operations
- `src/ops` - Modern PBO operations using HEMTT backend
  - `types` - Data structures for PBO operations
  - `error` - Error handling for PBO operations
  - `traits` - Async trait definitions
  - `hemtt_backend` - HEMTT PBO implementation
- `src/fs` - File system operations
- `tests` - Integration and unit tests

## Error Handling

The library uses comprehensive error types for different scenarios:
- `PboOperationError` - Main error enum for PBO operations
- `PboError` - Legacy error support for backward compatibility
- `FileSystemError` - File system operation errors

All operations return a `Result` type for proper error handling with detailed error messages and context.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under MIT