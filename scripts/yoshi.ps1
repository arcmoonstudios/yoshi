<#
  File:    yoshi\scripts\yoshi.ps1
  Purpose: Bootstrap the full Yoshi workspace on Windows 11
  Usage:   PS> .\scripts\yoshi.ps1            # first run
           PS> .\scripts\yoshi.ps1 -Force     # recreate blank placeholders
#>

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet('init', 'validate')]
    [string]$Command = 'init',

    [switch]$Force     # overwrite existing placeholder files
)

function ValidateForPublish {
    Write-Host "► Running crates.io publication validation checks..." -ForegroundColor Cyan

    # Format check
    Write-Host "`n⚡ Running cargo fmt check..." -ForegroundColor Yellow
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✖ Format check failed" -ForegroundColor Red
        return $false
    }

    # Clippy
    Write-Host "`n⚡ Running clippy..." -ForegroundColor Yellow
    cargo clippy --all-targets --all-features -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✖ Clippy check failed" -ForegroundColor Red
        return $false
    }

    # Tests
    Write-Host "`n⚡ Running tests..." -ForegroundColor Yellow
    cargo test --all-features
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✖ Tests failed" -ForegroundColor Red
        return $false
    }

    # Doc tests
    Write-Host "`n⚡ Running doc tests..." -ForegroundColor Yellow
    cargo test --doc --all-features
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✖ Doc tests failed" -ForegroundColor Red
        return $false
    }

    # Package verification
    Write-Host "`n⚡ Validating packages..." -ForegroundColor Yellow
    foreach ($pkg in @('yoshi-std', 'yoshi-derive', 'yoshi')) {
        Write-Host "  ► Checking $pkg..." -ForegroundColor Cyan
        cargo package --no-verify --allow-dirty -p $pkg
        if ($LASTEXITCODE -ne 0) {
            Write-Host "✖ Package validation failed for $pkg" -ForegroundColor Red
            return $false
        }
    }

    Write-Host "`n✔ All validation checks passed!" -ForegroundColor Green
    return $true
}

# ── Locate workspace root ─────────────────────────────────────────────────────
$RootPath = (Resolve-Path "$PSScriptRoot\..").ProviderPath
Write-Host "Workspace root  ➜  $RootPath`n"

# ── Main command handling ─────────────────────────────────────────────────────
switch ($Command) {
    'init' {
        # Original init logic continues below...
    }
    'validate' {
        ValidateForPublish
        if ($LASTEXITCODE -ne 0) { exit 1 }
        exit 0
    }
}

function Touch {
    param(
        [string]$RelPath,
        [string]$Seed = ''
    )
    $Abs = Join-Path $RootPath $RelPath
    if (Test-Path $Abs) {
        if ($Force) { Set-Content -Path $Abs -Value $Seed -NoNewline }
    }
    else {
        $null = New-Item -ItemType File -Path $Abs -Force
        if ($Seed) { Set-Content -Path $Abs -Value $Seed -NoNewline }
    }
}

# ── Directory list ────────────────────────────────────────────────────────────
$Dirs = @(
    '.cargo',
    '.github/workflows', '.github/ISSUE_TEMPLATE',
    '.vscode',
    'docs', 'examples',
    'yoshi-std/src', 'yoshi-derive/src', 'yoshi/src',
    'scripts'        # (holds this script already)
)

# ── Root-level placeholder files ──────────────────────────────────────────────
$RootFiles = @(
    '.gitignore', 'LICENSE', 'README.md',
    'Cargo.toml', 'rust-toolchain.toml',
    '.cargo/config.toml'
)

# ── CI workflow configuration ────────────────────────────────────────────────────────
$Workflows = @(
    '.github/workflows/ci.yml',
    '.github/workflows/release.yml'
)

# ── Crate placeholder files ──────────────────────────────────────────────────
$Crates = @{
    'yoshi-std'    = @('Cargo.toml', 'README.md', 'src/lib.rs')
    'yoshi-derive' = @('Cargo.toml', 'README.md', 'src/lib.rs')
    'yoshi'        = @('Cargo.toml', 'README.md', 'src/lib.rs')
}

# ── Create folders ───────────────────────────────────────────────────────────
Write-Host "► Creating directories …" -ForegroundColor Cyan
foreach ($d in $Dirs) {
    $Abs = Join-Path $RootPath $d
    if (Test-Path $Abs) {
        if ($Force) {
            Remove-Item -Recurse -Force $Abs
            $null = New-Item -ItemType Directory -Path $Abs
            Write-Host "  ± $d (reset)"
        }
    }
    else {
        $null = New-Item -ItemType Directory -Path $Abs -Force
        Write-Host "  + $d"
    }
}

# ── Touch root files ─────────────────────────────────────────────────────────
Write-Host "`n► Root files …" -ForegroundColor Cyan
foreach ($f in $RootFiles) { Touch $f }

# ── Touch crate placeholders ────────────────────────────────────────────────
Write-Host "`n► Crate files …" -ForegroundColor Cyan
foreach ($crate in $Crates.Keys) {
    foreach ($file in $Crates[$crate]) {
        Touch "$crate/$file"
    }
}

# ── CI workflow setup ────────────────────────────────────────────────────────
Write-Host "`n► Setting up CI workflows …" -ForegroundColor Cyan
foreach ($wf in $Workflows) { Touch $wf }

# ── Seed one-liners if brand-new ─────────────────────────────────────────────
if (-not $Force) {
    Touch 'README.md'               "# Yoshi – Structured Errors for Rust`n"
    Touch 'yoshi-std/README.md'     "# yoshi-std`n"
    Touch 'yoshi-derive/README.md'  "# yoshi-derive`n"
    Touch 'yoshi/README.md'         "# yoshi (facade crate)`n"
}

Write-Host "`n✔  Scaffold complete – happy hacking!" -ForegroundColor Green
