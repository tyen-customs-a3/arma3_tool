use std::path::Path;
use std::fs;

pub fn ensure_dir_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn write_report(path: &Path, filename: &str, content: &str) -> std::io::Result<()> {
    ensure_dir_exists(path)?;
    fs::write(path.join(filename), content)
} 