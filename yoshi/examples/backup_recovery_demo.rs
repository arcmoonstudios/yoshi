/* yoshi/examples/backup_recovery_demo.rs */
//! **Backup and Recovery System Demo**
//!
//! This example demonstrates the comprehensive backup and recovery system
//! implemented in YoshiAF, including CLI-based recovery operations and
//! async auto-recovery scanning.

use std::path::PathBuf;
use yoshi::auto_fix::MandatoryBackupManager;
use yoshi::{tracing, Hatch};

/// **Demonstrate basic backup operations**
async fn demo_basic_backup_operations() -> Hatch<()> {
    tracing::info!("🛡️ Demonstrating Basic Backup Operations");

    // Initialize backup manager
    let mut backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // Create some test files for backup
    let test_files = vec![PathBuf::from("src/lib.rs"), PathBuf::from("src/main.rs")];

    // Create derive backups
    tracing::info!("📦 Creating derive backups...");
    let backup_operation = backup_manager
        .create_derive_backups(&test_files)
        .map_err(|e| yoshi::yopost!(message: format!("Failed to create backups: {e}").into()))?;

    tracing::info!("✅ Backup operation completed:");
    tracing::info!(
        "  📁 Backup directory: {}",
        backup_operation.backup_directory.display()
    );
    tracing::info!("  📄 Files backed up: {}", backup_operation.manifests.len());
    tracing::info!("  ✅ Success: {}", backup_operation.success);

    Ok(())
}

/// **Demonstrate backup listing and recovery**
async fn demo_backup_listing_and_recovery() -> Hatch<()> {
    tracing::info!("📋 Demonstrating Backup Listing and Recovery");

    let backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // List available backups
    tracing::info!("📁 Listing available backups...");
    let backups = backup_manager
        .list_available_backups()
        .map_err(|e| yoshi::yopost!(message: format!("Failed to list backups: {e}").into()))?;

    if backups.is_empty() {
        tracing::info!("📋 No backups found - this is normal for a fresh installation");
        return Ok(());
    }

    tracing::info!("📋 Found {} backup(s):", backups.len());
    for backup in &backups {
        tracing::info!(
            "  📁 {} ({} files, {})",
            backup.directory_name,
            backup.file_count,
            backup.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );
    }

    // Get most recent backup
    if let Ok(recent_backup) = backup_manager.get_most_recent_backup() {
        tracing::info!("🕒 Most recent backup: {}", recent_backup.directory_name);
    }

    Ok(())
}

/// **Demonstrate auto-recovery scanning**
async fn demo_auto_recovery_scanning() -> Hatch<()> {
    tracing::info!("🔍 Demonstrating Auto-Recovery Scanning");

    let mut backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // Simulate auto-recovery scan on a test file
    let test_file = PathBuf::from("src/lib.rs");

    if test_file.exists() {
        tracing::info!(
            "🔍 Starting auto-recovery scan for: {}",
            test_file.display()
        );

        // Start auto-recovery scan (this would be called before YoshiAF makes changes)
        let recovery_result = backup_manager
            .auto_recovery_scan(&test_file)
            .await
            .map_err(
                |e| yoshi::yopost!(message: format!("Auto-recovery scan failed: {e}").into()),
            )?;

        tracing::info!("📊 Pre-fix diagnostics:");
        tracing::info!(
            "  ❌ Errors: {}",
            recovery_result.pre_fix_diagnostics.error_count
        );
        tracing::info!(
            "  ⚠️ Warnings: {}",
            recovery_result.pre_fix_diagnostics.warning_count
        );

        // Complete the auto-recovery scan (this would be called after YoshiAF makes changes)
        let final_result = backup_manager
            .complete_auto_recovery_scan(recovery_result)
            .await
            .map_err(
                |e| yoshi::yopost!(message: format!("Auto-recovery completion failed: {e}").into()),
            )?;

        if let Some(ref post_fix) = final_result.post_fix_diagnostics {
            tracing::info!("📊 Post-fix diagnostics:");
            tracing::info!("  ❌ Errors: {}", post_fix.error_count);
            tracing::info!("  ⚠️ Warnings: {}", post_fix.warning_count);
        }

        if final_result.recovery_triggered {
            tracing::info!("🚨 Auto-recovery was triggered!");
            if let Some(success) = final_result.recovery_successful {
                if success {
                    tracing::info!("✅ Auto-recovery completed successfully");
                } else {
                    tracing::error!("❌ Auto-recovery failed");
                }
            }
        } else {
            tracing::info!("✅ No regression detected - no recovery needed");
        }
    } else {
        tracing::warn!(
            "⚠️ Test file {} not found - skipping auto-recovery demo",
            test_file.display()
        );
    }

    Ok(())
}

/// **Demonstrate cleanup operations**
async fn demo_cleanup_operations() -> Hatch<()> {
    tracing::info!("🧹 Demonstrating Cleanup Operations");

    let backup_manager = MandatoryBackupManager::new().map_err(
        |e| yoshi::yopost!(message: format!("Failed to initialize backup manager: {e}").into()),
    )?;

    // Demonstrate cleanup (keep only 3 most recent backups)
    tracing::info!("🧹 Cleaning up old backups (keeping 3 most recent)...");
    let cleanup_result = backup_manager
        .cleanup_old_backups(3)
        .map_err(|e| yoshi::yopost!(message: format!("Cleanup failed: {e}").into()))?;

    tracing::info!("✅ Cleanup completed:");
    tracing::info!(
        "  🗑️ Removed: {} backups",
        cleanup_result.removed_backups.len()
    );
    tracing::info!("  📁 Kept: {} backups", cleanup_result.kept_backups.len());
    tracing::info!("  ✅ Success: {}", cleanup_result.success);

    if !cleanup_result.removed_backups.is_empty() {
        tracing::info!("  Removed backups:");
        for backup in &cleanup_result.removed_backups {
            tracing::info!("    🗑️ {}", backup.directory_name);
        }
    }

    Ok(())
}

/// **Main demo function**
#[tokio::main]
async fn main() -> Hatch<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 YoshiAF Backup and Recovery System Demo");
    tracing::info!("===========================================");

    // Run all demos
    demo_basic_backup_operations().await?;
    tracing::info!("");

    demo_backup_listing_and_recovery().await?;
    tracing::info!("");

    demo_auto_recovery_scanning().await?;
    tracing::info!("");

    demo_cleanup_operations().await?;
    tracing::info!("");

    tracing::info!("✅ All backup and recovery demos completed successfully!");
    tracing::info!("");
    tracing::info!("🔧 CLI Usage Examples:");
    tracing::info!("  yoshi --list-backups                    # List all available backups");
    tracing::info!("  yoshi --emergency-rollback              # Rollback to most recent backup");
    tracing::info!("  yoshi --restore-from=<DIR>              # Restore from specific backup");
    tracing::info!("  yoshi --cleanup-backups=5               # Keep only 5 most recent backups");
    tracing::info!("  yoshi --run-yoshiautorust --enable-auto-recovery  # Enable auto-recovery");

    Ok(())
}
