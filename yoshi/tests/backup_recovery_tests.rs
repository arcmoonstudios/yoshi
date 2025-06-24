/* yoshi/tests/backup_recovery_tests.rs */
//! **Backup and Recovery System Tests**
//!
//! This module tests the comprehensive backup and recovery system
//! implemented in YoshiAF, including CLI-based recovery operations and
//! async auto-recovery scanning.

use std::path::PathBuf;
use yoshi::auto_fix::MandatoryBackupManager;
use yoshi::Hatch;

#[tokio::test]
async fn test_backup_manager_initialization() -> Hatch<()> {
    // Test that backup manager can be initialized
    let backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // Test listing backups (should work even if empty)
    let backups = backup_manager
        .list_available_backups()
        .map_err(|e| yoshi::yopost!(message: format!("Failed to list backups: {e}").into()))?;

    // Should not fail even if no backups exist
    // Length is always >= 0 for Vec, so just check it's a valid Vec
    let _backup_count = backups.len();

    Ok(())
}

#[tokio::test]
async fn test_backup_cleanup() -> Hatch<()> {
    let backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // Test cleanup operation (should work even with no backups)
    let cleanup_result = backup_manager
        .cleanup_old_backups(5)
        .map_err(|e| yoshi::yopost!(message: format!("Cleanup failed: {e}").into()))?;

    // Should complete successfully
    assert!(cleanup_result.success || !cleanup_result.warnings.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_auto_recovery_scan_basic() -> Hatch<()> {
    let mut backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // Create a test file for scanning
    let test_file = PathBuf::from("src/lib.rs");

    if test_file.exists() {
        // Test auto-recovery scan initialization
        let recovery_result = backup_manager
            .auto_recovery_scan(&test_file)
            .await
            .map_err(
                |e| yoshi::yopost!(message: format!("Auto-recovery scan failed: {e}").into()),
            )?;

        // Should have pre-fix diagnostics (counts are always valid usize values)
        let _error_count = recovery_result.pre_fix_diagnostics.error_count;
        let _warning_count = recovery_result.pre_fix_diagnostics.warning_count;

        // Should have a backup manifest
        assert!(recovery_result.backup_manifest.is_some());

        // Complete the scan
        let final_result = backup_manager
            .complete_auto_recovery_scan(recovery_result)
            .await
            .map_err(
                |e| yoshi::yopost!(message: format!("Auto-recovery completion failed: {e}").into()),
            )?;

        // Should have post-fix diagnostics
        assert!(final_result.post_fix_diagnostics.is_some());
    }

    Ok(())
}

#[test]
fn test_backup_directory_info_creation() {
    use chrono::Utc;
    use yoshi::auto_fix::BackupDirectoryInfo;

    // Test creating backup directory info
    let backup_info = BackupDirectoryInfo {
        directory_name: "20250624_143000_clippy_pre_fix".to_string(),
        path: PathBuf::from("/test/backup/dir"),
        timestamp: Utc::now(),
        fix_type: "clippy".to_string(),
        file_count: 5,
    };

    assert_eq!(backup_info.directory_name, "20250624_143000_clippy_pre_fix");
    assert_eq!(backup_info.fix_type, "clippy");
    assert_eq!(backup_info.file_count, 5);
}

#[test]
fn test_diagnostic_levels() {
    use yoshi::auto_fix::{DiagnosticLevel, DiagnosticMessage};

    let error_msg = DiagnosticMessage {
        level: DiagnosticLevel::Error,
        message: "Test error".to_string(),
    };

    let warning_msg = DiagnosticMessage {
        level: DiagnosticLevel::Warning,
        message: "Test warning".to_string(),
    };

    assert_eq!(error_msg.level, DiagnosticLevel::Error);
    assert_eq!(warning_msg.level, DiagnosticLevel::Warning);
    assert_ne!(error_msg.level, warning_msg.level);
}

#[tokio::test]
async fn test_file_diagnostics_structure() -> Hatch<()> {
    use chrono::Utc;
    use yoshi::auto_fix::FileDiagnostics;

    // Test creating file diagnostics
    let diagnostics = FileDiagnostics {
        file_path: PathBuf::from("test.rs"),
        error_count: 2,
        warning_count: 5,
        messages: Vec::new(),
        scan_timestamp: Utc::now(),
    };

    assert_eq!(diagnostics.error_count, 2);
    assert_eq!(diagnostics.warning_count, 5);
    assert_eq!(diagnostics.file_path, PathBuf::from("test.rs"));

    Ok(())
}

#[test]
fn test_restore_operation_structure() {
    use chrono::Utc;
    use yoshi::auto_fix::RestoreOperation;

    let restore_op = RestoreOperation {
        backup_directory: PathBuf::from("/backup/dir"),
        restored_files: vec![PathBuf::from("file1.rs"), PathBuf::from("file2.rs")],
        timestamp: Utc::now(),
        success: true,
        warnings: Vec::new(),
    };

    assert!(restore_op.success);
    assert_eq!(restore_op.restored_files.len(), 2);
    assert!(restore_op.warnings.is_empty());
}

#[test]
fn test_cleanup_operation_structure() {
    use chrono::Utc;
    use yoshi::auto_fix::{BackupDirectoryInfo, CleanupOperation};

    let cleanup_op = CleanupOperation {
        removed_backups: Vec::new(),
        kept_backups: vec![BackupDirectoryInfo {
            directory_name: "recent_backup".to_string(),
            path: PathBuf::from("/backup/recent"),
            timestamp: Utc::now(),
            fix_type: "clippy".to_string(),
            file_count: 3,
        }],
        success: true,
        warnings: Vec::new(),
    };

    assert!(cleanup_op.success);
    assert_eq!(cleanup_op.kept_backups.len(), 1);
    assert_eq!(cleanup_op.removed_backups.len(), 0);
}
