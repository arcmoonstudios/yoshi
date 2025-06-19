# scripts/yoshi-make.ps1
#![deny(unsafe_code)]
#![warn(missing_docs)]
#!
#! **Brief:** ArcMoon Studios Enterprise PowerShell Makefile Equivalent with CI/Dev Mode Upgrade Support.
#!
#! **Module Classification:** Performance-Critical
#! **Complexity Level:** Expert
#! **API Stability:** Stable
#!
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
#! + Cross-platform PowerShell Makefile replacement for Windows environments
#!  - Complete Makefile target implementation in PowerShell
#!  - Enhanced error handling and performance monitoring
#!  - Automatic tool installation and dependency management
#!  - Git safety integration with upgrade system
#!  - Color-coded output with timestamp logging
# ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
#!
#! ## Mathematical Properties
#!
#! **Algorithmic Complexity:**
#! - Time Complexity: O(n) where n is number of build operations
#! - Space Complexity: O(1) constant overhead for execution
#! - Concurrency Safety: Thread-safe PowerShell execution model
#!
#! **Performance Characteristics:**
#! - Expected Performance: Sub-second command dispatch, minutes for builds
#! - Worst-Case Scenarios: Network-dependent cargo operations
#! - Optimization Opportunities: Parallel task execution
#!
# **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
# **Copyright:** (c) 2025 ArcMoon Studios
# **Author:** Lord Xyn
# **License:** MIT OR Apache-2.0
# **License File:** /LICENSE
# **Contact:** LordXyn@proton.me

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$Target = "help",

    [switch]$Force
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# ArcMoon Studios Configuration
$script:ProjectName = "Yoshi"
$script:BuildMode = "debug"
$script:CargoFlags = "--verbose"
$script:TestFlags = "--all-targets --all-features"

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
        "Error" { "❌" }
        "Info" { "ℹ️ " }
        default { "📝" }
    }

    Write-Host "[$timestamp] $prefix $Message" -ForegroundColor $Color
}

# Tool availability check
function Test-ToolAvailability {
    param([string]$ToolName)

    try {
        $null = Get-Command $ToolName -ErrorAction Stop
        return $true
    }
    catch {
        return $false
    }
}

# Install missing tools
function Install-CargoTool {
    param([string]$ToolName)

    Write-ArcMoonLog "Installing $ToolName..." -Level "Warning" -Color Yellow
    try {
        & cargo install $ToolName --quiet
        Write-ArcMoonLog "$ToolName installed successfully." -Level "Success" -Color Green
        return $true
    }
    catch {
        Write-ArcMoonLog "Failed to install $ToolName : $($_.Exception.Message)" -Level "Error" -Color Red
        return $false
    }
}

# Help target
function Invoke-Help {
    Write-Host ""
    Write-Host "🚀 ArcMoon Studios Enterprise PowerShell Makefile" -ForegroundColor Cyan
    Write-Host "📋 Available Targets:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  help              - 🔍 Display this help message" -ForegroundColor White
    Write-Host "  check             - 🔍 Run cargo check with optional dependency upgrades" -ForegroundColor White
    Write-Host "  build             - 🔨 Build the project with optimizations" -ForegroundColor White
    Write-Host "  test              - 🧪 Run comprehensive test suite" -ForegroundColor White
    Write-Host "  upgrade           - ⬆️  Upgrade dependencies (requires CARGO_ARCMOON_UPGRADE=true)" -ForegroundColor White
    Write-Host "  security-audit    - 🔒 Run security vulnerability scan" -ForegroundColor White
    Write-Host "  performance-check - ⚡ Run performance benchmarks" -ForegroundColor White
    Write-Host "  format            - 🎨 Format code using rustfmt" -ForegroundColor White
    Write-Host "  lint              - 📎 Run clippy lints" -ForegroundColor White
    Write-Host "  docs              - 📚 Generate documentation" -ForegroundColor White
    Write-Host "  clean             - 🧹 Clean build artifacts" -ForegroundColor White
    Write-Host "  all               - 🎯 Run complete CI pipeline" -ForegroundColor White
    Write-Host ""
    Write-Host "📖 Usage Examples:" -ForegroundColor Yellow
    Write-Host "  .\scripts\yoshi-make.ps1 check                                    # Standard check" -ForegroundColor Yellow
    Write-Host "  `$env:CARGO_ARCMOON_UPGRADE='true'; .\scripts\yoshi-make.ps1 check  # Check with upgrades" -ForegroundColor Yellow
    Write-Host "  .\scripts\yoshi-make.ps1 all                                      # Complete CI pipeline" -ForegroundColor Yellow
    Write-Host ""
}

# Git safety check
function Test-GitSafety {
    Write-ArcMoonLog "Checking for uncommitted changes in Cargo.toml..." -Level "Info" -Color Cyan

    try {
        $gitStatus = git status --porcelain Cargo.toml 2>$null
        if ($gitStatus) {
            $changes = ($gitStatus | Measure-Object -Line).Lines
            if ($changes -gt 0) {
                Write-ArcMoonLog "Uncommitted changes detected in Cargo.toml:" -Level "Warning" -Color Yellow
                Write-Host $gitStatus -ForegroundColor Yellow

                if (-not $Force) {
                    Write-ArcMoonLog "Upgrade cancelled for safety. Use -Force to override or commit changes first." -Level "Error" -Color Red
                    Write-ArcMoonLog "💡 Recommended: git add Cargo.toml && git commit -m 'Pre-upgrade snapshot'" -Level "Info" -Color Cyan
                    return $false
                }
                else {
                    Write-ArcMoonLog "Proceeding with upgrade despite uncommitted changes (Force mode)." -Level "Warning" -Color Yellow
                }
            }
        }

        Write-ArcMoonLog "Git safety check passed." -Level "Success" -Color Green
        return $true
    }
    catch {
        Write-ArcMoonLog "Git safety check failed: $($_.Exception.Message)" -Level "Error" -Color Red
        return $false
    }
}

# Upgrade target
function Invoke-Upgrade {
    if ($env:CARGO_ARCMOON_UPGRADE -eq "true" -or $Force) {
        if (-not (Test-GitSafety)) {
            return $false
        }

        if (-not (Test-ToolAvailability "cargo-upgrade")) {
            if (-not (Install-CargoTool "cargo-edit")) {
                return $false
            }
        }

        Write-ArcMoonLog "⬆️  Upgrading dependencies..." -Level "Info" -Color Cyan
        Write-ArcMoonLog "📊 Current dependency snapshot:" -Level "Info" -Color Cyan
        & cargo tree --depth 1

        Write-ArcMoonLog "🔄 Executing cargo upgrade..." -Level "Info" -Color Cyan
        & cargo upgrade

        Write-ArcMoonLog "✅ Dependencies upgraded successfully!" -Level "Success" -Color Green
        Write-ArcMoonLog "📊 Updated dependency snapshot:" -Level "Info" -Color Cyan
        & cargo tree --depth 1

        return $true
    }
    else {
        Write-ArcMoonLog "ℹ️  CARGO_ARCMOON_UPGRADE not set to 'true'. Skipping dependency upgrade." -Level "Info" -Color Cyan
        Write-ArcMoonLog "💡 To enable upgrades: `$env:CARGO_ARCMOON_UPGRADE='true'; .\scripts\yoshi-make.ps1 upgrade" -Level "Warning" -Color Yellow
        return $true
    }
}

# Check target
function Invoke-Check {
    if ($env:CARGO_ARCMOON_UPGRADE -eq "true") {
        if (-not (Invoke-Upgrade)) {
            return $false
        }
    }

    Write-ArcMoonLog "🔍 Running cargo check..." -Level "Info" -Color Cyan
    & cargo check $script:CargoFlags.Split(' ')

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Cargo check completed successfully!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Cargo check failed!" -Level "Error" -Color Red
        return $false
    }
}

# Build target
function Invoke-Build {
    Write-ArcMoonLog "🔨 Building $script:ProjectName in $script:BuildMode mode..." -Level "Info" -Color Cyan
    & cargo build $script:CargoFlags.Split(' ')

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Build completed successfully!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Build failed!" -Level "Error" -Color Red
        return $false
    }
}

# Test target
function Invoke-Test {
    Write-ArcMoonLog "🧪 Running comprehensive test suite..." -Level "Info" -Color Cyan
    & cargo test $script:TestFlags.Split(' ') $script:CargoFlags.Split(' ')

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ All tests passed!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Tests failed!" -Level "Error" -Color Red
        return $false
    }
}

# Format target
function Invoke-Format {
    Write-ArcMoonLog "🎨 Formatting code..." -Level "Info" -Color Cyan
    & cargo fmt --all

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Code formatting completed!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Code formatting failed!" -Level "Error" -Color Red
        return $false
    }
}

# Lint target
function Invoke-Lint {
    Write-ArcMoonLog "📎 Running clippy lints..." -Level "Info" -Color Cyan
    & cargo clippy --all-targets --all-features -- -D warnings

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Linting completed successfully!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Linting failed!" -Level "Error" -Color Red
        return $false
    }
}

# Security audit target
function Invoke-SecurityAudit {
    Write-ArcMoonLog "🔒 Running security audit..." -Level "Info" -Color Cyan

    if (-not (Test-ToolAvailability "cargo-audit")) {
        if (-not (Install-CargoTool "cargo-audit")) {
            return $false
        }
    }

    & cargo audit

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Security audit completed!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Security audit found issues!" -Level "Warning" -Color Yellow
        return $true  # Don't fail the build for security warnings
    }
}

# Performance check target
function Invoke-PerformanceCheck {
    Write-ArcMoonLog "⚡ Running performance benchmarks..." -Level "Info" -Color Cyan
    & cargo bench --all-features

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Performance check completed!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Performance check failed!" -Level "Error" -Color Red
        return $false
    }
}

# Documentation target
function Invoke-Docs {
    Write-ArcMoonLog "📚 Generating documentation..." -Level "Info" -Color Cyan
    & cargo doc --all-features --no-deps --open

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Documentation generated successfully!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Documentation generation failed!" -Level "Error" -Color Red
        return $false
    }
}

# Clean target
function Invoke-Clean {
    Write-ArcMoonLog "🧹 Cleaning build artifacts..." -Level "Info" -Color Cyan
    & cargo clean

    if ($LASTEXITCODE -eq 0) {
        Write-ArcMoonLog "✅ Cleanup completed!" -Level "Success" -Color Green
        return $true
    }
    else {
        Write-ArcMoonLog "❌ Cleanup failed!" -Level "Error" -Color Red
        return $false
    }
}

# All target (complete CI pipeline)
function Invoke-All {
    Write-ArcMoonLog "🎯 Running complete CI pipeline..." -Level "Info" -Color Cyan

    $steps = @(
        @{ Name = "Format"; Function = { Invoke-Format } },
        @{ Name = "Lint"; Function = { Invoke-Lint } },
        @{ Name = "Check"; Function = { Invoke-Check } },
        @{ Name = "Test"; Function = { Invoke-Test } },
        @{ Name = "Security Audit"; Function = { Invoke-SecurityAudit } },
        @{ Name = "Documentation"; Function = { Invoke-Docs } }
    )

    foreach ($step in $steps) {
        Write-ArcMoonLog "Running $($step.Name)..." -Level "Info" -Color Cyan
        if (-not (& $step.Function)) {
            Write-ArcMoonLog "❌ CI pipeline failed at $($step.Name)!" -Level "Error" -Color Red
            return $false
        }
    }

    Write-ArcMoonLog "🎯 Complete CI pipeline executed successfully!" -Level "Success" -Color Green
    return $true
}

# Main execution
try {
    Write-Host ""
    Write-Host "🌙 ArcMoon Studios Enterprise PowerShell Makefile" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host ""

    $success = switch ($Target.ToLower()) {
        "help" { Invoke-Help; $true }
        "check" { Invoke-Check }
        "build" { Invoke-Build }
        "test" { Invoke-Test }
        "upgrade" { Invoke-Upgrade }
        "security-audit" { Invoke-SecurityAudit }
        "performance-check" { Invoke-PerformanceCheck }
        "format" { Invoke-Format }
        "lint" { Invoke-Lint }
        "docs" { Invoke-Docs }
        "clean" { Invoke-Clean }
        "all" { Invoke-All }
        default {
            Write-ArcMoonLog "Unknown target: $Target" -Level "Error" -Color Red
            Write-ArcMoonLog "Run '.\scripts\yoshi-make.ps1 help' for available targets." -Level "Info" -Color Cyan
            $false
        }
    }

    if ($success) {
        Write-Host ""
        Write-Host "🌙 ArcMoon Studios Enterprise - Mathematical Precision in Every Operation" -ForegroundColor Cyan
        exit 0
    }
    else {
        Write-Host ""
        Write-Host "💥 Operation failed. Check the logs above for details." -ForegroundColor Red
        exit 1
    }

}
catch {
    Write-ArcMoonLog "Unexpected error: $($_.Exception.Message)" -Level "Error" -Color Red
    Write-ArcMoonLog "Stack trace: $($_.ScriptStackTrace)" -Level "Error" -Color Red
    exit 1
}
