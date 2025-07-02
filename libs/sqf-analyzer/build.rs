use std::env;
use std::path::Path;

fn main() {
    // Get the output directory for the build
    let _out_dir = env::var("OUT_DIR").unwrap();
    
    // Get the manifest directory (where Cargo.toml is)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Source binary path
    let sqfvm_src = Path::new(&manifest_dir).join("bin").join("sqfvm.exe");
    
    // Print the location of the binary for debugging
    println!("cargo:warning=Looking for sqfvm.exe at: {}", sqfvm_src.display());
    
    // Ensure the binary exists
    if !sqfvm_src.exists() {
        panic!("sqfvm.exe not found in bin directory. Please ensure it exists at: {}", sqfvm_src.display());
    }
    
    // Tell Cargo that if the binary changes, to rerun this build script
    println!("cargo:rerun-if-changed=bin/sqfvm.exe");
    
    // Print the binary location for use in the main code
    println!("cargo:rustc-env=SQFVM_BIN_PATH={}", sqfvm_src.to_str().unwrap());
} 