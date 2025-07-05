# ARMA 3 PBO Extractor Crate

## Overview

`arma3_extractor` is a Rust library designed to efficiently scan for, extract, and cache the contents of ARMA 3 PBO (Packed Bank of Objects) files. It provides a robust system for managing extracted game data and mission files, optimizing for performance by only extracting PBOs when necessary based on file modifications or configuration changes.

The primary goal is to provide a backend component for ARMA 3 development and modding tools that need fast and reliable access to PBO contents without repeatedly extracting large archives.

## Key Features

*   **PBO Scanning**: Recursively scans specified directories for PBO files (`.pbo`).
*   **Efficient Extraction**: Extracts PBO contents to a configurable cache location using the external `pbo_tools` library.
*   **Intelligent Caching**: Maintains a file-based database (`cache.db` by default) to track PBO metadata (path, hash, modification time, extracted files, etc.). PBOs are only re-extracted if the source PBO has changed or the extraction configuration (like filtered extensions) is different.
*   **Configurable Extension Filtering**: Allows specifying which file extensions should be extracted from PBOs (e.g., only `.sqf`, `.hpp`, `.paa`).
*   **`config.bin` Handling**: Automatically handles the common PBO quirk where `config.bin` is often intended to be used as `config.cpp`. If `cpp` is included in the extension filter, the library ensures `config.bin` is extracted and made available as `config.cpp` in the cache, while filtering out other `.bin` files unless explicitly requested.
*   **Parallel Processing**: Utilizes multiple threads (configurable) via `tokio` and potentially `rayon` (dependency details in `techContext.md`) for faster scanning and extraction.
*   **Self-Contained**: Operates without external database dependencies, using simple JSON files for its cache index.
*   **Source Tracking**: Allows looking up the source PBO for a given extracted file path (functionality details in `manager.rs` and `file_db.rs`).

## Caching and Database

The crate uses a file-based database (`cache.db` JSON file by default, located in the cache directory) to manage the cache state.

1.  **Scanning**: When an extraction is requested, the scanner finds all relevant PBOs.
2.  **Checking**: For each PBO, the `ExtractionManager` checks the database:
    *   Does metadata exist for this PBO path?
    *   Does the modification time or file hash match the stored metadata?
    *   Does the set of requested extensions match the extensions stored in the metadata?
3.  **Extraction**: If the PBO is missing from the database, has been modified, or the requested extensions differ, it is queued for extraction by the `PboProcessor`.
4.  **Filtering & Post-Filtering**:
    *   The `PboProcessor` passes the PBO path and a potentially augmented filter (including `.bin` if `.cpp` was requested) to `pbo_tools`.
    *   `pbo_tools` extracts matching files to a temporary directory.
    *   The `PboProcessor` reads the files from the temporary directory and performs a *post-extraction filter* based on the *original* user-requested extensions, handling the `config.bin` -> `config.cpp` rename if applicable.
    *   Only files passing the post-extraction filter are copied to the final cache location.
5.  **Metadata Update**: After successful extraction and filtering, the database is updated with the PBO's new metadata, including the list of *final* relative paths of the extracted files within the cache.

## Basic Usage

```rust
use arma3_extractor::{ExtractionConfig, ExtractionManager};
use std::path::PathBuf;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Define cache directory
    let cache_dir = PathBuf::from("./arma3_cache");

    // 2. Create a configuration
    let mut config = ExtractionConfig::new(cache_dir); // Sensible defaults

    // 3. Add directories to scan
    config = config.add_game_data_dir(PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Arma 3/addons"));
    config = config.add_mission_dir(PathBuf::from("C:/Users/YourUser/Documents/Arma 3 - Other Profiles/YourProfile/missions"));

    // 4. Customize extensions (optional)
    config = config.with_game_data_extensions(vec!["sqf".into(), "hpp".into(), "cpp".into(), "paa".into()]);
    config = config.with_mission_extensions(vec!["sqf".into(), "hpp".into(), "cpp".into(), "sqm".into(), "ext".into()]);

    // 5. Customize other options (optional)
    config = config.with_threads(4);
    config = config.with_verbose(true);

    // 6. Create the manager
    let mut manager = ExtractionManager::new(config)?;

    // 7. Run extraction (example: process game data)
    // These functions check the cache and only extract modified/new PBOs
    println!("Processing game data...");
    match manager.process_game_data(false).await { // Pass true to skip extraction, only scan
        Ok(extracted_files) => {
            println!("Game data processing complete. {} total files extracted/cached.", extracted_files.len());
            // Note: extracted_files contains paths to files extracted in THIS run.
            // To get all cached files, query the FileDbManager.
        }
        Err(e) => {
            eprintln!("Error processing game data: {}", e);
        }
    }

    // Example: Process missions
    // println!("Processing missions...");
    // match manager.process_all_missions(false).await {
    //     Ok(mission_map) => {
    //         println!("Mission processing complete. Processed {} missions.", mission_map.len());
    //         // mission_map is a HashMap<PathBuf (PBO path), Vec<PathBuf (cached file paths)>>
    //     }
    //     Err(e) => {
    //         eprintln!("Error processing missions: {}", e);
    //     }
    // }

    Ok(())
}
```

## Configuration

The extraction process is controlled via the `ExtractionConfig` struct, which allows setting:

*   Cache directories (`cache_dir`, `game_data_cache_dir`, `mission_cache_dir`)
*   Source directories (`game_data_dirs`, `mission_dirs`)
*   File extensions to filter (`game_data_extensions`, `mission_extensions`)
*   Number of parallel threads (`threads`)
*   Extraction timeout (`timeout`)
*   Verbosity (`verbose`)
*   Database path (`db_path`)

See `src/models.rs` for details. 