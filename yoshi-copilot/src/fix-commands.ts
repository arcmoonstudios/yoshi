/**
 * Yoshi Fix Commands - Automated fix execution for VS Code integration
 *
 * This module provides command handlers that execute automated fixes
 * triggered from VS Code's Quick Fix interface.
 */

import * as vscode from 'vscode';
import { spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

export interface FixResult {
    success: boolean;
    message: string;
    changes?: vscode.TextEdit[];
    error?: string | undefined;
}

/**
 * Command handlers for automated fixes
 */
export class YoshiFixCommands {
    private outputChannel: vscode.OutputChannel;
    private workspaceRoot: string;

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        this.workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || '';
    }

    /**
     * Register all fix commands
     */
    registerCommands(context: vscode.ExtensionContext): void {
        const commands = [
            vscode.commands.registerCommand('yoshi.runAutoFix', this.runAutoFix.bind(this)),
            vscode.commands.registerCommand('yoshi.runDeadCodeAnalysis', this.runDeadCodeAnalysis.bind(this)),
            vscode.commands.registerCommand('yoshi.runClippyFix', this.runClippyFix.bind(this)),
            vscode.commands.registerCommand('yoshi.analyzeFile', this.analyzeFile.bind(this)),
            vscode.commands.registerCommand('yoshi.fixAllInFile', this.fixAllInFile.bind(this)),
            vscode.commands.registerCommand('yoshi.runYoFixWhat', this.runYoFixWhat.bind(this)),
            vscode.commands.registerCommand('yoshi.applySystematicFixes', this.applySystematicFixes.bind(this))
        ];

        context.subscriptions.push(...commands);
    }

    /**
     * Run automated fix for specific issue type
     */
    async runAutoFix(
        uri: vscode.Uri,
        range: vscode.Range,
        fixType: string
    ): Promise<void> {
        try {
            this.outputChannel.appendLine(`🔧 Running auto-fix: ${fixType}`);

            const document = await vscode.workspace.openTextDocument(uri);
            const result = await this.executeAutoFix(document, range, fixType);

            if (result.success) {
                if (result.changes && result.changes.length > 0) {
                    const edit = new vscode.WorkspaceEdit();
                    edit.set(uri, result.changes);
                    await vscode.workspace.applyEdit(edit);
                }

                vscode.window.showInformationMessage(`✅ ${result.message}`);
                this.outputChannel.appendLine(`✅ Fix applied: ${result.message}`);
            } else {
                vscode.window.showErrorMessage(`❌ Fix failed: ${result.error}`);
                this.outputChannel.appendLine(`❌ Fix failed: ${result.error}`);
            }
        } catch (error) {
            const errorMsg = `Failed to run auto-fix: ${error}`;
            vscode.window.showErrorMessage(errorMsg);
            this.outputChannel.appendLine(errorMsg);
        }
    }

    /**
     * Run dead code analysis with dependency checking
     */
    async runDeadCodeAnalysis(_uri: vscode.Uri, _range: vscode.Range): Promise<void> {
        try {
            this.outputChannel.appendLine('🔍 Running dead code analysis...');

            // Run our systematic dead code analysis
            const result = await this.executeCommand('python', ['yoFixWhat.py', '--deets']);

            if (result.success) {
                vscode.window.showInformationMessage('✅ Dead code analysis complete. Check yoFixME.txt for results.');

                // Open the results file
                const resultsPath = path.join(this.workspaceRoot, 'yoFixME.txt');
                if (fs.existsSync(resultsPath)) {
                    const resultsUri = vscode.Uri.file(resultsPath);
                    await vscode.window.showTextDocument(resultsUri);
                }
            } else {
                vscode.window.showErrorMessage(`❌ Analysis failed: ${result.error}`);
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to run dead code analysis: ${error}`);
        }
    }

    /**
     * Run clippy fix for specific issue
     */
    async runClippyFix(
        uri: vscode.Uri,
        _range: vscode.Range,
        code?: string
    ): Promise<void> {
        try {
            this.outputChannel.appendLine(`🔧 Running clippy fix for: ${code || 'unknown'}`);

            // Get the crate directory
            const crateDir = await this.findCrateRoot(uri.fsPath);

            // Run clippy --fix for the specific crate
            const result = await this.executeCommand('cargo', ['clippy', '--fix', '--allow-dirty'], {
                cwd: crateDir
            });

            if (result.success) {
                vscode.window.showInformationMessage('✅ Clippy fixes applied successfully');

                // Reload the document to show changes
                const document = await vscode.workspace.openTextDocument(uri);
                await vscode.window.showTextDocument(document);
            } else {
                vscode.window.showWarningMessage(`⚠️ Clippy fix completed with warnings. Check output for details.`);
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to run clippy fix: ${error}`);
        }
    }

    /**
     * Analyze file with Yoshi diagnostics
     */
    async analyzeFile(uri: vscode.Uri): Promise<void> {
        try {
            this.outputChannel.appendLine(`🔍 Analyzing file: ${uri.fsPath}`);

            const crateDir = await this.findCrateRoot(uri.fsPath);

            // Run cargo check and clippy for the specific crate
            const checkResult = await this.executeCommand('cargo', ['check'], { cwd: crateDir });
            const clippyResult = await this.executeCommand('cargo', ['clippy'], { cwd: crateDir });

            this.outputChannel.appendLine('📊 Analysis Results:');
            this.outputChannel.appendLine(`Check: ${checkResult.success ? '✅ Passed' : '❌ Failed'}`);
            this.outputChannel.appendLine(`Clippy: ${clippyResult.success ? '✅ Passed' : '❌ Failed'}`);

            if (checkResult.output) {
                this.outputChannel.appendLine('\n🔍 Cargo Check Output:');
                this.outputChannel.appendLine(checkResult.output);
            }

            if (clippyResult.output) {
                this.outputChannel.appendLine('\n📎 Clippy Output:');
                this.outputChannel.appendLine(clippyResult.output);
            }

            vscode.window.showInformationMessage('✅ File analysis complete. Check Yoshi output for details.');
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to analyze file: ${error}`);
        }
    }

    /**
     * Fix all issues in the current file
     */
    async fixAllInFile(uri: vscode.Uri): Promise<void> {
        try {
            this.outputChannel.appendLine(`🚀 Fixing all issues in: ${uri.fsPath}`);

            const crateDir = await this.findCrateRoot(uri.fsPath);

            // Run our systematic fix approach
            await this.runYoFixWhat();

            // Then run clippy --fix
            const clippyResult = await this.executeCommand('cargo', ['clippy', '--fix', '--allow-dirty'], {
                cwd: crateDir
            });

            if (clippyResult.success) {
                vscode.window.showInformationMessage('✅ All fixes applied successfully');

                // Reload the document
                const document = await vscode.workspace.openTextDocument(uri);
                await vscode.window.showTextDocument(document);
            } else {
                vscode.window.showWarningMessage('⚠️ Some fixes may not have been applied. Check output for details.');
            }
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to fix all issues: ${error}`);
        }
    }

    /**
     * Run yoFixWhat.py analysis
     */
    async runYoFixWhat(): Promise<void> {
        try {
            this.outputChannel.appendLine('🔍 Running yoFixWhat.py analysis...');

            const result = await this.executeCommand('python', ['yoFixWhat.py', '--deets'], {
                cwd: this.workspaceRoot
            });

            if (result.success) {
                this.outputChannel.appendLine('✅ yoFixWhat.py analysis complete');

                // Show the results
                const resultsPath = path.join(this.workspaceRoot, 'yoFixME.txt');
                if (fs.existsSync(resultsPath)) {
                    const resultsUri = vscode.Uri.file(resultsPath);
                    await vscode.window.showTextDocument(resultsUri, { preview: false });
                }
            } else {
                this.outputChannel.appendLine(`❌ yoFixWhat.py failed: ${result.error}`);
            }
        } catch (error) {
            this.outputChannel.appendLine(`Failed to run yoFixWhat.py: ${error}`);
        }
    }

    /**
     * Apply systematic fixes based on our zero tolerance protocol
     */
    async applySystematicFixes(): Promise<void> {
        try {
            this.outputChannel.appendLine('🚀 Starting systematic fix protocol...');

            // Step 1: Run yoFixWhat.py for analysis
            await this.runYoFixWhat();

            // Step 2: Apply clippy fixes where safe
            const clippyResult = await this.executeCommand('cargo', ['clippy', '--fix', '--allow-dirty'], {
                cwd: this.workspaceRoot
            });

            // Step 3: Run cargo fmt
            const fmtResult = await this.executeCommand('cargo', ['fmt'], {
                cwd: this.workspaceRoot
            });

            // Step 4: Re-run analysis to check progress
            const finalResult = await this.executeCommand('python', ['yoFixWhat.py', '--norm'], {
                cwd: this.workspaceRoot
            });

            this.outputChannel.appendLine('📊 Systematic Fix Results:');
            this.outputChannel.appendLine(`Clippy fixes: ${clippyResult.success ? '✅' : '❌'}`);
            this.outputChannel.appendLine(`Formatting: ${fmtResult.success ? '✅' : '❌'}`);
            this.outputChannel.appendLine(`Final analysis: ${finalResult.success ? '✅' : '❌'}`);

            vscode.window.showInformationMessage('✅ Systematic fixes complete. Check yoFixME.txt for remaining issues.');
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to apply systematic fixes: ${error}`);
        }
    }

    /**
     * Execute a specific auto-fix
     */
    private async executeAutoFix(
        document: vscode.TextDocument,
        range: vscode.Range,
        fixType: string
    ): Promise<FixResult> {
        switch (fixType) {
            case 'unnecessary_wraps':
                return this.fixUnnecessaryWraps(document, range);
            case 'unused_variable':
                return this.fixUnusedVariable(document, range);
            default:
                return {
                    success: false,
                    message: 'Unknown fix type',
                    error: `Unsupported fix type: ${fixType}`
                };
        }
    }

    /**
     * Fix unnecessary Result wraps
     */
    private async fixUnnecessaryWraps(
        document: vscode.TextDocument,
        range: vscode.Range
    ): Promise<FixResult> {
        // This would require complex AST analysis
        // For now, delegate to clippy --fix
        const crateDir = await this.findCrateRoot(document.uri.fsPath);
        const result = await this.executeCommand('cargo', ['clippy', '--fix', '--allow-dirty'], {
            cwd: crateDir
        });

        return {
            success: result.success,
            message: result.success ? 'Unnecessary wraps fixed' : 'Fix failed',
            error: result.success ? undefined : result.error
        };
    }

    /**
     * Fix unused variable by adding underscore prefix
     */
    private async fixUnusedVariable(
        document: vscode.TextDocument,
        range: vscode.Range
    ): Promise<FixResult> {
        const line = document.lineAt(range.start.line);
        const text = line.text;

        const variableMatch = text.match(/let\s+(\w+)/);
        if (variableMatch && variableMatch[1] && !variableMatch[1].startsWith('_')) {
            const variableName = variableMatch[1];
            const startPos = text.indexOf(variableName);

            const edit = vscode.TextEdit.replace(
                new vscode.Range(
                    range.start.line,
                    startPos,
                    range.start.line,
                    startPos + variableName.length
                ),
                `_${variableName}`
            );

            return {
                success: true,
                message: `Added underscore prefix to ${variableName}`,
                changes: [edit]
            };
        }

        return {
            success: false,
            message: 'Could not identify variable to fix',
            error: 'Variable pattern not found'
        };
    }

    /**
     * Execute a command and return the result
     */
    private async executeCommand(
        command: string,
        args: string[],
        options?: { cwd?: string }
    ): Promise<{ success: boolean; output?: string | undefined; error?: string | undefined }> {
        return new Promise((resolve) => {
            const proc = spawn(command, args, {
                cwd: options?.cwd || this.workspaceRoot,
                shell: true
            });

            let output = '';
            let error = '';

            proc.stdout?.on('data', (data) => {
                output += data.toString();
            });

            proc.stderr?.on('data', (data) => {
                error += data.toString();
            });

            proc.on('close', (code) => {
                resolve({
                    success: code === 0,
                    output: output.trim(),
                    error: error.trim() || undefined
                });
            });

            proc.on('error', (err) => {
                resolve({
                    success: false,
                    error: err.message
                });
            });
        });
    }

    /**
     * Find the root directory of the crate containing the given file
     */
    private async findCrateRoot(filePath: string): Promise<string> {
        let dir = path.dirname(filePath);

        while (dir !== path.dirname(dir)) {
            const cargoTomlPath = path.join(dir, 'Cargo.toml');
            if (fs.existsSync(cargoTomlPath)) {
                return dir;
            }
            dir = path.dirname(dir);
        }

        return this.workspaceRoot;
    }
}
