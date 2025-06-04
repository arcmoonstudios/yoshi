<#
  File:    yoshi\scripts\yoshi.ps1
  Purpose: Bootstrap, validate, and publish the Yoshi workspace (SSH-Exclusive)
  Usage:

  # Initialize workspace:
  PS> .\scripts\yoshi.ps1 init                     # first run
  PS> .\scripts\yoshi.ps1 init -Force              # recreate blank placeholders

  # Validate for publishing:
  PS> .\scripts\yoshi.ps1 validate                 # run all validation checks
  PS> .\scripts\yoshi.ps1 validate -SkipBenchmarks # skip benchmark checks
  PS> .\scripts\yoshi.ps1 validate -CommitMessage "My commit message" # custom commit message

  # Git operations (SSH only):
  PS> .\scripts\yoshi.ps1 git                     # interactive git commit and push
  PS> .\scripts\yoshi.ps1 git -CommitMessage "My commit message" # with custom commit message
  PS> .\scripts\yoshi.ps1 git -CommitMessage "My commit message" -DoPush # commit and push

  # Publish to crates.io:
  PS> .\scripts\yoshi.ps1 publish -DryRun          # dry run publishing sequence
  PS> .\scripts\yoshi.ps1 publish                  # publish to crates.io in correct order
  PS> .\scripts\yoshi.ps1 publish -SkipBenchmarks  # publish without running benchmarks
  PS> .\scripts\yoshi.ps1 publish "yoshi-std yoshi-derive yoshi" # specify packages to publish
  PS> .\scripts\yoshi.ps1 publish -DoPush          # publish and push via SSH
#>

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet('init', 'validate', 'publish', 'git')]
    [string]$Command = 'init',

    [switch]$Force, # overwrite existing placeholder files

    [switch]$SkipBenchmarks, # Skip benchmarks during validation

    [string]$CommitMessage = "", # Optional commit message

    [switch]$DryRun, # Perform dry-run for publish command

    [switch]$DoCommit, # Automatically commit changes

    [switch]$DoPush, # Push commits to remote

    [string]$Packages = ""  # Space-separated list of packages to publish
)

# ── Locate workspace root ─────────────────────────────────────────────────────
$RootPath = (Resolve-Path "$PSScriptRoot\..").ProviderPath
Write-Host "🦀 Yoshi Workspace Management (SSH-Exclusive) 🦀" -ForegroundColor Magenta
Write-Host "=================================================" -ForegroundColor Magenta
Write-Host "Workspace root  ➜  $RootPath`n"

# ── Helper functions ─────────────────────────────────────────────────────────
function Read-UserChoice {
    param(
        [string]$Prompt,
        [string]$DefaultChoice = "Y"
    )

    $defaultText = if ($DefaultChoice -eq "Y") { "[Y]es or [N]o (default: Y)" } else { "[Y]es or [N]o (default: N)" }

    # Check if input is being piped in
    if ([Console]::IsInputRedirected) {
        try {
            # Try to read from piped input but with a timeout
            $input_obj = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyUp")
            $choice = $input_obj.Character
        }
        catch {
            # If reading from pipe fails, use default
            return $DefaultChoice
        }
    }
    else {
        # Interactive mode with single key press (no Enter required)
        Write-Host "$Prompt $defaultText" -NoNewline
        $key = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyUp")
        $choice = $key.Character
        Write-Host $choice # Show the selected choice
    }

    if ([string]::IsNullOrEmpty($choice)) {
        return $DefaultChoice
    }

    return $choice
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

function Initialize-UserOptions {
    param(
        [string]$Command,
        [switch]$SkipBenchmarks,
        [string]$CommitMessage,
        [switch]$DryRun,
        [switch]$DoCommit,
        [switch]$DoPush,
        [string]$Packages
    )

    $options = @{
        SkipBenchmarks  = $SkipBenchmarks.IsPresent
        CommitMessage   = $CommitMessage
        DryRun          = $DryRun.IsPresent
        DoCommit        = $DoCommit.IsPresent
        DoPush          = $DoPush.IsPresent
        DoPublish       = $false
        PublishPackages = @()
    }

    # Process command line package list if provided
    if (-not [string]::IsNullOrEmpty($Packages)) {
        $options.PublishPackages = $Packages -split '\s+'
    }

    # Only ask for options that weren't explicitly provided via parameters
    switch ($Command) {
        'validate' {
            if (-not $PSBoundParameters.ContainsKey('SkipBenchmarks')) {
                Write-Host "`n📋 VALIDATION OPTIONS" -ForegroundColor Cyan
                $benchChoice = Read-UserChoice "Skip benchmark compilation?" "N"
                $options.SkipBenchmarks = ($benchChoice -eq 'y' -or $benchChoice -eq 'Y')
            }

            if (-not $PSBoundParameters.ContainsKey('DoCommit')) {
                $commitChoice = Read-UserChoice "Commit changes after validation?"
                $options.DoCommit = ($commitChoice -eq 'y' -or $commitChoice -eq 'Y')
            }

            if ($options.DoCommit -and [string]::IsNullOrEmpty($CommitMessage)) {
                Write-Host "`nEnter commit message (leave blank for default):" -ForegroundColor Cyan
                $userMessage = Read-Host
                if (-not [string]::IsNullOrEmpty($userMessage)) {
                    $options.CommitMessage = $userMessage
                }
                else {
                    $options.CommitMessage = "Pre-publish validation - All checks passed"
                }
            }

            if ($options.DoCommit -and -not $PSBoundParameters.ContainsKey('DoPush')) {
                $pushChoice = Read-UserChoice "Push changes after commit?"
                $options.DoPush = ($pushChoice -eq 'y' -or $pushChoice -eq 'Y')
            }
        }
        'publish' {
            Write-Host "`n📦 PUBLISHING OPTIONS" -ForegroundColor Cyan

            if (-not $PSBoundParameters.ContainsKey('DryRun')) {
                $dryRunChoice = Read-UserChoice "Perform dry run only (no actual publishing)?"
                $options.DryRun = ($dryRunChoice -eq 'y' -or $dryRunChoice -eq 'Y')
            }

            if (-not $PSBoundParameters.ContainsKey('SkipBenchmarks')) {
                $benchChoice = Read-UserChoice "Skip benchmark compilation?" "N"
                $options.SkipBenchmarks = ($benchChoice -eq 'y' -or $benchChoice -eq 'Y')
            }

            if (-not $PSBoundParameters.ContainsKey('DoCommit')) {
                $commitChoice = Read-UserChoice "Commit changes before publishing?"
                $options.DoCommit = ($commitChoice -eq 'y' -or $commitChoice -eq 'Y')
            }

            if ($options.DoCommit -and [string]::IsNullOrEmpty($CommitMessage)) {
                Write-Host "`nEnter commit message (leave blank for default):" -ForegroundColor Cyan
                $userMessage = Read-Host
                if (-not [string]::IsNullOrEmpty($userMessage)) {
                    $options.CommitMessage = $userMessage
                }
                else {
                    $options.CommitMessage = "Pre-publish preparation - All checks passed"
                }
            }

            if ($options.DoCommit -and -not $PSBoundParameters.ContainsKey('DoPush')) {
                $pushChoice = Read-UserChoice "Push changes after commit?"
                $options.DoPush = ($pushChoice -eq 'y' -or $pushChoice -eq 'Y')
            }

            # Select packages to publish if not specified via command line
            if ($options.PublishPackages.Count -eq 0) {
                $defaultPackages = @('yoshi-std', 'yoshi-derive', 'yoshi')
                Write-Host "`nSelect packages to publish (in the correct order: yoshi-std → yoshi-derive → yoshi):" -ForegroundColor Cyan
                foreach ($pkg in $defaultPackages) {
                    $pkgChoice = Read-UserChoice "Include $pkg?"
                    if ($pkgChoice -eq 'y' -or $pkgChoice -eq 'Y') {
                        $options.PublishPackages += $pkg
                    }
                }
            }

            # Final confirmation for publishing
            if ($options.PublishPackages.Count -gt 0 -and -not $options.DryRun) {
                $options.DoPublish = $true
                Write-Host "`n⚠️ WARNING: You've selected to publish the following packages:" -ForegroundColor Red
                foreach ($pkg in $options.PublishPackages) {
                    Write-Host "   - $pkg" -ForegroundColor Yellow
                }

                $finalConfirm = Read-UserChoice "Are you absolutely sure you want to publish these packages?" "N"
                $options.DoPublish = ($finalConfirm -eq 'y' -or $finalConfirm -eq 'Y')

                if (-not $options.DoPublish) {
                    $options.DryRun = $true
                    Write-Host "Switched to dry-run mode for safety." -ForegroundColor Yellow
                }
            }
        }
    }

    return $options
}

function ValidateForPublish {
    param(
        [switch]$SkipBenchmarks
    )
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

    # Benchmarks (can be skipped)
    if (-not $SkipBenchmarks) {
        Write-Host "`n⚡ Running benchmarks (no-run)..." -ForegroundColor Yellow
        cargo bench --no-run
        if ($LASTEXITCODE -ne 0) {
            Write-Host "✖ Benchmark compilation failed" -ForegroundColor Red
            return $false
        }
    }
    else {
        Write-Host "`n⚡ Benchmarks skipped..." -ForegroundColor Yellow
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

function ConvertToSSHUrl {
    param(
        [string]$Url
    )

    # Check if the URL is HTTPS GitHub URL
    if ($Url -match '^https?://github.com/(.+?)(?:\.git)?$') {
        $RepoPath = $matches[1]
        # Ensure .git extension
        if (-not $RepoPath.EndsWith('.git')) {
            $RepoPath = "$RepoPath.git"
        }
        return "git@github.com:$RepoPath"
    }

    # Already SSH or other format, return as-is
    return $Url
}

function EnsureSSHRemote {
    Write-Host "`n🔐 Ensuring GitHub remote uses SSH..." -ForegroundColor Cyan

    # Get current remote URL
    $currentUrl = git remote get-url origin 2>$null

    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️ No remote named 'origin' found." -ForegroundColor Yellow
        return $false
    }

    # Check if it's already SSH
    if ($currentUrl -match '^git@github\.com:.+') {
        Write-Host "✓ Remote already using SSH: $currentUrl" -ForegroundColor Green
        return $true
    }

    # Convert to SSH
    $sshUrl = ConvertToSSHUrl -Url $currentUrl

    if ($sshUrl -eq $currentUrl) {
        Write-Host "✓ Remote URL doesn't need conversion: $currentUrl" -ForegroundColor Green
        return $true
    }

    # Update the remote URL
    Write-Host "► Converting remote URL to SSH..." -ForegroundColor Cyan
    git remote set-url origin $sshUrl

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Successfully updated remote URL to: $sshUrl" -ForegroundColor Green
        return $true
    }
    else {
        Write-Host "✖ Failed to update remote URL" -ForegroundColor Red
        return $false
    }
}

function GitCommitAndPush {
    param(
        [string]$CommitMessage,
        [bool]$DoPush = $false
    )

    # Always ensure SSH remote for this script
    $sshResult = EnsureSSHRemote
    if (-not $sshResult -and $DoPush) {
        Write-Host "⚠️ Warning: Could not ensure SSH remote. Push may fail." -ForegroundColor Yellow
    }

    # Check if there are any changes
    $status = git status --porcelain
    if (-not $status) {
        Write-Host "`n📝 No changes to commit - repository is clean." -ForegroundColor Cyan

        # Check if there are unpushed commits
        $unpushedCount = (git rev-list --count HEAD ^origin/HEAD) 2>$null

        if ($unpushedCount -gt 0 -and $DoPush) {
            Write-Host "🚀 Found $unpushedCount unpushed commit(s). Pushing..." -ForegroundColor Cyan
            git push
            if ($LASTEXITCODE -eq 0) {
                Write-Host "✔ Successfully pushed commits. CI has been triggered!" -ForegroundColor Green
            }
            else {
                Write-Host "✖ Push failed" -ForegroundColor Red
                return $false
            }
        }

        return $true
    }

    # Show changes
    Write-Host "`n📝 Changes to commit:" -ForegroundColor Cyan
    git status --short

    # Add and commit
    Write-Host "`n► Adding changes..." -ForegroundColor Cyan
    git add .
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✖ Git add failed" -ForegroundColor Red
        return $false
    }

    Write-Host "► Committing with message: $CommitMessage" -ForegroundColor Cyan
    git commit -m $CommitMessage
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✖ Git commit failed" -ForegroundColor Red
        return $false
    }

    # Push if requested
    if ($DoPush) {
        Write-Host "`n► Pushing commits to trigger CI..." -ForegroundColor Cyan
        git push
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✔ Successfully pushed commits. CI has been triggered!" -ForegroundColor Green
            return $true
        }
        else {
            Write-Host "✖ Push failed" -ForegroundColor Red
            return $false
        }
    }
    else {
        Write-Host "✔ Changes committed locally. Don't forget to push later to trigger CI." -ForegroundColor Green
    }

    return $true
}

function PublishCrates {
    param(
        [switch]$DryRun,
        [array]$Packages = @('yoshi-std', 'yoshi-derive', 'yoshi')
    )

    Write-Host "`n📦 Publishing crates in sequence" -ForegroundColor Cyan

    if ($Packages.Count -eq 0) {
        Write-Host "⚠️  No packages selected for publishing" -ForegroundColor Yellow
        return $true
    }

    Write-Host "   Selected packages: $($Packages -join ' → ')" -ForegroundColor Cyan

    if ($DryRun) {
        Write-Host "`n🔍 DRY RUN mode enabled - no actual publishing will occur" -ForegroundColor Yellow
    }
    else {
        Write-Host "`n⚠️  PRODUCTION mode - crates will be published to crates.io" -ForegroundColor Red
    }

    # Process each package
    foreach ($package in $Packages) {
        Write-Host "`n► Processing package: $package" -ForegroundColor Cyan

        # Perform dry-run check first
        Write-Host "   Running validation check: cargo publish --dry-run -p $package" -ForegroundColor Yellow
        Invoke-Expression "cargo publish --dry-run -p $package"

        if ($LASTEXITCODE -ne 0) {
            Write-Host "`n✖ Package $package failed dry-run validation" -ForegroundColor Red

            # For both dry-run and real mode, show a continue option
            $retry = Read-UserChoice "Try next package anyway?" "N"
            if ($retry -ne 'y' -and $retry -ne 'Y') {
                Write-Host "✖ Publishing sequence aborted" -ForegroundColor Red
                return $false
            }
            Write-Host "⚠️  Continuing to next package despite errors" -ForegroundColor Yellow
            continue
        }

        # If not just a dry run, do actual publish
        if (-not $DryRun) {
            Write-Host "   Publishing: cargo publish -p $package" -ForegroundColor Yellow
            Invoke-Expression "cargo publish -p $package"

            if ($LASTEXITCODE -eq 0) {
                Write-Host "✔ Successfully published $package to crates.io!" -ForegroundColor Green

                if ($package -ne $Packages[-1]) {
                    Write-Host "`nWaiting 20 seconds for crates.io to update index..." -ForegroundColor Yellow
                    Start-Sleep -Seconds 20
                }
            }
            else {
                Write-Host "✖ Failed to publish $package" -ForegroundColor Red

                $retry = Read-UserChoice "Try next package anyway?" "N"
                if ($retry -ne 'y' -and $retry -ne 'Y') {
                    Write-Host "✖ Publishing sequence aborted" -ForegroundColor Red
                    return $false
                }
                Write-Host "⚠️  Continuing to next package despite errors" -ForegroundColor Yellow
            }
        }
    }

    if ($DryRun) {
        Write-Host "`n✔ Dry run completed successfully" -ForegroundColor Green
    }
    else {
        Write-Host "`n✔ Publishing sequence completed" -ForegroundColor Green
    }

    return $true
}

# ── Show interactive menu if no specific command was provided ─────────────────
function Show-InteractiveMenu {
    Write-Host "`n📋 YOSHI COMMAND MENU" -ForegroundColor Cyan
    Write-Host "Please select an operation (press a single key):" -ForegroundColor Cyan
    Write-Host "  [1] Initialize workspace (create folder structure)"
    Write-Host "  [2] Validate for publishing"
    Write-Host "  [3] Add, Commit, & Push to Github (SSH)"
    Write-Host "  [4] Publish to crates.io"
    Write-Host "  [q] Quit"

    Write-Host "`nYour choice: " -NoNewline
    $key = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyUp")
    $choice = $key.Character
    Write-Host $choice

    switch ($choice) {
        "1" { return "init" }
        "2" { return "validate" }
        "3" { return "git" }
        "4" { return "publish" }
        "q" { exit 0 }
        default {
            Write-Host "`n❌ Invalid choice. Please try again." -ForegroundColor Red
            return Show-InteractiveMenu
        }
    }
}

# ── Main command handling ─────────────────────────────────────────────────────
# Check if script was run without specifying a command (just ./yoshi.ps1)
if ($MyInvocation.BoundParameters.Count -eq 0 -and $args.Count -eq 0) {
    $Command = Show-InteractiveMenu
}

# Process each command with proper interactive options
switch ($Command) {
    'init' {
        Write-Host "`n📂 INITIALIZATION OPTIONS" -ForegroundColor Cyan
        # Ask if user wants to force reinitialize
        if (-not $PSBoundParameters.ContainsKey('Force')) {
            $forceChoice = Read-UserChoice "Recreate existing placeholder files?"
            $Force = ($forceChoice -eq 'y' -or $forceChoice -eq 'Y')
        }

        # Display execution plan
        Write-Host "`n🔄 EXECUTION PLAN:" -ForegroundColor Magenta
        Write-Host "   > Create workspace directory structure"
        if ($Force) {
            Write-Host "   > Recreate ALL existing files (destructive)"
        }
        else {
            Write-Host "   > Create only missing files (non-destructive)"
        }

        # Confirm execution
        Write-Host "`n⚙️ Ready to execute the plan" -ForegroundColor Cyan
        $confirmExecution = Read-UserChoice "Proceed with initialization?" "Y"
        if ($confirmExecution -eq 'n' -or $confirmExecution -eq 'N') {
            Write-Host "`n❌ Initialization cancelled by user" -ForegroundColor Yellow
            exit 0
        }

        Write-Host "`n► Initializing Yoshi workspace..." -ForegroundColor Cyan
        # Init logic continues below...
    }
    'validate' {
        # Collect all user options upfront
        $userOptions = Initialize-UserOptions -Command 'validate' -SkipBenchmarks:$SkipBenchmarks -CommitMessage $CommitMessage -DoCommit:$DoCommit -DoPush:$DoPush

        # Display execution plan
        Write-Host "`n🔄 EXECUTION PLAN:" -ForegroundColor Magenta
        Write-Host "   > Run validation checks"
        if ($userOptions.SkipBenchmarks) {
            Write-Host "   > Skip benchmark compilation"
        }
        if ($userOptions.DoCommit) {
            Write-Host "   > Commit changes with message: '$($userOptions.CommitMessage)'"
            if ($userOptions.DoPush) {
                Write-Host "   > Push changes to remote (SSH)"
            }
        }

        # Confirm execution
        Write-Host "`n⚙️ Ready to execute the plan" -ForegroundColor Cyan
        $confirmExecution = Read-UserChoice "Proceed with execution?" "Y"
        if ($confirmExecution -eq 'n' -or $confirmExecution -eq 'N') {
            Write-Host "`n❌ Execution cancelled by user" -ForegroundColor Yellow
            exit 0
        }

        # Run validation with collected options
        $validationResult = ValidateForPublish -SkipBenchmarks:$userOptions.SkipBenchmarks

        if (-not $validationResult) {
            Write-Host "`n❌ Validation failed. Fix errors before proceeding." -ForegroundColor Red
            exit 1
        }

        Write-Host "`n✅ Validation succeeded!" -ForegroundColor Green

        # Handle Git operations
        if ($userOptions.DoCommit) {
            $commitResult = GitCommitAndPush -CommitMessage $userOptions.CommitMessage -DoPush $userOptions.DoPush

            if (-not $commitResult) {
                Write-Host "`n❌ Git operations failed." -ForegroundColor Red
                exit 1
            }
        }

        Write-Host "`n✨ Validation process completed successfully!" -ForegroundColor Green
        exit 0
    }
    'git' {
        # Git operations mode - always uses SSH
        Write-Host "`n📝 GIT OPERATIONS (SSH)" -ForegroundColor Cyan

        # Get commit message
        if ([string]::IsNullOrEmpty($CommitMessage)) {
            Write-Host "`nEnter commit message:" -ForegroundColor Cyan
            $commitInput = Read-Host
            if ([string]::IsNullOrEmpty($commitInput)) {
                $CommitMessage = "Update project files - $(Get-Date -Format 'yyyy-MM-dd HH:mm')"
                Write-Host "Using default commit message: $CommitMessage" -ForegroundColor Yellow
            } else {
                $CommitMessage = $commitInput
            }
        }

        # Ask about pushing
        if (-not $PSBoundParameters.ContainsKey('DoPush')) {
            $pushChoice = Read-UserChoice "Push changes after commit?"
            $DoPush = ($pushChoice -eq 'y' -or $pushChoice -eq 'Y')
        }

        # Display execution plan
        Write-Host "`n🔄 EXECUTION PLAN:" -ForegroundColor Magenta
        Write-Host "   > Ensure SSH remote is configured"
        Write-Host "   > Add and commit changes with message: '$CommitMessage'"
        if ($DoPush) {
            Write-Host "   > Push changes to remote (SSH)"
        }

        # Confirm execution
        Write-Host "`n⚙️ Ready to execute the plan" -ForegroundColor Cyan
        $confirmExecution = Read-UserChoice "Proceed with Git operations?" "Y"
        if ($confirmExecution -eq 'n' -or $confirmExecution -eq 'N') {
            Write-Host "`n❌ Git operations cancelled by user" -ForegroundColor Yellow
            exit 0
        }

        # Execute Git operations
        $commitResult = GitCommitAndPush -CommitMessage $CommitMessage -DoPush $DoPush

        if (-not $commitResult) {
            Write-Host "`n❌ Git operations failed." -ForegroundColor Red
            exit 1
        }

        Write-Host "`n✨ Git operations completed successfully!" -ForegroundColor Green
        exit 0
    }
    'publish' {
        # Collect all user options upfront
        $userOptions = Initialize-UserOptions -Command 'publish' -SkipBenchmarks:$SkipBenchmarks -CommitMessage $CommitMessage -DryRun:$DryRun -DoCommit:$DoCommit -DoPush:$DoPush -Packages $Packages

        # Display execution plan
        Write-Host "`n🔄 EXECUTION PLAN:" -ForegroundColor Magenta
        Write-Host "   > Run validation checks"
        if ($userOptions.SkipBenchmarks) {
            Write-Host "   > Skip benchmark compilation"
        }
        if ($userOptions.DoCommit) {
            Write-Host "   > Commit changes with message: '$($userOptions.CommitMessage)'"
            if ($userOptions.DoPush) {
                Write-Host "   > Push changes to remote (SSH)"
            }
        }

        if ($userOptions.PublishPackages.Count -gt 0) {
            if ($userOptions.DryRun) {
                Write-Host "   > Dry-run publish for packages:"
            }
            else {
                Write-Host "   > Publish packages to crates.io:"
            }
            foreach ($pkg in $userOptions.PublishPackages) {
                Write-Host "      - $pkg"
            }
        }

        # Confirm execution
        Write-Host "`n⚙️ Ready to execute the plan" -ForegroundColor Cyan
        $confirmExecution = Read-UserChoice "Proceed with execution?" "Y"
        if ($confirmExecution -eq 'n' -or $confirmExecution -eq 'N') {
            Write-Host "`n❌ Execution cancelled by user" -ForegroundColor Yellow
            exit 0
        }

        # Run validation
        Write-Host "► Running pre-publish validation..." -ForegroundColor Cyan
        $validationResult = ValidateForPublish -SkipBenchmarks:$userOptions.SkipBenchmarks

        if (-not $validationResult) {
            Write-Host "`n❌ Validation failed. Fix errors before publishing." -ForegroundColor Red
            exit 1
        }

        # Handle Git operations if requested
        if ($userOptions.DoCommit) {
            $commitResult = GitCommitAndPush -CommitMessage $userOptions.CommitMessage -DoPush $userOptions.DoPush

            if (-not $commitResult) {
                Write-Host "`n❌ Git operations failed." -ForegroundColor Red
                $proceed = Read-UserChoice "Continue with publishing anyway?" "N"
                if ($proceed -ne 'y' -and $proceed -ne 'Y') {
                    Write-Host "`n❌ Publishing aborted." -ForegroundColor Red
                    exit 1
                }
                Write-Host "Continuing despite Git issues..." -ForegroundColor Yellow
            }
        }

        # Proceed with publishing
        if ($userOptions.PublishPackages.Count -gt 0) {
            PublishCrates -DryRun:$userOptions.DryRun -Packages $userOptions.PublishPackages
        }
        else {
            Write-Host "`n⚠️ No packages selected for publishing." -ForegroundColor Yellow
        }

        exit 0
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
