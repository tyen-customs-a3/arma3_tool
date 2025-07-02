use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

/// File system utilities
pub struct FileUtils;

impl FileUtils {
    /// Recursively find all config files in a directory
    pub fn find_config_files(root: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        Self::find_config_files_recursive(root, &mut files)?;
        Ok(files)
    }

    fn find_config_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                Self::find_config_files_recursive(&path, files)?;
            } else if Self::is_config_file(&path) {
                files.push(path);
            }
        }

        Ok(())
    }

    /// Check if a file is a configuration file
    pub fn is_config_file(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|s| s.to_str()).unwrap_or(""),
            "cpp" | "hpp" | "h"
        )
    }

    /// Get relative path from base directory
    pub fn get_relative_path(base: &Path, target: &Path) -> Option<PathBuf> {
        target.strip_prefix(base).ok().map(|p| p.to_path_buf())
    }

    /// Normalize path separators for consistent display
    pub fn normalize_path(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "/")
    }

    /// Create backup of a file
    pub fn create_backup(file: &Path) -> Result<PathBuf> {
        let backup_path = file.with_extension(
            format!("{}.bak", file.extension().and_then(|s| s.to_str()).unwrap_or(""))
        );
        fs::copy(file, &backup_path)?;
        Ok(backup_path)
    }

    /// Get file size in a human-readable format
    pub fn format_file_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size_f = size as f64;
        let mut unit_index = 0;

        while size_f >= 1024.0 && unit_index < UNITS.len() - 1 {
            size_f /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size_f, UNITS[unit_index])
        }
    }

    /// Check if directory contains config files
    pub fn contains_config_files(dir: &Path) -> Result<bool> {
        if !dir.is_dir() {
            return Ok(false);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && Self::is_config_file(&path) {
                return Ok(true);
            } else if path.is_dir() && Self::contains_config_files(&path)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Get directory statistics
    pub fn get_directory_stats(dir: &Path) -> Result<DirectoryStats> {
        let mut stats = DirectoryStats::default();
        Self::collect_directory_stats(dir, &mut stats)?;
        Ok(stats)
    }

    fn collect_directory_stats(dir: &Path, stats: &mut DirectoryStats) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                stats.directories += 1;
                Self::collect_directory_stats(&path, stats)?;
            } else if path.is_file() {
                stats.files += 1;
                if Self::is_config_file(&path) {
                    stats.config_files += 1;
                }
                if let Ok(metadata) = fs::metadata(&path) {
                    stats.total_size += metadata.len();
                }
            }
        }
        Ok(())
    }
}

/// Directory statistics
#[derive(Debug, Default, Clone)]
pub struct DirectoryStats {
    pub files: usize,
    pub config_files: usize,
    pub directories: usize,
    pub total_size: u64,
}

impl DirectoryStats {
    pub fn summary(&self) -> String {
        format!(
            "Files: {}, Config files: {}, Directories: {}, Total size: {}",
            self.files,
            self.config_files,
            self.directories,
            FileUtils::format_file_size(self.total_size)
        )
    }
}
