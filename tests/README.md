# Arma 3 Tool Tests

This directory contains tests for the Arma 3 Tool codebase. The tests are organized into separate modules, each testing a specific part of the codebase.

## Test Structure

- `config_tests.rs` - Tests for the configuration module
- `cache_tests.rs` - Tests for the cache management module
- `scanner_tests.rs` - Tests for the scanner modules (game data, mission, report)
- `error_tests.rs` - Tests for the error handling module
- `integration_tests.rs` - Integration tests that test multiple components together
- `cli_tests.rs` - Tests for the command-line interface
- `config_with_paths_test.rs` - Tests for loading the actual scan_config.json file
- `scan_with_config_test.rs` - Tests for scanner initialization with the actual config

## Test Fixtures

The `fixtures` directory contains sample data for testing:

- `gamedata/` - Sample game data PBOs
  - `@tc_headgear_pumpkin/` - Sample mod with PBO files
  - `@tc_headgear_reindeernose/` - Sample mod with PBO files
- `missions/` - Sample mission PBOs
  - `adv48_Joust.VR.pbo` - Sample mission PBO
  - `adv70_RATSvSNAKES_v1a.Tembelan.pbo` - Sample mission PBO

## Running Tests

To run all tests:

```bash
cargo test --test '*'
```

To run a specific test module:

```bash
cargo test --test config_tests
```

To run a specific test:

```bash
cargo test --test config_tests test_load_valid_config
```

To run ignored tests (like CLI tests that require the binary to be built):

```bash
cargo test --test cli_tests -- --ignored
```

## Test Results

All tests are currently passing:

- **Config Tests**: 4 tests passing
  - `test_load_valid_config`
  - `test_default_values`
  - `test_invalid_config_path`
  - `test_invalid_config_json`

- **Cache Tests**: 4 tests passing
  - `test_cache_path_generation`
  - `test_mission_cache_path_generation`
  - `test_cache_update_and_check`
  - `test_clear_cache`

- **Scanner Tests**: 2 tests passing
  - `test_scanner_creation`
  - `test_report_generator_creation`

- **Error Tests**: 4 tests passing
  - `test_error_conversion`
  - `test_anyhow_conversion`
  - `test_error_display`
  - `test_error_clone`

- **Integration Tests**: 2 tests passing
  - `test_full_pipeline_setup`
  - `test_cache_directory_creation`

- **Config With Paths Test**: 1 test passing
  - `test_load_real_config`

- **Scan With Config Test**: 1 test passing
  - `test_scanner_initialization_with_config`

- **CLI Tests**: 2 tests ignored (require binary to be built)
  - `test_cli_help`
  - `test_cli_scan_gamedata`

## Adding New Tests

When adding new tests:

1. Add the test to the appropriate test module
2. If testing a new component, create a new test module
3. Add any necessary fixtures to the `fixtures` directory
4. Update this README if necessary

## Test Coverage

The tests aim to cover:

- Configuration loading and validation
- Cache management (creation, checking, updating)
- PBO discovery and extraction
- Mission and game data scanning
- Report generation
- Error handling
- Command-line interface 