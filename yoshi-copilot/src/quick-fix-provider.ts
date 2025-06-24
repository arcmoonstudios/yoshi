/**
 * Yoshi Quick Fix Provider - Integrates automated fixes into VS Code's Quick Fix system
 *
 * This provider analyzes clippy warnings and errors, then provides automated fixes
 * directly through VS Code's lightbulb (💡) Quick Fix interface.
 */

import * as vscode from 'vscode';

export interface YoshiQuickFix {
    title: string;
    description: string;
    kind: vscode.CodeActionKind;
    edit?: vscode.WorkspaceEdit | undefined;
    command?: vscode.Command | undefined;
    diagnostics?: vscode.Diagnostic[];
    confidence: number;
    category: 'correctness' | 'performance' | 'style' | 'docs';
}

export interface ClippyDiagnostic {
    message: string;
    code: string;
    severity: 'error' | 'warning';
    file: string;
    line: number;
    column: number;
    endLine?: number;
    endColumn?: number;
    suggestion?: string;
}

/**
 * Code Action Provider that integrates Yoshi automated fixes with VS Code
 */
export class YoshiQuickFixProvider implements vscode.CodeActionProvider {
    private outputChannel: vscode.OutputChannel;

    constructor() {
        this.outputChannel = vscode.window.createOutputChannel('Yoshi Quick Fix');
    }

    /**
     * Provide code actions for the given document and range
     */
    async provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        _token: vscode.CancellationToken
    ): Promise<vscode.CodeAction[]> {
        const actions: vscode.CodeAction[] = [];

        try {
            // Get diagnostics for the current range
            const relevantDiagnostics = context.diagnostics.filter(diagnostic =>
                diagnostic.source === 'rust-analyzer' || diagnostic.source === 'clippy'
            );

            if (relevantDiagnostics.length === 0) {
                return actions;
            }

            // Generate quick fixes for each diagnostic
            for (const diagnostic of relevantDiagnostics) {
                const quickFixes = await this.generateQuickFixes(document, diagnostic, range);

                for (const fix of quickFixes) {
                    const action = new vscode.CodeAction(fix.title, fix.kind);
                    // Note: detail property doesn't exist on CodeAction, using title for description
                    if (fix.edit) {
                        action.edit = fix.edit;
                    }
                    if (fix.command) {
                        action.command = fix.command;
                    }
                    action.diagnostics = [diagnostic];

                    // Add confidence indicator to the title
                    if (fix.confidence > 0.8) {
                        action.title = `🎯 ${fix.title}`;
                    } else if (fix.confidence > 0.6) {
                        action.title = `⚡ ${fix.title}`;
                    } else {
                        action.title = `💡 ${fix.title}`;
                    }

                    actions.push(action);
                }
            }

            // Add general Yoshi actions
            actions.push(...this.createGeneralActions(document, range));

        } catch (error) {
            this.outputChannel.appendLine(`Error providing code actions: ${error}`);
        }

        return actions;
    }

    /**
     * Generate quick fixes for a specific diagnostic
     */
    private async generateQuickFixes(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic,
        _range: vscode.Range
    ): Promise<YoshiQuickFix[]> {
        const fixes: YoshiQuickFix[] = [];
        const diagnosticMessage = diagnostic.message.toLowerCase();

        // Missing documentation fixes
        if (diagnosticMessage.includes('missing documentation')) {
            fixes.push(await this.createDocumentationFix(document, diagnostic));
        }

        // Missing errors doc fixes
        if (diagnosticMessage.includes('missing `# errors` section')) {
            fixes.push(await this.createErrorsDocFix(document, diagnostic));
        }

        // Unnecessary wraps fixes
        if (diagnosticMessage.includes('unnecessarily wrapped by `result`')) {
            fixes.push(await this.createUnwrapResultFix(document, diagnostic));
        }

        // Unused variable fixes
        if (diagnosticMessage.includes('unused variable') || diagnosticMessage.includes('never read')) {
            fixes.push(await this.createUnusedVariableFix(document, diagnostic));
        }

        // Dead code fixes
        if (diagnosticMessage.includes('never used') || diagnosticMessage.includes('dead code')) {
            fixes.push(await this.createDeadCodeFix(document, diagnostic));
        }

        // Clippy style fixes
        if (diagnosticMessage.includes('clippy::')) {
            fixes.push(await this.createClippyFix(document, diagnostic));
        }

        return fixes;
    }

    /**
     * Create documentation fix
     */
    private async createDocumentationFix(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): Promise<YoshiQuickFix> {
        const line = document.lineAt(diagnostic.range.start.line);
        const text = line.text;
        const indent = text.match(/^\s*/)?.[0] || '';

        let docComment = '';

        // Determine what kind of documentation to add
        if (text.includes('struct')) {
            const structName = text.match(/struct\s+(\w+)/)?.[1] || 'Unknown';
            docComment = `${indent}/// **${structName}**\n${indent}///\n${indent}/// TODO: Add description for ${structName}\n`;
        } else if (text.includes('enum')) {
            const enumName = text.match(/enum\s+(\w+)/)?.[1] || 'Unknown';
            docComment = `${indent}/// **${enumName}**\n${indent}///\n${indent}/// TODO: Add description for ${enumName}\n`;
        } else if (text.includes('fn ')) {
            const fnName = text.match(/fn\s+(\w+)/)?.[1] || 'unknown';
            docComment = `${indent}/// **${fnName}**\n${indent}///\n${indent}/// TODO: Add description for ${fnName}\n`;
        } else if (text.includes(':')) {
            // Field documentation
            const fieldName = text.match(/(\w+)\s*:/)?.[1] || 'field';
            docComment = `${indent}/// TODO: Document ${fieldName}\n`;
        } else {
            docComment = `${indent}/// TODO: Add documentation\n`;
        }

        const edit = new vscode.WorkspaceEdit();
        edit.insert(document.uri, diagnostic.range.start, docComment);

        return {
            title: 'Add documentation',
            description: 'Add missing documentation comment',
            kind: vscode.CodeActionKind.QuickFix,
            edit,
            confidence: 0.9,
            category: 'docs'
        };
    }

    /**
     * Create errors documentation fix
     */
    private async createErrorsDocFix(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): Promise<YoshiQuickFix> {
        const line = document.lineAt(diagnostic.range.start.line);
        const text = line.text;
        const indent = text.match(/^\s*/)?.[0] || '';

        // Find the end of the existing doc comment
        let insertLine = diagnostic.range.start.line;
        for (let i = diagnostic.range.start.line - 1; i >= 0; i--) {
            const prevLine = document.lineAt(i);
            if (prevLine.text.trim().startsWith('///')) {
                insertLine = i + 1;
                break;
            }
            if (prevLine.text.trim() === '') {
                continue;
            }
            break;
        }

        const errorsDoc = `${indent}///\n${indent}/// # Errors\n${indent}///\n${indent}/// Returns an error if the operation fails.\n`;

        const edit = new vscode.WorkspaceEdit();
        edit.insert(document.uri, new vscode.Position(insertLine, 0), errorsDoc);

        return {
            title: 'Add # Errors section',
            description: 'Add missing # Errors section to documentation',
            kind: vscode.CodeActionKind.QuickFix,
            edit,
            confidence: 0.95,
            category: 'docs'
        };
    }

    /**
     * Create unnecessary Result unwrap fix
     */
    private async createUnwrapResultFix(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): Promise<YoshiQuickFix> {
        // This would require more complex analysis to safely unwrap Results
        // For now, provide a command to run automated fix
        return {
            title: 'Remove unnecessary Result wrapper',
            description: 'Automatically remove unnecessary Result wrapper',
            kind: vscode.CodeActionKind.QuickFix,
            command: {
                title: 'Run Yoshi Auto-fix',
                command: 'yoshi.runAutoFix',
                arguments: [document.uri, diagnostic.range, 'unnecessary_wraps']
            },
            confidence: 0.8,
            category: 'correctness'
        };
    }

    /**
     * Create unused variable fix
     */
    private async createUnusedVariableFix(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): Promise<YoshiQuickFix> {
        const line = document.lineAt(diagnostic.range.start.line);
        const text = line.text;

        // Add underscore prefix to unused variable
        const variableName = text.match(/let\s+(\w+)/)?.[1];
        if (variableName && !variableName.startsWith('_')) {
            const edit = new vscode.WorkspaceEdit();
            const range = new vscode.Range(
                diagnostic.range.start.line,
                text.indexOf(variableName),
                diagnostic.range.start.line,
                text.indexOf(variableName) + variableName.length
            );
            edit.replace(document.uri, range, `_${variableName}`);

            return {
                title: `Prefix with underscore: _${variableName}`,
                description: 'Add underscore prefix to indicate intentionally unused variable',
                kind: vscode.CodeActionKind.QuickFix,
                edit,
                confidence: 0.9,
                category: 'style'
            };
        }

        return {
            title: 'Fix unused variable',
            description: 'Apply automatic fix for unused variable',
            kind: vscode.CodeActionKind.QuickFix,
            command: {
                title: 'Run Yoshi Auto-fix',
                command: 'yoshi.runAutoFix',
                arguments: [document.uri, diagnostic.range, 'unused_variable']
            },
            confidence: 0.7,
            category: 'style'
        };
    }

    /**
     * Create dead code fix
     */
    private async createDeadCodeFix(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): Promise<YoshiQuickFix> {
        return {
            title: 'Remove dead code',
            description: 'Safely remove unused code after dependency analysis',
            kind: vscode.CodeActionKind.QuickFix,
            command: {
                title: 'Run Yoshi Dead Code Analysis',
                command: 'yoshi.runDeadCodeAnalysis',
                arguments: [document.uri, diagnostic.range]
            },
            confidence: 0.6,
            category: 'correctness'
        };
    }

    /**
     * Create general clippy fix
     */
    private async createClippyFix(
        document: vscode.TextDocument,
        diagnostic: vscode.Diagnostic
    ): Promise<YoshiQuickFix> {
        return {
            title: 'Apply clippy suggestion',
            description: 'Apply automatic clippy fix',
            kind: vscode.CodeActionKind.QuickFix,
            command: {
                title: 'Run Clippy Fix',
                command: 'yoshi.runClippyFix',
                arguments: [document.uri, diagnostic.range, diagnostic.code]
            },
            confidence: 0.8,
            category: 'style'
        };
    }

    /**
     * Create general Yoshi actions
     */
    private createGeneralActions(
        document: vscode.TextDocument,
        _range: vscode.Range
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];

        // Run full Yoshi analysis
        const analyzeAction = new vscode.CodeAction(
            '🔍 Run Yoshi Analysis',
            vscode.CodeActionKind.Source
        );
        analyzeAction.command = {
            title: 'Analyze with Yoshi',
            command: 'yoshi.analyzeFile',
            arguments: [document.uri]
        };
        actions.push(analyzeAction);

        // Fix all issues in file
        const fixAllAction = new vscode.CodeAction(
            '🚀 Fix All Yoshi Issues',
            vscode.CodeActionKind.SourceFixAll
        );
        fixAllAction.command = {
            title: 'Fix All Issues',
            command: 'yoshi.fixAllInFile',
            arguments: [document.uri]
        };
        actions.push(fixAllAction);

        return actions;
    }
}
