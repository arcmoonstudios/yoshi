#!/usr/bin/env python3
"""
CrateCheck - Comprehensive Rust Crate Quality Validation Script
Runs all essential checks required for professional crates.io release

Author: ArcMoon Studios
License: MIT
"""

import sys
import time
import subprocess
from pathlib import Path
from typing import List, Tuple

class Colors:
    """ANSI color codes for terminal output"""
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    WHITE = '\033[97m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    END = '\033[0m'

class CrateChecker:
    """Main class for running crate quality checks"""

    def __init__(self):
        self.passed_checks = 0
        self.total_checks = 0
        self.failed_checks = []
        self.warnings = []

    def print_header(self, title: str):
        """Print a formatted section header"""
        print(f"\n{Colors.CYAN}{Colors.BOLD}{'='*60}{Colors.END}")
        print(f"{Colors.CYAN}{Colors.BOLD}{title.center(60)}{Colors.END}")
        print(f"{Colors.CYAN}{Colors.BOLD}{'='*60}{Colors.END}")

    def print_step(self, step: str):
        """Print a step description"""
        print(f"\n{Colors.BLUE}🔍 {step}{Colors.END}")

    def run_command(self, cmd: List[str], description: str, critical: bool = True) -> Tuple[bool, str]:
        """Run a command and return success status and output"""
        self.total_checks += 1

        try:
            print(f"   Running: {' '.join(cmd)}")
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=300  # 5 minute timeout
            )

            if result.returncode == 0:
                print(f"   {Colors.GREEN}✅ {description} - PASSED{Colors.END}")
                self.passed_checks += 1
                return True, result.stdout
            else:
                error_msg = f"{description} - FAILED"
                print(f"   {Colors.RED}❌ {error_msg}{Colors.END}")
                if result.stderr:
                    print(f"   {Colors.RED}Error: {result.stderr.strip()}{Colors.END}")

                if critical:
                    self.failed_checks.append(error_msg)
                else:
                    self.warnings.append(error_msg)

                return False, result.stderr

        except subprocess.TimeoutExpired:
            error_msg = f"{description} - TIMEOUT"
            print(f"   {Colors.RED}⏰ {error_msg}{Colors.END}")
            if critical:
                self.failed_checks.append(error_msg)
            else:
                self.warnings.append(error_msg)
            return False, "Command timed out"

        except Exception as e:
            error_msg = f"{description} - ERROR: {str(e)}"
            print(f"   {Colors.RED}💥 {error_msg}{Colors.END}")
            if critical:
                self.failed_checks.append(error_msg)
            else:
                self.warnings.append(error_msg)
            return False, str(e)

    def check_prerequisites(self) -> bool:
        """Check if required tools are available"""
        self.print_header("PREREQUISITE CHECKS")

        tools = [
            (["cargo", "--version"], "Cargo availability"),
            (["rustc", "--version"], "Rust compiler availability"),
            (["git", "--version"], "Git availability"),
        ]

        all_good = True
        for cmd, desc in tools:
            success, _ = self.run_command(cmd, desc, critical=True)
            if not success:
                all_good = False

        return all_good

    def core_compilation_tests(self) -> bool:
        """Run core compilation and testing checks"""
        self.print_header("CORE COMPILATION & TESTING")

        checks = [
            (["cargo", "check"], "Basic compilation check"),
            (["cargo", "build"], "Full build"),
            (["cargo", "build", "--release"], "Release build"),
            (["cargo", "test"], "All tests"),
            (["cargo", "test", "--all-targets"], "All targets tests"),
        ]

        all_passed = True
        for cmd, desc in checks:
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=True)
            if not success:
                all_passed = False

        return all_passed

    def code_quality_checks(self) -> bool:
        """Run code quality and linting checks"""
        self.print_header("CODE QUALITY CHECKS")

        checks = [
            (["cargo", "fmt", "--all", "--check"], "Code formatting"),
            (["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"], "Clippy linting"),
            (["cargo", "check", "--all-targets", "--all-features"], "All features check"),
        ]

        all_passed = True
        for cmd, desc in checks:
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=True)
            if not success:
                all_passed = False

        return all_passed

    def documentation_checks(self) -> bool:
        """Run documentation generation checks"""
        self.print_header("DOCUMENTATION CHECKS")

        checks = [
            (["cargo", "doc", "--no-deps"], "Basic documentation"),
            (["cargo", "doc", "--all-features", "--no-deps"], "Documentation with all features"),
        ]

        all_passed = True
        for cmd, desc in checks:
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=False)  # Non-critical
            if not success:
                all_passed = False

        return all_passed

    def package_validation(self) -> bool:
        """Run package validation for crates.io"""
        self.print_header("PACKAGE VALIDATION")

        checks = [
            (["cargo", "package", "--list"], "Package file list"),
            (["cargo", "package", "--allow-dirty"], "Package creation"),
        ]

        all_passed = True
        for cmd, desc in checks:
            self.print_step(desc)
            success, _ = self.run_command(cmd, desc, critical=False)  # Non-critical for now
            if not success:
                all_passed = False

        return all_passed

    def metadata_checks(self) -> bool:
        """Check required metadata files"""
        self.print_header("METADATA & FILES CHECK")

        required_files = [
            ("Cargo.toml", "Cargo manifest"),
            ("README.md", "README file"),
            ("LICENSE", "License file"),
        ]

        all_passed = True
        for filename, desc in required_files:
            self.print_step(f"Checking {desc}")
            if Path(filename).exists():
                print(f"   {Colors.GREEN}✅ {desc} - EXISTS{Colors.END}")
                self.passed_checks += 1
            else:
                print(f"   {Colors.YELLOW}⚠️  {desc} - MISSING{Colors.END}")
                self.warnings.append(f"{desc} missing")
                if filename in ["Cargo.toml"]:  # Critical files
                    all_passed = False
            self.total_checks += 1

        return all_passed

    def print_summary(self):
        """Print final summary of all checks"""
        self.print_header("FINAL SUMMARY")

        print(f"\n{Colors.BOLD}📊 RESULTS SUMMARY:{Colors.END}")
        print(f"   Total Checks: {self.total_checks}")
        print(f"   {Colors.GREEN}Passed: {self.passed_checks}{Colors.END}")
        print(f"   {Colors.RED}Failed: {len(self.failed_checks)}{Colors.END}")
        print(f"   {Colors.YELLOW}Warnings: {len(self.warnings)}{Colors.END}")

        if self.failed_checks:
            print(f"\n{Colors.RED}{Colors.BOLD}❌ CRITICAL FAILURES:{Colors.END}")
            for failure in self.failed_checks:
                print(f"   {Colors.RED}• {failure}{Colors.END}")

        if self.warnings:
            print(f"\n{Colors.YELLOW}{Colors.BOLD}⚠️  WARNINGS:{Colors.END}")
            for warning in self.warnings:
                print(f"   {Colors.YELLOW}• {warning}{Colors.END}")

        # Final verdict - MUST have zero failures AND zero warnings for release
        if not self.failed_checks and not self.warnings:
            print(f"\n{Colors.GREEN}{Colors.BOLD}🎉 CRATE IS READY FOR CRATES.IO RELEASE! 🎉{Colors.END}")
            print(f"{Colors.GREEN}All checks passed with no failures or warnings. You can proceed with publishing.{Colors.END}")

            # Auto-commit when all checks pass and trigger CI
            self.auto_commit_and_trigger_ci()
            return True
        elif not self.failed_checks and self.warnings:
            print(f"\n{Colors.YELLOW}{Colors.BOLD}⚠️  CRATE NOT READY - WARNINGS MUST BE FIXED{Colors.END}")
            print(f"{Colors.YELLOW}All critical checks passed, but warnings must be addressed before release.{Colors.END}")
            print(f"{Colors.YELLOW}Please fix the warnings above to proceed with publishing.{Colors.END}")
            return False
        else:
            print(f"\n{Colors.RED}{Colors.BOLD}🚫 CRATE NOT READY FOR RELEASE{Colors.END}")
            print(f"{Colors.RED}Please fix the critical failures and warnings before publishing.{Colors.END}")
            return False

    def auto_commit_and_trigger_ci(self):
        """Auto-commit when all checks pass and trigger CI"""
        print(f"\n{Colors.CYAN}{Colors.BOLD}🔄 AUTO-COMMITTING SUCCESS & TRIGGERING CI...{Colors.END}")

        try:
            # Check if there are any changes to commit
            result = subprocess.run(
                ["git", "status", "--porcelain"],
                capture_output=True,
                text=True,
                timeout=30
            )

            if result.returncode == 0 and result.stdout.strip():
                # There are changes to commit
                print(f"   {Colors.CYAN}📝 Adding all changes...{Colors.END}")

                # Add all changes
                add_result = subprocess.run(
                    ["git", "add", "."],
                    capture_output=True,
                    text=True,
                    timeout=30
                )

                if add_result.returncode == 0:
                    # Create comprehensive commit message
                    commit_message = f"""🎉 CRATE READY FOR CRATES.IO RELEASE - ALL CHECKS PASS

✅ **COMPREHENSIVE QUALITY VALIDATION COMPLETE:**

📊 **Results Summary:**
- Total Checks: {self.total_checks}
- Passed: {self.passed_checks}
- Failed: 0
- Warnings: 0

🔍 **All Validations Passed:**
- ✅ Prerequisites (Cargo, Rust, Git)
- ✅ Core Compilation & Testing
- ✅ Code Quality (fmt, clippy)
- ✅ Documentation Generation
- ✅ Package Validation
- ✅ Metadata & Files Check

🚀 **Production Ready:**
- Zero failures, zero warnings
- Meets all crates.io quality standards
- Professional-grade error handling framework
- Comprehensive procedural macro support
- Ready for immediate publication

🎯 **Validated by cratecheck.py v1.0**
Automated quality assurance for Rust crates"""

                    print(f"   {Colors.CYAN}💾 Committing with success message...{Colors.END}")

                    # Commit with the comprehensive message
                    commit_result = subprocess.run(
                        ["git", "commit", "-m", commit_message],
                        capture_output=True,
                        text=True,
                        timeout=60
                    )

                    if commit_result.returncode == 0:
                        print(f"   {Colors.GREEN}✅ Successfully committed all changes!{Colors.END}")

                        # Now trigger CI by pushing
                        print(f"   {Colors.CYAN}🚀 Pushing to trigger CI...{Colors.END}")
                        push_result = subprocess.run(
                            ["git", "push"],
                            capture_output=True,
                            text=True,
                            timeout=120
                        )

                        if push_result.returncode == 0:
                            print(f"   {Colors.GREEN}✅ Successfully pushed to remote - CI triggered!{Colors.END}")
                            print(f"   {Colors.GREEN}🔄 Check your repository for CI pipeline status{Colors.END}")
                        else:
                            print(f"   {Colors.YELLOW}⚠️  Push failed: {push_result.stderr.strip()}{Colors.END}")
                            print(f"   {Colors.YELLOW}💡 You may need to push manually to trigger CI{Colors.END}")
                    else:
                        print(f"   {Colors.YELLOW}⚠️  Commit failed: {commit_result.stderr.strip()}{Colors.END}")
                else:
                    print(f"   {Colors.YELLOW}⚠️  Git add failed: {add_result.stderr.strip()}{Colors.END}")
            else:
                print(f"   {Colors.CYAN}ℹ️  No changes to commit - repository is clean{Colors.END}")

                # Check if we're ahead of remote (unpushed commits)
                ahead_result = subprocess.run(
                    ["git", "rev-list", "--count", "HEAD", "^origin/HEAD"],
                    capture_output=True,
                    text=True,
                    timeout=30
                )

                if ahead_result.returncode == 0 and ahead_result.stdout.strip() != "0":
                    # We have unpushed commits, push them to trigger CI
                    unpushed_count = ahead_result.stdout.strip()
                    print(f"   {Colors.CYAN}🚀 Found {unpushed_count} unpushed commit(s) - pushing to trigger CI...{Colors.END}")

                    push_result = subprocess.run(
                        ["git", "push"],
                        capture_output=True,
                        text=True,
                        timeout=120
                    )

                    if push_result.returncode == 0:
                        print(f"   {Colors.GREEN}✅ Successfully pushed {unpushed_count} commit(s) - CI triggered!{Colors.END}")
                        print(f"   {Colors.GREEN}🔄 Check your repository for CI pipeline status{Colors.END}")
                    else:
                        print(f"   {Colors.YELLOW}⚠️  Push failed: {push_result.stderr.strip()}{Colors.END}")
                else:
                    # Repository is clean and up-to-date, no need for CI trigger
                    print(f"   {Colors.GREEN}✅ Repository is clean and up-to-date{Colors.END}")
                    print(f"   {Colors.GREEN}🎯 All quality checks passed - ready for crates.io release{Colors.END}")
                    print(f"   {Colors.CYAN}💡 No CI trigger needed - repository is already synchronized{Colors.END}")

        except subprocess.TimeoutExpired:
            print(f"   {Colors.YELLOW}⚠️  Git operation timed out{Colors.END}")
        except Exception as e:
            print(f"   {Colors.YELLOW}⚠️  Auto-commit/CI trigger failed: {str(e)}{Colors.END}")

    def run_all_checks(self) -> bool:
        """Run all checks in sequence"""
        print(f"{Colors.MAGENTA}{Colors.BOLD}")
        print("🦀 RUST CRATE QUALITY CHECKER 🦀")
        print("Comprehensive validation for crates.io release")
        print(f"{'='*50}{Colors.END}")

        start_time = time.time()

        # Run all check categories
        checks_passed = True

        if not self.check_prerequisites():
            print(f"\n{Colors.RED}❌ Prerequisites failed. Cannot continue.{Colors.END}")
            return False

        checks_passed &= self.core_compilation_tests()
        checks_passed &= self.code_quality_checks()
        checks_passed &= self.documentation_checks()
        checks_passed &= self.package_validation()
        checks_passed &= self.metadata_checks()

        # Print timing
        elapsed = time.time() - start_time
        print(f"\n{Colors.CYAN}⏱️  Total execution time: {elapsed:.2f} seconds{Colors.END}")

        # Print summary
        return self.print_summary()

def main():
    """Main entry point"""
    if len(sys.argv) > 1 and sys.argv[1] in ["-h", "--help"]:
        print("CrateCheck - Rust Crate Quality Validator")
        print("Usage: python cratecheck.py")
        print("\nRuns comprehensive checks for crates.io release readiness:")
        print("• Compilation and testing")
        print("• Code quality (fmt, clippy)")
        print("• Documentation generation")
        print("• Package validation")
        print("• Metadata verification")
        return

    checker = CrateChecker()
    success = checker.run_all_checks()

    # Exit with appropriate code
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
