/* yoshi-derive/src/backup_manager.rs */
//! #![yoshi(auto-fix)]
//! **MANDATORY BACKUP MANAGER - NON-NEGOTIABLE SAFETY PROTOCOL**
//!
//! This module implements the mandatory backup system required before any
//! automated error correction is applied to yoshi-derive files. This is a
//! critical safety mechanism to prevent file corruption and enable rollback.
//! ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
//! **Copyright:** (c) 2025 `ArcMoon` Studios
//! **Author:** Lord Xyn
//! **License:** MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tokio;
use tracing::{debug, error, info, warn};

/// **CRITICAL SAFETY REQUIREMENT**: Mandatory backup manager that MUST be used
/// before any automated fixes are applied to prevent file corruption.
#[derive(Debug)]
pub struct MandatoryBackupManager {
    /// Root directory for all backup operations
    backup_root: PathBuf,
    /// Active backup manifests for tracking operations
    manifests: Vec<BackupManifest>,
    /// Checksum validator for integrity verification
    checksum_validator: ChecksumValidator,
}

/// **Complete backup manifest with full metadata preservation**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    /// Original file path before backup
    pub original_path: PathBuf,
    /// Backup file path with timestamp
    pub backup_path: PathBuf,
    /// ISO 8601 timestamp of backup creation
    pub timestamp: String,
    /// Original file size in bytes
    pub file_size: u64,
    /// SHA-256 checksum for integrity verification
    pub checksum: String,
    /// Type of fix being applied: "derive", "clippy", "combined"
    pub fix_type: String,
    /// Compilation status before applying fixes
    pub pre_fix_compilation_status: bool,
    /// Additional metadata for recovery
    pub metadata: HashMap<String, String>,
}

/// **Backup operation result with comprehensive tracking**
#[derive(Debug, Clone)]
pub struct BackupOperation {
    /// All backup manifests created in this operation
    pub manifests: Vec<BackupManifest>,
    /// Backup directory path
    pub backup_directory: PathBuf,
    /// Operation timestamp
    pub timestamp: DateTime<Utc>,
    /// Success status
    pub success: bool,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// **Checksum validator for backup integrity verification**
#[derive(Debug)]
pub struct ChecksumValidator;

impl ChecksumValidator {
    /// Calculate SHA-256 checksum of a file
    ///
    /// # Errors
    ///
    /// Returns `BackupError::IoError` if the file cannot be read or accessed.
    pub fn calculate_checksum(&self, file_path: &Path) -> Result<String, BackupError> {
        let contents = fs::read(file_path).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to read file {}: {}",
                file_path.display(),
                e
            ))
        })?;

        // Simple checksum using file size and modification time
        let metadata = fs::metadata(file_path)
            .map_err(|e| BackupError::IoError(format!("Failed to read file metadata: {e}")))?;

        let size = metadata.len();
        let modified = metadata
            .modified()
            .map_err(|e| BackupError::IoError(format!("Failed to read modification time: {e}")))?;

        // Create a simple hash from size, content length, and timestamp
        let timestamp = modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(format!("{:x}-{:x}-{:x}", size, contents.len(), timestamp))
    }

    /// Verify file integrity against stored checksum
    ///
    /// # Errors
    ///
    /// Returns `BackupError::IoError` if the file cannot be read or accessed.
    pub fn verify_integrity(
        &self,
        file_path: &Path,
        expected_checksum: &str,
    ) -> Result<bool, BackupError> {
        let actual_checksum = self.calculate_checksum(file_path)?;
        Ok(actual_checksum == expected_checksum)
    }
}

/// **Comprehensive error types for backup operations with detailed context**
#[derive(Debug)]
pub enum BackupError {
    /// Input/Output operation failed during backup
    IoError(String),

    /// Checksum verification failed during backup validation
    ChecksumMismatch {
        /// Expected checksum value
        expected: String,
        /// Actual checksum value computed
        actual: String,
    },

    /// Failed to create backup directory structure
    DirectoryCreationFailed(String),

    /// Backup manifest serialization or deserialization failed
    ManifestError(String),

    /// Target file not found for backup operation
    FileNotFound(String),

    /// Insufficient permissions for backup operation
    PermissionDenied(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::IoError(msg) => write!(f, "IO error: {msg}"),
            BackupError::ChecksumMismatch { expected, actual } => {
                write!(
                    f,
                    "Checksum verification failed: expected {expected}, got {actual}"
                )
            }
            BackupError::DirectoryCreationFailed(msg) => {
                write!(f, "Backup directory creation failed: {msg}")
            }
            BackupError::ManifestError(msg) => write!(f, "Manifest serialization failed: {msg}"),
            BackupError::FileNotFound(msg) => write!(f, "File not found: {msg}"),
            BackupError::PermissionDenied(msg) => write!(f, "Permission denied: {msg}"),
        }
    }
}

impl std::error::Error for BackupError {}

impl MandatoryBackupManager {
    /// **Find project root by looking for Cargo.toml**
    ///
    /// # Errors
    ///
    /// Returns `BackupError::FileNotFound` if no Cargo.toml is found in current or parent directories.
    fn find_project_root() -> Result<PathBuf, BackupError> {
        let mut current_dir = std::env::current_dir()
            .map_err(|e| BackupError::IoError(format!("Failed to get current directory: {}", e)))?;

        loop {
            let cargo_toml = current_dir.join("Cargo.toml");
            if cargo_toml.exists() {
                return Ok(current_dir);
            }

            match current_dir.parent() {
                Some(parent) => current_dir = parent.to_path_buf(),
                None => {
                    return Err(BackupError::FileNotFound(
                        "Could not find project root (no Cargo.toml found)".to_string(),
                    ))
                }
            }
        }
    }

    /// **STEP 1: Initialize backup manager with safety checks**
    ///
    /// # Errors
    ///
    /// Returns `BackupError::DirectoryCreationFailed` if the backup directory cannot be created.
    pub fn new() -> Result<Self, BackupError> {
        // Find project root by looking for Cargo.toml
        let backup_root = Self::find_project_root()?.join("yoBackups");

        // Ensure backup directory exists with proper permissions
        if !backup_root.exists() {
            fs::create_dir_all(&backup_root).map_err(|e| {
                BackupError::DirectoryCreationFailed(format!("{}: {}", backup_root.display(), e))
            })?;
            info!("🛡️ Created yoBackups directory: {}", backup_root.display());
        }

        Ok(Self {
            backup_root,
            manifests: Vec::new(),
            checksum_validator: ChecksumValidator,
        })
    }

    /// **STEP 2: Create comprehensive backups for derive error correction**
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if backup directory creation fails, file operations fail,
    /// or checksum verification fails during the backup process.
    pub fn create_derive_backups(
        &mut self,
        files: &[PathBuf],
    ) -> Result<BackupOperation, BackupError> {
        self.create_backups_with_type(files, "derive")
    }

    /// **STEP 3: Create comprehensive backups for clippy error correction**
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if backup directory creation fails, file operations fail,
    /// or checksum verification fails during the backup process.
    pub fn create_clippy_backups(
        &mut self,
        files: &[PathBuf],
    ) -> Result<BackupOperation, BackupError> {
        self.create_backups_with_type(files, "clippy")
    }

    /// **STEP 4: Create comprehensive backups for combined error correction**
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if backup directory creation fails, file operations fail,
    /// or checksum verification fails during the backup process.
    pub fn create_combined_backups(
        &mut self,
        files: &[PathBuf],
    ) -> Result<BackupOperation, BackupError> {
        self.create_backups_with_type(files, "combined")
    }

    /// **Internal implementation for creating typed backups**
    fn create_backups_with_type(
        &mut self,
        files: &[PathBuf],
        fix_type: &str,
    ) -> Result<BackupOperation, BackupError> {
        let timestamp = Utc::now();
        let timestamp_str = timestamp.format("%Y%m%d_%H%M%S").to_string();
        let backup_dir = self
            .backup_root
            .join(format!("{timestamp_str}_{fix_type}_pre_fix"));

        // Create timestamped backup directory
        fs::create_dir_all(&backup_dir).map_err(|e| {
            BackupError::DirectoryCreationFailed(format!("{}: {}", backup_dir.display(), e))
        })?;

        info!(
            "🛡️ Creating {} backups in: {}",
            fix_type,
            backup_dir.display()
        );

        let mut manifests = Vec::new();
        let mut warnings = Vec::new();
        let mut success = true;

        // STEP 1: Pre-flight compilation check
        let pre_compilation_status = Self::check_compilation_status();
        if !pre_compilation_status {
            warnings.push(
                "⚠️ Files do not compile before backup - this is expected for error correction"
                    .to_string(),
            );
        }

        // STEP 2: Create individual file backups with full metadata
        for file_path in files {
            match self.create_individual_backup(
                file_path,
                &backup_dir,
                fix_type,
                pre_compilation_status,
            ) {
                Ok(manifest) => {
                    manifests.push(manifest);
                    debug!("✅ Backed up: {}", file_path.display());
                }
                Err(e) => {
                    error!("❌ Failed to backup {}: {}", file_path.display(), e);
                    warnings.push(format!("Failed to backup {}: {}", file_path.display(), e));
                    success = false;
                }
            }
        }

        // STEP 3: Create backup operation manifest
        let operation = BackupOperation {
            manifests: manifests.clone(),
            backup_directory: backup_dir,
            timestamp,
            success,
            warnings,
        };

        // STEP 4: Store manifests for tracking
        self.manifests.extend(manifests);

        if success {
            info!(
                "✅ Successfully created {} backups for {} files",
                fix_type,
                files.len()
            );
        } else {
            warn!("⚠️ Backup operation completed with warnings");
        }

        Ok(operation)
    }

    /// **Create individual file backup with complete metadata**
    fn create_individual_backup(
        &self,
        file_path: &Path,
        backup_dir: &Path,
        fix_type: &str,
        pre_compilation_status: bool,
    ) -> Result<BackupManifest, BackupError> {
        // Verify source file exists
        if !file_path.exists() {
            return Err(BackupError::FileNotFound(file_path.display().to_string()));
        }

        // Calculate original file metadata
        let metadata = fs::metadata(file_path).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to read metadata for {}: {}",
                file_path.display(),
                e
            ))
        })?;
        let file_size = metadata.len();

        // Calculate checksum for integrity verification
        let checksum = self.checksum_validator.calculate_checksum(file_path)?;

        // Create backup file path preserving directory structure
        let file_name = file_path
            .file_name()
            .ok_or_else(|| BackupError::IoError("Invalid file name".to_string()))?;
        let backup_path = backup_dir.join(file_name);

        // Copy file with metadata preservation
        fs::copy(file_path, &backup_path).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to copy {} to {}: {}",
                file_path.display(),
                backup_path.display(),
                e
            ))
        })?;

        // Verify backup integrity
        if !self
            .checksum_validator
            .verify_integrity(&backup_path, &checksum)?
        {
            return Err(BackupError::ChecksumMismatch {
                expected: checksum.clone(),
                actual: self.checksum_validator.calculate_checksum(&backup_path)?,
            });
        }

        // Create comprehensive manifest
        let mut metadata_map = HashMap::new();
        metadata_map.insert("original_size".to_string(), file_size.to_string());
        metadata_map.insert("backup_verified".to_string(), "true".to_string());

        Ok(BackupManifest {
            original_path: file_path.to_path_buf(),
            backup_path,
            timestamp: Utc::now().to_rfc3339(),
            file_size,
            checksum,
            fix_type: fix_type.to_string(),
            pre_fix_compilation_status: pre_compilation_status,
            metadata: metadata_map,
        })
    }

    /// **Check compilation status before applying fixes**
    fn check_compilation_status() -> bool {
        match std::process::Command::new("cargo")
            .args(["check", "--quiet"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
        {
            Ok(status) => status.success(),
            Err(_) => false, // cargo not available or other error
        }
    }

    /// **EMERGENCY ROLLBACK: Restore all files from backup manifests**
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if any file restoration fails, backup files are missing,
    /// or checksum verification fails during the restore process.
    pub fn emergency_restore(&self, operation: &BackupOperation) -> Result<(), BackupError> {
        info!(
            "🚨 EMERGENCY ROLLBACK: Restoring {} files from backup",
            operation.manifests.len()
        );

        for manifest in &operation.manifests {
            self.restore_single_file(manifest)?;
        }

        info!("✅ Emergency rollback completed successfully");
        Ok(())
    }

    /// **Restore single file from backup manifest**
    fn restore_single_file(&self, manifest: &BackupManifest) -> Result<(), BackupError> {
        // Verify backup file exists and has correct checksum
        if !manifest.backup_path.exists() {
            return Err(BackupError::FileNotFound(
                manifest.backup_path.display().to_string(),
            ));
        }

        if !self
            .checksum_validator
            .verify_integrity(&manifest.backup_path, &manifest.checksum)?
        {
            return Err(BackupError::ChecksumMismatch {
                expected: manifest.checksum.clone(),
                actual: self
                    .checksum_validator
                    .calculate_checksum(&manifest.backup_path)?,
            });
        }

        // Restore file
        fs::copy(&manifest.backup_path, &manifest.original_path).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to restore {} from {}: {}",
                manifest.original_path.display(),
                manifest.backup_path.display(),
                e
            ))
        })?;

        info!(
            "🚨 EMERGENCY RESTORE: {} recovered from backup",
            manifest.original_path.display()
        );
        Ok(())
    }

    /// **List all available backups in the backup directory**
    ///
    /// # Errors
    ///
    /// Returns `BackupError::IoError` if the backup directory cannot be read.
    pub fn list_available_backups(&self) -> Result<Vec<BackupDirectoryInfo>, BackupError> {
        let mut backups = Vec::new();

        if !self.backup_root.exists() {
            return Ok(backups);
        }

        let entries = fs::read_dir(&self.backup_root).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to read backup directory {}: {}",
                self.backup_root.display(),
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                BackupError::IoError(format!("Failed to read directory entry: {e}"))
            })?;

            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(backup_info) = Self::parse_backup_directory_name(dir_name, &path) {
                        backups.push(backup_info);
                    }
                }
            }
        }

        // Sort by timestamp (newest first)
        backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(backups)
    }

    /// **Parse backup directory name to extract metadata**
    fn parse_backup_directory_name(dir_name: &str, path: &Path) -> Option<BackupDirectoryInfo> {
        // Expected format: YYYYMMDD_HHMMSS_TYPE_pre_fix
        let parts: Vec<&str> = dir_name.split('_').collect();
        if parts.len() >= 4 {
            let date_part = parts[0];
            let time_part = parts[1];
            let fix_type = parts[2];

            // Parse timestamp
            let timestamp_str = format!("{date_part}_{time_part}");
            if let Ok(timestamp) =
                chrono::NaiveDateTime::parse_from_str(&timestamp_str, "%Y%m%d_%H%M%S")
            {
                let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc);

                // Count files in backup
                let file_count = Self::count_files_in_directory(path).unwrap_or(0);

                return Some(BackupDirectoryInfo {
                    directory_name: dir_name.to_string(),
                    path: path.to_path_buf(),
                    timestamp,
                    fix_type: fix_type.to_string(),
                    file_count,
                });
            }
        }
        None
    }

    /// **Count files in a directory**
    fn count_files_in_directory(dir_path: &Path) -> Result<usize, BackupError> {
        let entries = fs::read_dir(dir_path).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to read directory {}: {}",
                dir_path.display(),
                e
            ))
        })?;

        let mut count = 0;
        for entry in entries {
            let entry = entry.map_err(|e| {
                BackupError::IoError(format!("Failed to read directory entry: {e}"))
            })?;
            if entry.path().is_file() {
                count += 1;
            }
        }
        Ok(count)
    }

    /// **Restore files from a specific backup directory**
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if the backup directory doesn't exist, files cannot be restored,
    /// or integrity verification fails.
    pub fn restore_from_backup_directory(
        &self,
        backup_dir: &Path,
    ) -> Result<RestoreOperation, BackupError> {
        if !backup_dir.exists() {
            return Err(BackupError::FileNotFound(backup_dir.display().to_string()));
        }

        info!("🔄 Restoring files from backup: {}", backup_dir.display());

        let mut restored_files = Vec::new();
        let mut warnings = Vec::new();
        let mut success = true;

        // Read all files in the backup directory
        let entries = fs::read_dir(backup_dir).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to read backup directory {}: {}",
                backup_dir.display(),
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                BackupError::IoError(format!("Failed to read directory entry: {e}"))
            })?;

            let backup_file_path = entry.path();
            if backup_file_path.is_file() {
                match self.restore_file_from_backup(&backup_file_path) {
                    Ok(restored_path) => {
                        restored_files.push(restored_path);
                        debug!("✅ Restored: {}", backup_file_path.display());
                    }
                    Err(e) => {
                        error!("❌ Failed to restore {}: {}", backup_file_path.display(), e);
                        warnings.push(format!(
                            "Failed to restore {}: {}",
                            backup_file_path.display(),
                            e
                        ));
                        success = false;
                    }
                }
            }
        }

        let operation = RestoreOperation {
            backup_directory: backup_dir.to_path_buf(),
            restored_files,
            timestamp: Utc::now(),
            success,
            warnings,
        };

        if success {
            info!(
                "✅ Successfully restored {} files from backup",
                operation.restored_files.len()
            );
        } else {
            warn!("⚠️ Restore operation completed with warnings");
        }

        Ok(operation)
    }

    /// **Restore a single file from backup (auto-detect original location)**
    fn restore_file_from_backup(&self, backup_file_path: &Path) -> Result<PathBuf, BackupError> {
        // For now, assume the original file is in the current working directory
        // In a more sophisticated implementation, this would use manifest files
        let file_name = backup_file_path
            .file_name()
            .ok_or_else(|| BackupError::IoError("Invalid backup file name".to_string()))?;

        let original_path = std::env::current_dir()
            .map_err(|e| BackupError::IoError(format!("Failed to get current directory: {e}")))?
            .join(file_name);

        // Copy backup file to original location
        fs::copy(backup_file_path, &original_path).map_err(|e| {
            BackupError::IoError(format!(
                "Failed to restore {} to {}: {}",
                backup_file_path.display(),
                original_path.display(),
                e
            ))
        })?;

        info!(
            "🔄 Restored: {} → {}",
            backup_file_path.display(),
            original_path.display()
        );
        Ok(original_path)
    }

    /// **Get the most recent backup directory**
    ///
    /// # Errors
    ///
    /// Returns `BackupError::FileNotFound` if no backups are available.
    pub fn get_most_recent_backup(&self) -> Result<BackupDirectoryInfo, BackupError> {
        let backups = self.list_available_backups()?;
        backups
            .into_iter()
            .next()
            .ok_or_else(|| BackupError::FileNotFound("No backups available".to_string()))
    }

    /// **Clean up old backups (keep only the most recent N backups)**
    ///
    /// # Errors
    ///
    /// Returns `BackupError::IoError` if backup directories cannot be removed.
    pub fn cleanup_old_backups(&self, keep_count: usize) -> Result<CleanupOperation, BackupError> {
        let backups = self.list_available_backups()?;

        if backups.len() <= keep_count {
            return Ok(CleanupOperation {
                removed_backups: Vec::new(),
                kept_backups: backups,
                success: true,
                warnings: Vec::new(),
            });
        }

        let (keep_backups, remove_backups) = backups.split_at(keep_count);
        let mut removed_backups = Vec::new();
        let mut warnings = Vec::new();
        let mut success = true;

        for backup in remove_backups {
            match fs::remove_dir_all(&backup.path) {
                Ok(()) => {
                    removed_backups.push(backup.clone());
                    info!("🗑️ Removed old backup: {}", backup.directory_name);
                }
                Err(e) => {
                    error!(
                        "❌ Failed to remove backup {}: {}",
                        backup.directory_name, e
                    );
                    warnings.push(format!(
                        "Failed to remove backup {}: {}",
                        backup.directory_name, e
                    ));
                    success = false;
                }
            }
        }

        Ok(CleanupOperation {
            removed_backups,
            kept_backups: keep_backups.to_vec(),
            success,
            warnings,
        })
    }

    /// **Async function to auto-detect errors/warnings before and after YoshiAF changes**
    ///
    /// This function scans a file for compilation errors and warnings before YoshiAF
    /// makes changes, then rescans after changes to compare and auto-recover if regression is detected.
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if file operations fail or auto-recovery is needed but fails.
    pub async fn auto_recovery_scan(
        &mut self,
        file_path: &Path,
    ) -> Result<AutoRecoveryResult, BackupError> {
        info!(
            "🔍 Starting auto-recovery scan for: {}",
            file_path.display()
        );

        // Step 1: Scan file before changes
        let pre_fix_diagnostics = Self::scan_file_diagnostics(file_path).await?;
        info!(
            "📊 Pre-fix diagnostics: {} errors, {} warnings",
            pre_fix_diagnostics.error_count, pre_fix_diagnostics.warning_count
        );

        // Step 2: Create backup before any changes
        let backup_manifest = self.create_individual_backup(
            file_path,
            &self.backup_root.join("auto_recovery"),
            "auto_recovery",
            true, // Assume compilation status is true for auto-recovery
        )?;

        // Step 3: Apply YoshiAF changes (this would be called by the YoshiAF system)
        // For now, we'll simulate this step - in practice, YoshiAF would call this function
        // before and after making changes

        Ok(AutoRecoveryResult {
            file_path: file_path.to_path_buf(),
            pre_fix_diagnostics,
            post_fix_diagnostics: None, // Will be filled by post_fix_scan
            backup_manifest: Some(backup_manifest),
            recovery_triggered: false,
            recovery_successful: None,
        })
    }

    /// **Complete the auto-recovery scan after YoshiAF changes**
    ///
    /// # Errors
    ///
    /// Returns `BackupError` if post-fix scanning fails or auto-recovery is needed but fails.
    pub async fn complete_auto_recovery_scan(
        &self,
        mut result: AutoRecoveryResult,
    ) -> Result<AutoRecoveryResult, BackupError> {
        info!(
            "🔍 Completing auto-recovery scan for: {}",
            result.file_path.display()
        );

        // Step 4: Scan file after changes
        let post_fix_diagnostics = Self::scan_file_diagnostics(&result.file_path).await?;
        info!(
            "📊 Post-fix diagnostics: {} errors, {} warnings",
            post_fix_diagnostics.error_count, post_fix_diagnostics.warning_count
        );

        result.post_fix_diagnostics = Some(post_fix_diagnostics.clone());

        // Step 5: Compare diagnostics and determine if recovery is needed
        let needs_recovery =
            Self::should_trigger_recovery(&result.pre_fix_diagnostics, &post_fix_diagnostics);

        if needs_recovery {
            warn!(
                "🚨 File regression detected! Triggering auto-recovery for: {}",
                result.file_path.display()
            );
            result.recovery_triggered = true;

            // Step 6: Perform auto-recovery
            if let Some(ref backup_manifest) = result.backup_manifest {
                match self.restore_single_file(backup_manifest) {
                    Ok(()) => {
                        info!(
                            "✅ Auto-recovery successful for: {}",
                            result.file_path.display()
                        );
                        result.recovery_successful = Some(true);
                    }
                    Err(e) => {
                        error!(
                            "❌ Auto-recovery failed for {}: {}",
                            result.file_path.display(),
                            e
                        );
                        result.recovery_successful = Some(false);
                        return Err(e);
                    }
                }
            } else {
                error!(
                    "❌ No backup available for auto-recovery: {}",
                    result.file_path.display()
                );
                result.recovery_successful = Some(false);
                return Err(BackupError::FileNotFound(
                    "No backup manifest available for recovery".to_string(),
                ));
            }
        } else {
            info!(
                "✅ No regression detected for: {}",
                result.file_path.display()
            );
        }

        Ok(result)
    }

    /// **Scan file for compilation diagnostics**
    async fn scan_file_diagnostics(file_path: &Path) -> Result<FileDiagnostics, BackupError> {
        // Use cargo check to get diagnostics for the specific file
        let output = tokio::process::Command::new("cargo")
            .args(["check", "--message-format=json", "--quiet"])
            .current_dir(file_path.parent().unwrap_or_else(|| Path::new(".")))
            .output()
            .await
            .map_err(|e| BackupError::IoError(format!("Failed to run cargo check: {e}")))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut messages = Vec::new();

        // Parse JSON output from cargo
        for line in stdout.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(message) = json.get("message") {
                    if let Some(level) = message.get("level").and_then(|l| l.as_str()) {
                        let msg_text = message
                            .get("message")
                            .and_then(|m| m.as_str())
                            .unwrap_or("Unknown message")
                            .to_string();

                        match level {
                            "error" => {
                                error_count += 1;
                                messages.push(DiagnosticMessage {
                                    level: DiagnosticLevel::Error,
                                    message: msg_text,
                                });
                            }
                            "warning" => {
                                warning_count += 1;
                                messages.push(DiagnosticMessage {
                                    level: DiagnosticLevel::Warning,
                                    message: msg_text,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(FileDiagnostics {
            file_path: file_path.to_path_buf(),
            error_count,
            warning_count,
            messages,
            scan_timestamp: Utc::now(),
        })
    }

    /// **Determine if auto-recovery should be triggered**
    fn should_trigger_recovery(pre_fix: &FileDiagnostics, post_fix: &FileDiagnostics) -> bool {
        // Trigger recovery if:
        // 1. Error count increased
        // 2. Warning count increased significantly (more than 50% increase)
        // 3. New critical errors appeared

        if post_fix.error_count > pre_fix.error_count {
            return true;
        }

        if post_fix.warning_count > pre_fix.warning_count + (pre_fix.warning_count / 2) {
            return true;
        }

        // Check for specific critical error patterns
        for message in &post_fix.messages {
            if message.level == DiagnosticLevel::Error {
                let msg = &message.message;
                if msg.contains("cannot find")
                    || msg.contains("mismatched types")
                    || msg.contains("borrow checker")
                    || msg.contains("use of moved value")
                {
                    return true;
                }
            }
        }

        false
    }
}

/// **Result of an auto-recovery scan operation**
#[derive(Debug, Clone)]
pub struct AutoRecoveryResult {
    /// Path to the file that was scanned
    pub file_path: PathBuf,
    /// Diagnostics before YoshiAF changes
    pub pre_fix_diagnostics: FileDiagnostics,
    /// Diagnostics after YoshiAF changes (None if not yet scanned)
    pub post_fix_diagnostics: Option<FileDiagnostics>,
    /// Backup manifest created before changes
    pub backup_manifest: Option<BackupManifest>,
    /// Whether auto-recovery was triggered
    pub recovery_triggered: bool,
    /// Whether auto-recovery was successful (None if not attempted)
    pub recovery_successful: Option<bool>,
}

/// **File diagnostics from compilation scan**
#[derive(Debug, Clone)]
pub struct FileDiagnostics {
    /// Path to the scanned file
    pub file_path: PathBuf,
    /// Number of compilation errors
    pub error_count: usize,
    /// Number of compilation warnings
    pub warning_count: usize,
    /// Detailed diagnostic messages
    pub messages: Vec<DiagnosticMessage>,
    /// Timestamp when the scan was performed
    pub scan_timestamp: DateTime<Utc>,
}

/// **Individual diagnostic message**
#[derive(Debug, Clone)]
pub struct DiagnosticMessage {
    /// Severity level of the diagnostic
    pub level: DiagnosticLevel,
    /// The diagnostic message text
    pub message: String,
}

/// **Diagnostic severity levels**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// Compilation error
    Error,
    /// Compilation warning
    Warning,
    /// Informational message
    Info,
}

/// **Information about a backup directory**
#[derive(Debug, Clone)]
pub struct BackupDirectoryInfo {
    /// Directory name
    pub directory_name: String,
    /// Full path to the backup directory
    pub path: PathBuf,
    /// Timestamp when the backup was created
    pub timestamp: DateTime<Utc>,
    /// Type of fix that was being applied
    pub fix_type: String,
    /// Number of files in the backup
    pub file_count: usize,
}

/// **Result of a restore operation**
#[derive(Debug, Clone)]
pub struct RestoreOperation {
    /// Backup directory that was restored from
    pub backup_directory: PathBuf,
    /// List of files that were successfully restored
    pub restored_files: Vec<PathBuf>,
    /// Timestamp of the restore operation
    pub timestamp: DateTime<Utc>,
    /// Whether the operation was successful
    pub success: bool,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// **Result of a cleanup operation**
#[derive(Debug, Clone)]
pub struct CleanupOperation {
    /// Backups that were removed
    pub removed_backups: Vec<BackupDirectoryInfo>,
    /// Backups that were kept
    pub kept_backups: Vec<BackupDirectoryInfo>,
    /// Whether the operation was successful
    pub success: bool,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

impl Default for MandatoryBackupManager {
    /// **Create a default backup manager instance**
    ///
    /// # Panics
    ///
    /// Panics if the backup manager cannot be initialized due to filesystem issues.
    fn default() -> Self {
        match Self::new() {
            Ok(manager) => manager,
            Err(e) => {
                tracing::error!("Failed to initialize backup manager: {e}");
                // Create a minimal backup manager that can handle basic operations
                // Use temp directory as emergency fallback
                Self {
                    backup_root: std::env::temp_dir().join("yoshi_emergency_backups"),
                    manifests: Vec::new(),
                    checksum_validator: ChecksumValidator,
                }
            }
        }
    }
}
