# Pull Request Review: feature/clean-batch-restructure

## Overview
This is a major refactoring that reorganizes the codebase from a `libs/` structure to a more modular `crates/` structure following Rust conventions. The refactor introduces a unified type system and cleaner architecture.

## ✅ Preserved Functionality
1. **Type System**: Successfully migrated to a unified type system with backwards compatibility through migration utilities
2. **Parsers**: All parser functionality (HPP, SQF, SQM) remains intact with namespace updates
3. **Service Layer**: Scanning capabilities for weapons, missions, and gamedata preserved
4. **Configuration**: Item filter and scan configurations moved but maintained
5. **CLI**: New unified CLI preserves all command functionality through subcommands
6. **Database Operations**: All database functionality remains intact

## ⚠️ Architecture Improvements
1. **Better Organization**: Clear separation into core, infra, services, parsers, and apps layers
2. **Unified Types**: New `arma3-types` crate eliminates type conversions between modules
3. **Modern Patterns**: Trait-based interfaces, better error handling, async support
4. **HEMTT Integration**: Modern PBO handling through native Rust implementation

## 🔴 Critical Missing Functionality

### PBO Extraction is Completely Broken
- The core extraction logic in `crates/infra/extract/src/processor.rs` is stubbed out
- Lines 133-143 show TODOs instead of actual extraction implementation:
  ```rust
  // TODO: Implement PBO extraction when PBO API is available
  // For now, just log and return empty results
  info!("Would extract PBO {} with filter {}", pbo_path.display(), filter_pattern);
  let extracted_files: Vec<PathBuf> = Vec::new(); // Empty for now
  ```
- Tests confirm this: extraction tests are failing because no files are extracted
- This is a **major loss of functionality** - the tool cannot extract PBO files

## Impact Assessment
Without PBO extraction working:
- Workflow commands will fail when processing PBO files
- Mission scanning of PBO files won't work
- Any functionality depending on extracting content from PBOs is broken
- Only direct file scanning (non-PBO) remains functional

## Test Results
```
---- test_extract_headgear_pumpkin_fixture stdout ----
thread 'test_extract_headgear_pumpkin_fixture' panicked at crates/infra/extract/tests/integration_tests.rs:402:5:
assertion `left == right` failed: Expected 10 files, but found 0
  left: 0
 right: 10

---- test_extract_joust_mission_fixture stdout ----
thread 'test_extract_joust_mission_fixture' panicked at crates/infra/extract/tests/integration_tests.rs:476:5:
Extraction results map should not be empty
```

## Recommendation
**Do not merge this PR until PBO extraction is reimplemented**. While the architectural improvements are excellent, losing core extraction functionality makes the tool unusable for its primary purpose. The TODO comments need to be addressed by implementing the actual PBO extraction using the new `arma3-pbo` crate infrastructure.

## Required Actions Before Merge
1. Implement the PBO extraction logic in `crates/infra/extract/src/processor.rs`
2. Ensure all extraction tests pass
3. Verify end-to-end workflows work with PBO files
4. Test mission and mod scanning with actual PBO files