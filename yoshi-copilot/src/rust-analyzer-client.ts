import * as vscode from 'vscode';
import { DiagnosticInfo } from './types';

/**
 * Rust Analyzer client for enhanced diagnostics integration
 */
export class RustAnalyzer {
    private diagnosticCollection: vscode.DiagnosticCollection;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('yoshi-copilot');
    }

    /**
     * Get diagnostics from rust-analyzer for a specific document
     */
    async getDiagnostics(uri: vscode.Uri): Promise<DiagnosticInfo[]> {
        const diagnostics = vscode.languages.getDiagnostics(uri);

        return diagnostics.map(diagnostic => ({
            message: diagnostic.message,
            severity: this.mapSeverity(diagnostic.severity),
            range: diagnostic.range,
            source: diagnostic.source || 'rust-analyzer'
        }));
    }

    /**
     * Get all workspace diagnostics
     */
    async getWorkspaceDiagnostics(): Promise<Map<string, DiagnosticInfo[]>> {
        const diagnosticsMap = new Map<string, DiagnosticInfo[]>();

        // Get all diagnostics from the workspace
        vscode.languages.getDiagnostics().forEach(([uri, diagnostics]) => {
            if (uri.path.endsWith('.rs')) {
                const diagnosticInfos = diagnostics.map(diagnostic => ({
                    message: diagnostic.message,
                    severity: this.mapSeverity(diagnostic.severity),
                    range: diagnostic.range,
                    source: diagnostic.source || 'rust-analyzer'
                }));

                diagnosticsMap.set(uri.toString(), diagnosticInfos);
            }
        });

        return diagnosticsMap;
    }

    /**
     * Check if rust-analyzer is available and running
     */
    async isRustAnalyzerAvailable(): Promise<boolean> {
        const rustAnalyzerExtension = vscode.extensions.getExtension('rust-lang.rust-analyzer');
        return rustAnalyzerExtension !== undefined && rustAnalyzerExtension.isActive;
    }

    /**
     * Get code actions from rust-analyzer for a specific range
     */
    async getCodeActions(document: vscode.TextDocument, range: vscode.Range): Promise<vscode.CodeAction[]> {
        try {
            const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>(
                'vscode.executeCodeActionProvider',
                document.uri,
                range
            );

            return codeActions || [];
        } catch (error) {
            console.warn('Failed to get code actions from rust-analyzer:', error);
            return [];
        }
    }

    /**
     * Get hover information from rust-analyzer
     */
    async getHoverInfo(document: vscode.TextDocument, position: vscode.Position): Promise<vscode.Hover[]> {
        try {
            const hovers = await vscode.commands.executeCommand<vscode.Hover[]>(
                'vscode.executeHoverProvider',
                document.uri,
                position
            );

            return hovers || [];
        } catch (error) {
            console.warn('Failed to get hover info from rust-analyzer:', error);
            return [];
        }
    }

    /**
     * Get completion items from rust-analyzer
     */
    async getCompletions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        try {
            const completions = await vscode.commands.executeCommand<vscode.CompletionList>(
                'vscode.executeCompletionItemProvider',
                document.uri,
                position
            );

            return completions?.items || [];
        } catch (error) {
            console.warn('Failed to get completions from rust-analyzer:', error);
            return [];
        }
    }

    /**
     * Get definition locations from rust-analyzer
     */
    async getDefinitions(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.Location[]> {
        try {
            const definitions = await vscode.commands.executeCommand<vscode.Location[]>(
                'vscode.executeDefinitionProvider',
                document.uri,
                position
            );

            return definitions || [];
        } catch (error) {
            console.warn('Failed to get definitions from rust-analyzer:', error);
            return [];
        }
    }

    /**
     * Get references from rust-analyzer
     */
    async getReferences(
        document: vscode.TextDocument,
        position: vscode.Position,
        includeDeclaration: boolean = false
    ): Promise<vscode.Location[]> {
        try {
            const references = await vscode.commands.executeCommand<vscode.Location[]>(
                'vscode.executeReferenceProvider',
                document.uri,
                position,
                includeDeclaration
            );

            return references || [];
        } catch (error) {
            console.warn('Failed to get references from rust-analyzer:', error);
            return [];
        }
    }

    /**
     * Check if a diagnostic is related to error handling
     */
    isErrorHandlingDiagnostic(diagnostic: DiagnosticInfo): boolean {
        const errorKeywords = [
            'unwrap',
            'expect',
            'panic',
            'Result',
            'Option',
            'error',
            'Error',
            '?'
        ];

        return errorKeywords.some(keyword =>
            diagnostic.message.toLowerCase().includes(keyword.toLowerCase())
        );
    }

    /**
     * Filter diagnostics to only error handling related ones
     */
    filterErrorHandlingDiagnostics(diagnostics: DiagnosticInfo[]): DiagnosticInfo[] {
        return diagnostics.filter(diagnostic => this.isErrorHandlingDiagnostic(diagnostic));
    }

    /**
     * Get macro expansion information if available
     */
    async getMacroExpansion(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<string | null> {
        try {
            // This would require rust-analyzer to support macro expansion queries
            // For now, we'll return null and implement basic macro detection
            return null;
        } catch (error) {
            console.warn('Failed to get macro expansion from rust-analyzer:', error);
            return null;
        }
    }

    /**
     * Check if position is inside a yoshi macro
     */
    async isInsideYoshiMacro(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<boolean> {
        const line = document.lineAt(position.line);
        const textBefore = document.getText(new vscode.Range(
            new vscode.Position(Math.max(0, position.line - 10), 0),
            position
        ));

        // Look for yoshi macro patterns
        const yoshiMacroPatterns = [
            /yoshi_af!\s*\{/,
            /#\[derive\(.*YoshiError.*\)\]/,
            /yopost!/
        ];

        return yoshiMacroPatterns.some(pattern => pattern.test(textBefore));
    }

    /**
     * Map VS Code diagnostic severity to string
     */
    private mapSeverity(severity: vscode.DiagnosticSeverity): string {
        switch (severity) {
            case vscode.DiagnosticSeverity.Error:
                return 'error';
            case vscode.DiagnosticSeverity.Warning:
                return 'warning';
            case vscode.DiagnosticSeverity.Information:
                return 'info';
            case vscode.DiagnosticSeverity.Hint:
                return 'hint';
            default:
                return 'unknown';
        }
    }

    /**
     * Dispose of resources
     */
    dispose(): void {
        this.diagnosticCollection.dispose();
    }
}
