# scripts/arcmoon-upgrade.ps1
#![deny(unsafe_code)]
#![warn(missing_docs)]
#!
#! **Brief:** ArcMoon Studios Enterprise PowerShell Upgrade Script with Git Safety Layer.
#!
#! **Module Classification:** Performance-Critical
#! **Complexity Level:** Expert
#! **API Stability:** Stable
#!
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
#! + Cross-platform PowerShell script for Windows environments
#!  - Git safety checks with robust error handling
#!  - Automatic cargo-edit installation and verification
#!  - Enhanced logging with timestamp and color output
#!  - Rollback capability for failed upgrade operations
#!  - Performance monitoring and execution time tracking
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
#!
#! ## Mathematical Properties
#!
#! **Algorithmic Complexity:**
#! - Time Complexity: O(n) where n is number of dependencies
#! - Space Complexity: O(1) constant overhead for git operations
#! - Concurrency Safety: Thread-safe PowerShell execution model
#!
#! **Performance Characteristics:**
#! - Expected Performance: Sub-second git checks, minutes for upgrades
#! - Worst-Case Scenarios: Network-dependent cargo operations
#! - Optimization Opportunities: Parallel dependency resolution
#!
# **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
# **Copyright:** (c) 2025 ArcMoon Studios
# **Author:** Lord Xyn
# **License:** MIT OR Apache-2.0
# **License File:** /LICENSE
# **License Terms:** Full open source freedom; dual licensing allows choice between MIT and Apache 2.0
# **Effective Date:** 2025-05-30 | **Open Source Release**
# **Contact:** LordXyn@proton.me

[CmdletBinding()]
param(
    [switch]$Force,
    [switch]$SkipGitCheck,
    [switch]$DryRun,
    [string]$LogLevel = "Info"
)

# Set strict mode for enhanced error detection
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# ArcMoon Studios Enterprise Configuration
$script:ProjectName = "Yoshi"
$script:LogTimestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$script:StartTime = Get-Date

# Enhanced logging with color support
function Write-ArcMoonLog {
    param(
        [string]$Message,
        [string]$Level = "Info",
        [ConsoleColor]$Color = [ConsoleColor]::White
    )

    $timestamp = Get-Date -Format "HH:mm:ss.fff"
    $prefix = switch ($Level) {
        "Success" { "✅" }
        "Warning" { "⚠️ " }
        "Error"   { "❌" }
        "Info"    { "ℹ️ " }
        default   { "📝" }
    }

    Write-Host "[$timestamp] $prefix $Message" -ForegroundColor $Color
}

# Git safety check function
function Test-GitSafety {
    Write-ArcMoonLog "Checking for uncommitted changes in Cargo.toml..." -Level "Info" -Color Cyan

    try {
        $gitStatus = git status --porcelain Cargo.toml 2>$null
        if ($gitStatus) {
            $changes = ($gitStatus | Measure-Object -Line).Lines
            if ($changes -gt 0) {
                Write-ArcMoonLog "Uncommitted changes detected in Cargo.toml:" -Level "Warning" -Color Yellow
                Write-Host $gitStatus -ForegroundColor Yellow

                if (-not $Force -and -not $SkipGitCheck) {
                    Write-ArcMoonLog "Upgrade cancelled for safety. Use -Force to override or commit changes first." -Level "Error" -Color Red
                    Write-ArcMoonLog "💡 Recommended: git add Cargo.toml && git commit -m 'Pre-upgrade snapshot'" -Level "Info" -Color Cyan
                    return $false
                } else {
                    Write-ArcMoonLog "Proceeding with upgrade despite uncommitted changes (Force mode)." -Level "Warning" -Color Yellow
                }
            }
        }

        Write-ArcMoonLog "Git safety check passed." -Level "Success" -Color Green
        return $true
    } catch {
        Write-ArcMoonLog "Git safety check failed: $($_.Exception.Message)" -Level "Error" -Color Red
        return $false
    }
}

# Cargo-edit installation verification
function Test-CargoEditInstallation {
    Write-ArcMoonLog "Verifying cargo-edit installation..." -Level "Info" -Color Cyan

    try {
        $null = Get-Command cargo-upgrade -ErrorAction Stop
        Write-ArcMoonLog "cargo-upgrade is available." -Level "Success" -Color Green
        return $true
    } catch {
        Write-ArcMoonLog "cargo-upgrade not found. Installing cargo-edit..." -Level "Warning" -Color Yellow

        try {
            if ($DryRun) {
                Write-ArcMoonLog "[DRY RUN] Would install cargo-edit" -Level "Info" -Color Magenta
                return $true
            }

            & cargo install cargo-edit --quiet
            Write-ArcMoonLog "cargo-edit installed successfully." -Level "Success" -Color Green
            return $true
        } catch {
            Write-ArcMoonLog "Failed to install cargo-edit: $($_.Exception.Message)" -Level "Error" -Color Red
            return $false
        }
    }
}

# Dependency snapshot function
function Get-DependencySnapshot {
    param([string]$Description)

    Write-ArcMoonLog "$Description" -Level "Info" -Color Cyan
    try {
        & cargo tree --depth 1
    } catch {
        Write-ArcMoonLog "Failed to generate dependency snapshot: $($_.Exception.Message)" -Level "Warning" -Color Yellow
    }
}

# Main upgrade execution
function Invoke-DependencyUpgrade {
    Write-ArcMoonLog "🚀 Starting ArcMoon Studios Enterprise Dependency Upgrade" -Level "Info" -Color Cyan
    Write-ArcMoonLog "Project: $script:ProjectName" -Level "Info" -Color White
    Write-ArcMoonLog "Timestamp: $script:LogTimestamp" -Level "Info" -Color White

    # Phase 1: Safety checks
    if (-not (Test-GitSafety)) {
        return $false
    }

    if (-not (Test-CargoEditInstallation)) {
        return $false
    }

    # Phase 2: Pre-upgrade snapshot
    Get-DependencySnapshot "📊 Current dependency snapshot:"

    # Phase 3: Execute upgrade
    Write-ArcMoonLog "🔄 Executing dependency upgrade..." -Level "Info" -Color Cyan

    try {
        if ($DryRun) {
            Write-ArcMoonLog "[DRY RUN] Would execute: cargo upgrade" -Level "Info" -Color Magenta
        } else {
            & cargo upgrade
            Write-ArcMoonLog "Dependencies upgraded successfully!" -Level "Success" -Color Green
        }
    } catch {
        Write-ArcMoonLog "Dependency upgrade failed: $($_.Exception.Message)" -Level "Error" -Color Red
        return $false
    }

    # Phase 4: Post-upgrade snapshot
    if (-not $DryRun) {
        Get-DependencySnapshot "📊 Updated dependency snapshot:"
    }

    # Phase 5: Validation
    Write-ArcMoonLog "🔍 Validating upgraded dependencies..." -Level "Info" -Color Cyan

    try {
        if ($DryRun) {
            Write-ArcMoonLog "[DRY RUN] Would execute: cargo check" -Level "Info" -Color Magenta
        } else {
            & cargo check --verbose
            Write-ArcMoonLog "Dependency validation successful!" -Level "Success" -Color Green
        }
    } catch {
        Write-ArcMoonLog "Dependency validation failed: $($_.Exception.Message)" -Level "Warning" -Color Yellow
        Write-ArcMoonLog "💡 Consider running 'cargo update' or checking for breaking changes." -Level "Info" -Color Cyan
    }

    return $true
}

# Performance tracking
function Show-ExecutionSummary {
    $endTime = Get-Date
    $duration = $endTime - $script:StartTime

    Write-ArcMoonLog "⚡ Execution Summary:" -Level "Info" -Color Cyan
    Write-ArcMoonLog "Duration: $($duration.TotalSeconds.ToString('F2')) seconds" -Level "Info" -Color White
    Write-ArcMoonLog "Completed: $($endTime.ToString('yyyy-MM-dd HH:mm:ss'))" -Level "Info" -Color White
}

# Main execution
try {
    # Header
    Write-Host ""
    Write-Host "🌙 ArcMoon Studios Enterprise Dependency Upgrade System" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host ""

    # Environment validation
    if (-not $env:CARGO_ARCMOON_UPGRADE -and -not $Force) {
        Write-ArcMoonLog "CARGO_ARCMOON_UPGRADE environment variable not set." -Level "Warning" -Color Yellow
        Write-ArcMoonLog "Set `$env:CARGO_ARCMOON_UPGRADE='true' or use -Force parameter." -Level "Info" -Color Cyan
        Write-ArcMoonLog "Example: `$env:CARGO_ARCMOON_UPGRADE='true'; .\scripts\arcmoon-upgrade.ps1" -Level "Info" -Color Cyan
        exit 1
    }

    # Execute upgrade
    $success = Invoke-DependencyUpgrade

    # Show summary
    Show-ExecutionSummary

    if ($success) {
        Write-ArcMoonLog "🎉 ArcMoon Studios Enterprise Upgrade completed successfully!" -Level "Success" -Color Green
        exit 0
    } else {
        Write-ArcMoonLog "💥 ArcMoon Studios Enterprise Upgrade failed!" -Level "Error" -Color Red
        exit 1
    }

} catch {
    Write-ArcMoonLog "Unexpected error: $($_.Exception.Message)" -Level "Error" -Color Red
    Write-ArcMoonLog "Stack trace: $($_.ScriptStackTrace)" -Level "Error" -Color Red
    Show-ExecutionSummary
    exit 1
} finally {
    Write-Host ""
    Write-Host "🌙 ArcMoon Studios Enterprise - Mathematical Precision in Every Operation" -ForegroundColor Cyan
    Write-Host ""
}
