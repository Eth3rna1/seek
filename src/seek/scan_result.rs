use std::ops::Add;
use std::path::PathBuf;

/// A structure that encapsulates the scan()'s
/// function result, containing the paths memoized
/// and the counters
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub paths: Vec<PathBuf>,
    pub error_count: usize,
    pub success_count: usize,
}

impl ScanResult {
    /// Initializes an empty ScanResult
    pub fn new() -> Self {
        ScanResult {
            paths: Vec::new(),
            error_count: 0,
            success_count: 0,
        }
    }

    /// Pushes into self.paths
    pub fn push(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    /// Returns the length of self.paths
    pub fn size(&self) -> usize {
        self.paths.len()
    }

    /// Increases the success counter
    pub fn increase_success(&mut self, amount: usize) {
        self.success_count += amount;
    }

    /// Increases the error counter
    pub fn increase_error(&mut self, amount: usize) {
        self.error_count += amount;
    }

    /// Returns the total amount of objects
    /// the scan() function came across
    pub fn total(&self) -> usize {
        self.success_count + self.error_count
    }

    /// Merges another ScanResult into itself
    pub fn append(&mut self, mut other: ScanResult) {
        self.paths.append(&mut other.paths);
        self.error_count += other.error_count;
        self.success_count += other.success_count;
    }
}
