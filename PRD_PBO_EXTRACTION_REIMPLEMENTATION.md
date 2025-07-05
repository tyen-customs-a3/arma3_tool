# Product Requirements Document: PBO Extraction Reimplementation

## 1. Overview

### 1.1 Purpose
This PRD outlines the requirements and implementation plan for restoring PBO (Packed Bohemia Object) extraction functionality in the refactored Arma3 tool codebase. The extraction capability was removed during the architectural refactoring and must be reimplemented to restore core functionality.

### 1.2 Background
- **Current State**: PBO extraction is stubbed out with TODO comments in `crates/infra/extract/src/processor.rs`
- **Previous Implementation**: Used `pbo_tools` crate with working extraction to temporary directories
- **New Architecture**: Should use `arma3-pbo` crate with HEMTT backend for modern Rust-native implementation

### 1.3 Goals
1. Restore full PBO extraction capability
2. Maintain backwards compatibility with existing interfaces
3. Leverage the new HEMTT-based PBO handling
4. Ensure all tests pass and workflows function end-to-end

## 2. Technical Requirements

### 2.1 Core Functionality to Implement

#### 2.1.1 PBO Extraction Engine
**Location**: `crates/infra/extract/src/processor.rs`

**Required Implementation**:
```rust
// Replace lines 133-143 with actual implementation
// Current TODO block:
// TODO: Implement PBO extraction when PBO API is available
// For now, just log and return empty results

// Should become:
let pbo_api = arma3_pbo::PboApi::new(timeout_seconds);
let extraction_result = pbo_api.extract_filtered(
    &pbo_path,
    &temp_dir_path,
    &filter_pattern
).await?;
```

#### 2.1.2 Integration Points
1. **PBO API Creation**: Initialize the PBO processor with appropriate timeout
2. **Extraction Logic**: Implement filtered extraction based on file extensions
3. **Binary Conversion**: Handle config.bin → config.cpp conversion
4. **Error Handling**: Proper error propagation and retry logic
5. **Resource Cleanup**: Ensure temporary directories are cleaned up

### 2.2 Detailed Implementation Steps

#### Step 1: Add PBO Crate Dependency
```toml
# In crates/infra/extract/Cargo.toml
[dependencies]
arma3-pbo = { path = "../pbo" }
```

#### Step 2: Implement PBO API Wrapper
Create a new module in `crates/infra/extract/src/pbo_interface.rs`:

```rust
use arma3_pbo::{PboApi, PboError};
use std::path::{Path, PathBuf};
use async_trait::async_trait;

#[async_trait]
pub trait PboExtractor: Send + Sync {
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboError>;
    
    async fn convert_binary_configs(
        &self,
        extracted_files: &[PathBuf],
    ) -> Result<Vec<PathBuf>, PboError>;
}

pub struct HemttPboExtractor {
    timeout_seconds: u64,
}

impl HemttPboExtractor {
    pub fn new(timeout_seconds: u64) -> Self {
        Self { timeout_seconds }
    }
}

#[async_trait]
impl PboExtractor for HemttPboExtractor {
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboError> {
        // Implementation using arma3-pbo crate
        todo!()
    }
    
    async fn convert_binary_configs(
        &self,
        extracted_files: &[PathBuf],
    ) -> Result<Vec<PathBuf>, PboError> {
        // Convert config.bin to config.cpp
        todo!()
    }
}
```

#### Step 3: Update Processor Implementation
In `crates/infra/extract/src/processor.rs`, update the extraction logic:

```rust
// Around line 86, add:
let pbo_extractor = Arc::new(HemttPboExtractor::new(60));

// Replace lines 133-143:
let extraction_result = pbo_extractor.extract_filtered(
    &pbo_path,
    temp_dir.path(),
    &filter_pattern
).await;

match extraction_result {
    Ok(mut extracted_files) => {
        // Handle binary config conversion if needed
        if user_wants_cpp {
            let converted_files = pbo_extractor
                .convert_binary_configs(&extracted_files)
                .await
                .unwrap_or_else(|e| {
                    warn!("Failed to convert binary configs: {}", e);
                    Vec::new()
                });
            extracted_files.extend(converted_files);
        }
        
        // Continue with existing post-extraction logic...
    }
    Err(e) => {
        record_failure(format!("Failed to extract PBO {}: {}", 
            pbo_path.display(), e));
        return;
    }
}
```

### 2.3 Binary Config Conversion

#### Requirements:
1. Detect `config.bin` files in extracted content
2. Convert to `config.cpp` using HEMTT's debinarize functionality
3. Handle conversion errors gracefully
4. Update file lists to include converted files

#### Implementation:
```rust
async fn convert_binary_config(
    bin_path: &Path,
    cpp_path: &Path,
) -> Result<(), PboError> {
    use arma3_pbo::config::debinarize;
    
    let bin_content = tokio::fs::read(bin_path).await?;
    let cpp_content = debinarize(&bin_content)?;
    tokio::fs::write(cpp_path, cpp_content).await?;
    
    Ok(())
}
```

### 2.4 Error Handling and Retry Logic

#### Requirements:
1. Implement exponential backoff for transient failures
2. Distinguish between recoverable and non-recoverable errors
3. Maintain failure tracking for reporting
4. Ensure partial extractions are cleaned up

#### Implementation:
```rust
const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF_MS: u64 = 100;

async fn extract_with_retry(
    extractor: &dyn PboExtractor,
    pbo_path: &Path,
    output_dir: &Path,
    filter: &str,
) -> Result<Vec<PathBuf>, PboError> {
    let mut backoff = INITIAL_BACKOFF_MS;
    
    for attempt in 0..MAX_RETRIES {
        match extractor.extract_filtered(pbo_path, output_dir, filter).await {
            Ok(files) => return Ok(files),
            Err(e) if is_transient_error(&e) && attempt < MAX_RETRIES - 1 => {
                warn!("Extraction attempt {} failed, retrying: {}", attempt + 1, e);
                tokio::time::sleep(Duration::from_millis(backoff)).await;
                backoff *= 2;
            }
            Err(e) => return Err(e),
        }
    }
    
    unreachable!()
}

fn is_transient_error(error: &PboError) -> bool {
    // Define which errors are worth retrying
    matches!(error, PboError::IoError(_) | PboError::Timeout)
}
```

### 2.5 Testing Requirements

#### Unit Tests:
1. Test extraction with various filter patterns
2. Test binary config conversion
3. Test error handling and retry logic
4. Test resource cleanup

#### Integration Tests:
1. Test end-to-end extraction of fixture PBOs
2. Verify extracted file contents
3. Test parallel extraction performance
4. Test cache behavior

#### Test Fixtures:
- Use existing test PBOs in `crates/infra/extract/tests/fixtures/`
- Ensure tests cover both mission and mod PBOs
- Include PBOs with binary configs

## 3. Implementation Plan

### Phase 1: Foundation (Days 1-2)
1. Set up PBO crate dependencies
2. Create PBO extractor interface
3. Implement basic extraction without filtering
4. Get minimal tests passing

### Phase 2: Core Features (Days 3-4)
1. Implement file filtering
2. Add binary config conversion
3. Implement error handling and retries
4. Update all integration points

### Phase 3: Testing & Polish (Days 5-6)
1. Fix all failing tests
2. Add comprehensive test coverage
3. Performance optimization
4. Documentation updates

### Phase 4: Validation (Day 7)
1. End-to-end workflow testing
2. Performance benchmarking
3. Memory usage profiling
4. Final code review

## 4. Acceptance Criteria

### Functional Requirements:
- [ ] All PBO extraction tests pass
- [ ] Binary config conversion works correctly
- [ ] File filtering works as before
- [ ] Parallel extraction maintains performance
- [ ] Error handling provides clear feedback
- [ ] Resource cleanup prevents disk space issues

### Performance Requirements:
- [ ] Extraction speed comparable to previous implementation
- [ ] Memory usage remains bounded for large PBOs
- [ ] Parallel extraction scales with CPU cores
- [ ] No file handle leaks

### Integration Requirements:
- [ ] Workflow commands process PBOs successfully
- [ ] Mission scanning works with PBO files
- [ ] Weapon scanning extracts from mod PBOs
- [ ] Cache system correctly tracks extracted files

## 5. Technical Considerations

### 5.1 HEMTT Integration
- Leverage HEMTT's native Rust implementation for better performance
- Use HEMTT's config debinarization for binary configs
- Ensure compatibility with HEMTT's error types

### 5.2 Backwards Compatibility
- Maintain existing `ExtractorInterface` trait API
- Preserve extraction result format
- Keep cache directory structure unchanged

### 5.3 Future Enhancements
- Streaming extraction for large files
- Incremental extraction updates
- Compression analysis and optimization
- Parallel binary config conversion

## 6. Risk Mitigation

### Technical Risks:
1. **HEMTT API Changes**: Pin to specific version, add compatibility layer
2. **Memory Usage**: Implement streaming for large files
3. **Platform Differences**: Test on Windows, Linux, macOS

### Implementation Risks:
1. **Scope Creep**: Focus on restoration first, enhancements later
2. **Test Coverage**: Use existing test suite as baseline
3. **Performance Regression**: Benchmark against original implementation

## 7. Success Metrics

1. **Test Coverage**: 100% of existing extraction tests pass
2. **Performance**: Extraction speed within 10% of original
3. **Reliability**: <0.1% extraction failure rate
4. **Memory**: Peak memory usage <500MB for typical PBOs

## 8. Documentation Requirements

1. Update API documentation for PBO extractor
2. Add examples for common extraction scenarios
3. Document error types and recovery strategies
4. Create migration guide for API changes