use fs_extra::dir::{copy, CopyOptions};
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

pub fn setup_test_project() -> (TempDir, PathBuf) {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let project_root_in_temp = temp_dir.path().join("pca_test_project");

    // Path to your fixtures directory, relative to the crate root
    // CARGO_MANIFEST_DIR is the directory containing Cargo.toml of the current crate
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pca");

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true; // Copies contents of fixtures_path into project_root_in_temp

    copy(&fixtures_path, &project_root_in_temp, &options)
        .expect("Failed to copy fixtures to temp dir");

    (temp_dir, project_root_in_temp)
}
