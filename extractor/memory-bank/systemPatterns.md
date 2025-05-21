# System Patterns

## Architecture Overview

The ARMA3 Extractor is built around a layered architecture with clear separation of concerns:

```
┌───────────────────────────────┐
│         Public API            │
│   (extract_game_data, etc.)   │
└───────────────┬───────────────┘
                │
┌───────────────▼───────────────┐
│     ExtractionManager         │
│  (Orchestrates operations)    │
└─┬───────────────────────────┬─┘
  │                           │
┌─▼───────────────┐    ┌──────▼────────┐
│   PboScanner    │    │  PboProcessor │
│  (Finds PBOs)   │    │ (Extracts)    │
└─────────────────┘    └──────┬────────┘
                              │
┌────────────────────┐  ┌─────▼──────────┐
│     Models         │  │  FileDbManager │
│ (Data structures)  │  │(Cache tracking)│
└────────────────────┘  └────────────────┘
```

## Core Design Patterns

1. **Manager Pattern**: The `ExtractionManager` acts as the central coordinator, delegating specialized tasks to other components.

2. **Repository Pattern**: The `FileDbManager` provides an abstraction layer over the file-based database operations, handling all persistence concerns. It loads the database on initialization and offers explicit `save` and `load` methods for external control over persistence.

3. **Factory Methods**: Used for creating PBO metadata and records (`create_metadata`, `create_pbo_record`).

4. **Builder Pattern**: Used in `ExtractionConfig` to construct configuration objects with sensible defaults.

5. **Command Pattern**: Extraction operations are encapsulated as asynchronous tasks that can be executed independently.

6. **Strategy Pattern**: Different extraction strategies for game data vs. mission PBOs.

7. **Error Handling**: Comprehensive error types using the `thiserror` crate with custom error variants.

8. **Serialization Pattern**: Using serde for JSON serialization of database records.

9. **Post-Extraction Filtering Pattern**: Implemented within `PboProcessor` to reconcile discrepancies between the filtering capabilities/behavior of the external `pbo_tools` and the desired filtering outcome based on user configuration. This involves:
    - Optionally augmenting the filter passed to `pbo_tools` (e.g., adding `bin` if `cpp` is requested).
    - Performing a secondary filtering pass on the files returned by `pbo_tools` based on the user's *original* extension list.
    - Handling specific file renaming logic (e.g., `config.bin` -> `config.cpp`) during the secondary pass.

## Data Flow

1. **Scanning Flow**:
   - `ExtractionManager` requests PBO scanning via `PboScanner`
   - `PboScanner` searches directories and returns found PBOs
   - `ExtractionManager` filters PBOs that need extraction

2. **Extraction Flow**:
   - `ExtractionManager` passes PBOs to `PboProcessor`
   - `PboProcessor` potentially augments the extension filter before calling `pbo_tools`.
   - `PboProcessor` calls `pbo_tools` to extract files matching the (potentially augmented) filter to a temporary location.
   - `PboProcessor` catalogs files from the temporary location.
   - `PboProcessor` applies post-extraction filtering based on the *original* user extensions, handling `config.bin` renaming if necessary.
   - `PboProcessor` copies the final filtered/renamed files to the cache.
   - Extracted file metadata (using final relative paths) is returned to `ExtractionManager`.
   - `ExtractionManager` updates the file database via `FileDbManager`.

3. **Query Flow**:
   - Client code queries `ExtractionManager` or uses the convenient `find_source_pbo` function
   - `ExtractionManager` delegates to `FileDbManager`
   - `FileDbManager` performs operations on the in-memory representation of the file database
   - Results are returned through the chain

## Key Implementation Patterns

1. **Async/Await**: Used for I/O-bound operations, particularly in the extraction process.

2. **Parallel Processing**: Using rayon for CPU-bound tasks like file scanning and extraction.

3. **Error Propagation**: Using the `?` operator for clean error handling throughout the codebase.

4. **Immutable Data**: Preference for immutable data structures where possible.

5. **Path Normalization**: Converting between absolute and relative paths as needed.

6. **JSON Serialization**: Using serde JSON for storing and retrieving file database records.

7. **Safe Resource Management**: Using Rust's ownership system to ensure proper cleanup.

## Performance Patterns

1. **Selective Extraction**: Only extracting files with specific extensions, enforced by both `pbo_tools` pre-filter and `PboProcessor` post-filter.

2. **Incremental Processing**: Only processing PBOs that have changed.

3. **Parallel Execution**: Using thread pools for concurrent operations.

4. **Efficient Path Handling**: Careful management of path operations to avoid unnecessary string conversions.

5. **In-Memory Database**: Loading the entire file database into memory for fast access, with periodic persistence to disk.

6. **Extension Sorting**: Sorting extension lists to ensure that order doesn't affect extraction decisions.

## Testing Patterns

1. **Unit Tests**: Most modules include tests for their specific functionality. Increased coverage recently for `FileDbManager`.

2. **Integration Tests**: End-to-end tests for extraction workflows.

3. **Temporary Directories**: Using `tempfile` for test isolation.

4. **Test Fixtures**: Example PBOs (`headgear_pumpkin.pbo`, `adv48_Joust.VR.pbo`) and directories for testing specific extraction scenarios, including edge cases like `config.bin` renaming.

5. **Mutable vs. Immutable API**: Careful control of mutability in the API to support testing. 