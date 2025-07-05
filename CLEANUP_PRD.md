# Legacy Code Cleanup PRD

## Overview
This PRD outlines the cleanup tasks needed to complete the codebase restructure from the current feature branch. The pull request has successfully migrated from a flat `libs/` structure to a layered `crates/` architecture, but legacy code remains that needs removal.

## Priority Tasks (High Impact)

### 1. Remove Legacy Workspace Dependencies
**File:** `/home/tyen/git/arma3_tool/Cargo.toml:124-138`
**Impact:** Critical - these aliases are blocking the transition to the new architecture
**Task:** Delete the entire `# Legacy aliases for transition period` section

### 2. Update PropertyValue References
**Files:** 
- `crates/parsers/hpp/tests/integration_validation.rs:238`
- `crates/core/gamedata-models/src/lib.rs:103`
**Impact:** High - ensures consistency with new type system
**Task:** Replace all `PropertyValue::Class` with `PropertyValue::Object`

## Medium Priority Tasks

### 3. Remove HEMTT Library References
**Files:**
- `crates/services/scan-gamedata/Cargo.toml`
- `crates/apps/scanner-tools/Cargo.toml`
**Impact:** Medium - cleanup commented-out dependencies
**Task:** Remove commented HEMTT library references

### 4. Update Import Paths
**Files:** 10 files with legacy `parser_hpp::` imports
**Impact:** Medium - modernize import structure
**Task:** Update all legacy import patterns to use new crate structure

## Low Priority Tasks

### 5. Dead Code Review
**Files:**
- `crates/services/scan-weapons/src/database/mod.rs`
- `crates/parsers/hpp/src/workspace_manager.rs`
- `crates/infra/extract/src/processor.rs`
**Impact:** Low - code quality improvement
**Task:** Review `#[allow(dead_code)]` annotations and remove unused code

### 6. Configuration Cleanup
**Files:** Various Cargo.toml files
**Impact:** Low - maintainability
**Task:** Remove commented-out dependency sections

### 7. Migration Code Evaluation
**File:** `crates/core/types/src/migration.rs`
**Impact:** Low - post-transition cleanup
**Task:** Evaluate if migration utilities can be removed

## Success Criteria
- [ ] All legacy workspace dependencies removed
- [ ] All `PropertyValue::Class` references updated
- [ ] All commented-out HEMTT references removed
- [ ] All legacy imports modernized
- [ ] Dead code annotations reviewed and cleaned
- [ ] Codebase compiles without warnings
- [ ] All tests pass

## Execution Order
1. Execute high-priority tasks first (workspace dependencies and PropertyValue updates)
2. Proceed with medium-priority tasks (HEMTT references and imports)
3. Complete low-priority tasks (dead code and configuration cleanup)

## Task List
1. Remove legacy workspace dependencies (lines 124-138) from root Cargo.toml
2. Update PropertyValue::Class to PropertyValue::Object in hpp/tests/integration_validation.rs:238
3. Update PropertyValue::Class to PropertyValue::Object in core/gamedata-models/src/lib.rs:103
4. Remove commented-out HEMTT library references from services/scan-gamedata/Cargo.toml
5. Remove commented-out HEMTT library references from apps/scanner-tools/Cargo.toml
6. Update legacy parser_hpp:: imports across 10 identified files
7. Review and remove dead code annotations in scan-weapons/src/database/mod.rs
8. Review and remove dead code annotations in hpp/src/workspace_manager.rs
9. Review and remove dead code annotations in infra/extract/src/processor.rs
10. Remove commented-out dependency sections from Cargo.toml files
11. Evaluate if core/types/src/migration.rs can be removed (post-transition)