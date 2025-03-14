// This file is used to run all tests in the tests directory
// It's not a test file itself, but a way to organize the tests

fn main() {
    println!("Running all tests for Arma 3 Tool");
    
    // The tests are organized in separate modules
    // Each module tests a specific part of the codebase
    
    // config_tests.rs - Tests for the config module
    // cache_tests.rs - Tests for the cache module
    // scanner_tests.rs - Tests for the scanner modules
    // error_tests.rs - Tests for the error module
    // integration_tests.rs - Integration tests for multiple components
    // cli_tests.rs - Tests for the CLI functionality
    
    // To run all tests:
    // cargo test --test '*'
    
    // To run a specific test module:
    // cargo test --test config_tests
    
    // To run a specific test:
    // cargo test --test config_tests test_load_valid_config
    
    // To run ignored tests (like CLI tests that require the binary to be built):
    // cargo test --test cli_tests -- --ignored
} 