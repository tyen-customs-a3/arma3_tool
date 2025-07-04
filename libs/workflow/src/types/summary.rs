use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};

/// Summary of extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionSummary {
    /// Number of PBOs extracted
    pub extracted_pbos: usize,
    
    /// Extraction paths
    pub extraction_paths: Vec<PathBuf>,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during extraction
    pub errors: Vec<String>,
}

impl ExtractionSummary {
    /// Create a new extraction summary
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add an extracted PBO
    pub fn add_extracted_pbo(&mut self, path: PathBuf) {
        self.extracted_pbos += 1;
        self.extraction_paths.push(path);
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    /// Set the elapsed time
    pub fn set_elapsed_time(&mut self, elapsed: Duration) {
        self.elapsed_time = elapsed;
    }
    
    /// Check if extraction was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }
    
    /// Get the success rate (extracted vs errors)
    pub fn success_rate(&self) -> f64 {
        if self.extracted_pbos == 0 && self.errors.is_empty() {
            return 1.0;
        }
        
        let total = self.extracted_pbos + self.errors.len();
        if total == 0 {
            return 1.0;
        }
        
        self.extracted_pbos as f64 / total as f64
    }
}

impl Default for ExtractionSummary {
    fn default() -> Self {
        Self {
            extracted_pbos: 0,
            extraction_paths: Vec::new(),
            elapsed_time: Duration::from_secs(0),
            errors: Vec::new(),
        }
    }
}

impl std::fmt::Display for ExtractionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extraction Summary: {} PBOs extracted in {:.2}s", 
               self.extracted_pbos, 
               self.elapsed_time.as_secs_f64())?;
        
        if !self.errors.is_empty() {
            write!(f, " with {} errors", self.errors.len())?;
        }
        
        Ok(())
    }
}

/// Summary of processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingSummary {
    /// Number of PBOs processed
    pub processed_pbos: usize,
    
    /// Number of files processed
    pub files_processed: usize,
    
    /// Number of entries found
    pub entries_found: usize,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during processing
    pub errors: Vec<String>,
}

impl ProcessingSummary {
    /// Create a new processing summary
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a processed PBO
    pub fn add_processed_pbo(&mut self) {
        self.processed_pbos += 1;
    }
    
    /// Add processed files
    pub fn add_processed_files(&mut self, count: usize) {
        self.files_processed += count;
    }
    
    /// Add found entries
    pub fn add_found_entries(&mut self, count: usize) {
        self.entries_found += count;
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    /// Set the elapsed time
    pub fn set_elapsed_time(&mut self, elapsed: Duration) {
        self.elapsed_time = elapsed;
    }
    
    /// Check if processing was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }
    
    /// Get processing rate (files per second)
    pub fn processing_rate(&self) -> f64 {
        if self.elapsed_time.as_secs_f64() == 0.0 {
            return 0.0;
        }
        
        self.files_processed as f64 / self.elapsed_time.as_secs_f64()
    }
    
    /// Get average entries per file
    pub fn avg_entries_per_file(&self) -> f64 {
        if self.files_processed == 0 {
            return 0.0;
        }
        
        self.entries_found as f64 / self.files_processed as f64
    }
}

impl Default for ProcessingSummary {
    fn default() -> Self {
        Self {
            processed_pbos: 0,
            files_processed: 0,
            entries_found: 0,
            elapsed_time: Duration::from_secs(0),
            errors: Vec::new(),
        }
    }
}

impl std::fmt::Display for ProcessingSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Processing Summary: {} PBOs, {} files, {} entries in {:.2}s", 
               self.processed_pbos, 
               self.files_processed,
               self.entries_found,
               self.elapsed_time.as_secs_f64())?;
        
        if !self.errors.is_empty() {
            write!(f, " with {} errors", self.errors.len())?;
        }
        
        Ok(())
    }
}

/// Summary of reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingSummary {
    /// Number of reports generated
    pub reports_generated: usize,
    
    /// Report paths
    pub report_paths: Vec<PathBuf>,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during reporting
    pub errors: Vec<String>,
}

/// Summary of export operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSummary {
    /// Number of exports generated
    pub exports_generated: usize,
    
    /// Export paths
    pub export_paths: Vec<PathBuf>,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during export
    pub errors: Vec<String>,
}

impl ReportingSummary {
    /// Create a new reporting summary
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a generated report
    pub fn add_generated_report(&mut self, path: PathBuf) {
        self.reports_generated += 1;
        self.report_paths.push(path);
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    /// Set the elapsed time
    pub fn set_elapsed_time(&mut self, elapsed: Duration) {
        self.elapsed_time = elapsed;
    }
    
    /// Check if reporting was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }
    
    /// Get report generation rate (reports per second)
    pub fn generation_rate(&self) -> f64 {
        if self.elapsed_time.as_secs_f64() == 0.0 {
            return 0.0;
        }
        
        self.reports_generated as f64 / self.elapsed_time.as_secs_f64()
    }
}

impl Default for ReportingSummary {
    fn default() -> Self {
        Self {
            reports_generated: 0,
            report_paths: Vec::new(),
            elapsed_time: Duration::from_secs(0),
            errors: Vec::new(),
        }
    }
}

impl std::fmt::Display for ReportingSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reporting Summary: {} reports generated in {:.2}s", 
               self.reports_generated, 
               self.elapsed_time.as_secs_f64())?;
        
        if !self.errors.is_empty() {
            write!(f, " with {} errors", self.errors.len())?;
        }
        
        Ok(())
    }
}

impl ExportSummary {
    /// Create a new export summary
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a generated export
    pub fn add_generated_export(&mut self, path: PathBuf) {
        self.exports_generated += 1;
        self.export_paths.push(path);
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    /// Set the elapsed time
    pub fn set_elapsed_time(&mut self, elapsed: Duration) {
        self.elapsed_time = elapsed;
    }
    
    /// Check if export was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }
    
    /// Get export generation rate (exports per second)
    pub fn generation_rate(&self) -> f64 {
        if self.elapsed_time.as_secs_f64() == 0.0 {
            return 0.0;
        }
        
        self.exports_generated as f64 / self.elapsed_time.as_secs_f64()
    }
}

impl Default for ExportSummary {
    fn default() -> Self {
        Self {
            exports_generated: 0,
            export_paths: Vec::new(),
            elapsed_time: Duration::from_secs(0),
            errors: Vec::new(),
        }
    }
}

impl std::fmt::Display for ExportSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Export Summary: {} exports generated in {:.2}s", 
               self.exports_generated, 
               self.elapsed_time.as_secs_f64())?;
        
        if !self.errors.is_empty() {
            write!(f, " with {} errors", self.errors.len())?;
        }
        
        Ok(())
    }
}

/// Combined summary for all workflow operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSummary {
    /// Extraction summary (if extraction was performed)
    pub extraction: Option<ExtractionSummary>,
    
    /// Processing summary (if processing was performed)
    pub processing: Option<ProcessingSummary>,
    
    /// Reporting summary (if reporting was performed)
    pub reporting: Option<ReportingSummary>,
    
    /// Export summary (if export was performed)
    pub export: Option<ExportSummary>,
    
    /// Total elapsed time for the entire workflow
    pub total_elapsed_time: Duration,
}

impl WorkflowSummary {
    /// Create a new workflow summary
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set extraction summary
    pub fn set_extraction(&mut self, summary: ExtractionSummary) {
        self.extraction = Some(summary);
    }
    
    /// Set processing summary
    pub fn set_processing(&mut self, summary: ProcessingSummary) {
        self.processing = Some(summary);
    }
    
    /// Set reporting summary
    pub fn set_reporting(&mut self, summary: ReportingSummary) {
        self.reporting = Some(summary);
    }
    
    /// Set export summary
    pub fn set_export(&mut self, summary: ExportSummary) {
        self.export = Some(summary);
    }
    
    /// Set total elapsed time
    pub fn set_total_elapsed_time(&mut self, elapsed: Duration) {
        self.total_elapsed_time = elapsed;
    }
    
    /// Check if the entire workflow was successful
    pub fn is_successful(&self) -> bool {
        let extraction_success = self.extraction.as_ref().map_or(true, |s| s.is_successful());
        let processing_success = self.processing.as_ref().map_or(true, |s| s.is_successful());
        let reporting_success = self.reporting.as_ref().map_or(true, |s| s.is_successful());
        let export_success = self.export.as_ref().map_or(true, |s| s.is_successful());
        
        extraction_success && processing_success && reporting_success && export_success
    }
    
    /// Get total error count across all summaries
    pub fn total_errors(&self) -> usize {
        let extraction_errors = self.extraction.as_ref().map_or(0, |s| s.errors.len());
        let processing_errors = self.processing.as_ref().map_or(0, |s| s.errors.len());
        let reporting_errors = self.reporting.as_ref().map_or(0, |s| s.errors.len());
        let export_errors = self.export.as_ref().map_or(0, |s| s.errors.len());
        
        extraction_errors + processing_errors + reporting_errors + export_errors
    }
}

impl Default for WorkflowSummary {
    fn default() -> Self {
        Self {
            extraction: None,
            processing: None,
            reporting: None,
            export: None,
            total_elapsed_time: Duration::from_secs(0),
        }
    }
}

impl std::fmt::Display for WorkflowSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Workflow Summary (Total time: {:.2}s):", self.total_elapsed_time.as_secs_f64())?;
        
        if let Some(extraction) = &self.extraction {
            writeln!(f, "  {}", extraction)?;
        }
        
        if let Some(processing) = &self.processing {
            writeln!(f, "  {}", processing)?;
        }
        
        if let Some(reporting) = &self.reporting {
            writeln!(f, "  {}", reporting)?;
        }
        
        if let Some(export) = &self.export {
            writeln!(f, "  {}", export)?;
        }
        
        if !self.is_successful() {
            writeln!(f, "  Total errors: {}", self.total_errors())?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_extraction_summary() {
        let mut summary = ExtractionSummary::new();
        
        summary.add_extracted_pbo(PathBuf::from("test1.pbo"));
        summary.add_extracted_pbo(PathBuf::from("test2.pbo"));
        summary.add_error("Test error".to_string());
        summary.set_elapsed_time(Duration::from_secs(10));
        
        assert_eq!(summary.extracted_pbos, 2);
        assert_eq!(summary.extraction_paths.len(), 2);
        assert_eq!(summary.errors.len(), 1);
        assert!(!summary.is_successful());
        assert_eq!(summary.success_rate(), 2.0 / 3.0);
    }
    
    #[test]
    fn test_processing_summary() {
        let mut summary = ProcessingSummary::new();
        
        summary.add_processed_pbo();
        summary.add_processed_pbo();
        summary.add_processed_files(100);
        summary.add_found_entries(500);
        summary.set_elapsed_time(Duration::from_secs(5));
        
        assert_eq!(summary.processed_pbos, 2);
        assert_eq!(summary.files_processed, 100);
        assert_eq!(summary.entries_found, 500);
        assert_eq!(summary.processing_rate(), 20.0);
        assert_eq!(summary.avg_entries_per_file(), 5.0);
    }
    
    #[test]
    fn test_reporting_summary() {
        let mut summary = ReportingSummary::new();
        
        summary.add_generated_report(PathBuf::from("report1.md"));
        summary.add_generated_report(PathBuf::from("report2.json"));
        summary.set_elapsed_time(Duration::from_secs(2));
        
        assert_eq!(summary.reports_generated, 2);
        assert_eq!(summary.report_paths.len(), 2);
        assert_eq!(summary.generation_rate(), 1.0);
        assert!(summary.is_successful());
    }
    
    #[test]
    fn test_workflow_summary() {
        let mut workflow_summary = WorkflowSummary::new();
        
        let mut extraction = ExtractionSummary::new();
        extraction.add_extracted_pbo(PathBuf::from("test.pbo"));
        
        let mut processing = ProcessingSummary::new();
        processing.add_processed_pbo();
        processing.add_error("Processing error".to_string());
        
        workflow_summary.set_extraction(extraction);
        workflow_summary.set_processing(processing);
        workflow_summary.set_total_elapsed_time(Duration::from_secs(15));
        
        assert!(!workflow_summary.is_successful());
        assert_eq!(workflow_summary.total_errors(), 1);
    }
    
    #[test]
    fn test_summary_display() {
        let mut summary = ExtractionSummary::new();
        summary.add_extracted_pbo(PathBuf::from("test.pbo"));
        summary.set_elapsed_time(Duration::from_millis(1500));
        
        let display = format!("{}", summary);
        assert!(display.contains("1 PBOs extracted"));
        assert!(display.contains("1.50s"));
    }
}