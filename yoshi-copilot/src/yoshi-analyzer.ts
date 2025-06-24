import * as vscode from 'vscode';
import { YoshiMacroContext, YoshiPattern, YoshiMacroInfo } from './types';

/**
 * Yoshi Macro Analyzer for detecting and analyzing yoshi macro usage
 */
export class YoshiMacroAnalyzer {
    private macroCache: Map<string, YoshiMacroInfo[]> = new Map();

    /**
     * Find yoshi macros in a document
     */
    async findYoshiMacros(document: vscode.TextDocument): Promise<YoshiMacroContext> {
        const text = document.getText();
        const macros = this.extractYoshiMacros(text);

        return {
            hasYoshiMacros: macros.length > 0,
            errorEnums: this.extractErrorEnums(text),
            autofixPatterns: this.extractAutofixPatterns(macros),
            deriveMacros: macros.filter(m => m.type === 'derive_yoshi_error').map(m => ({
                enumName: this.extractEnumName(m.content || ''),
                variants: [],
                attributes: [],
                location: m.location,
                defaultSeverity: 2
            })),
            afMacros: macros.filter(m => m.type === 'yoshi_af').map(m => ({
                functionName: 'unknown',
                originalCode: m.content || '',
                enhancedCode: m.content || '',
                errorHandlingAdded: true,
                location: m.location,
                content: m.content || '',
                detectedPatterns: m.generatedPatterns.map(p => ({
                    pattern: p,
                    confidence: 0.8,
                    location: { line: 0, column: 0 },
                    type: 'unknown',
                    suggestion: `Apply ${p} pattern`,
                    severity: 'warning',
                    description: `Detected ${p} pattern in yoshi_af macro`
                })),
                optimizations: []
            }))
        };
    }

    /**
     * Extract patterns from a document for learning
     */
    async extractPatterns(document: vscode.TextDocument): Promise<YoshiPattern[]> {
        const text = document.getText();
        const patterns: YoshiPattern[] = [];

        // Extract yoshi_af! macro patterns
        const yoshiAfMatches = text.matchAll(/yoshi_af!\s*\{([^}]+)\}/gs);
        for (const match of yoshiAfMatches) {
            const content = match[1];
            if (!content) continue;
            const pattern = this.analyzeYoshiAfContent(content);
            if (pattern) {
                patterns.push({
                    id: `yoshi_af_${Date.now()}_${Math.random()}`,
                    pattern: pattern.type,
                    confidence: pattern.confidence,
                    errorKind: pattern.errorKind,
                    signpost: pattern.signpost,
                    severity: 'warning',
                    category: 'yoshi_af'
                });
            }
        }

        // Extract derive macro patterns
        const deriveMatches = text.matchAll(/#\[derive\(.*YoshiError.*\)\]\s*(?:pub\s+)?enum\s+(\w+)\s*\{([^}]+)\}/gs);
        for (const match of deriveMatches) {
            const enumName = match[1];
            const enumContent = match[2];
            if (!enumName || !enumContent) continue;
            const pattern = this.analyzeDeriveContent(enumName, enumContent);
            if (pattern) {
                patterns.push({
                    id: `derive_${enumName}_${Date.now()}`,
                    pattern: pattern.type,
                    confidence: pattern.confidence,
                    errorKind: pattern.errorKind,
                    signpost: pattern.signpost,
                    severity: 'info',
                    category: 'derive'
                });
            }
        }

        return patterns;
    }

    /**
     * Extract yoshi macros from text
     */
    private extractYoshiMacros(text: string): YoshiMacroInfo[] {
        const macros: YoshiMacroInfo[] = [];

        // Find yoshi_af! macros
        const yoshiAfMatches = text.matchAll(/yoshi_af!\s*\{([^}]+)\}/gs);
        for (const match of yoshiAfMatches) {
            const content = match[1];
            if (!content) continue;

            const startIndex = match.index || 0;
            const lines = text.substring(0, startIndex).split('\n');
            const line = lines.length - 1;
            const column = lines[lines.length - 1]?.length || 0;

            macros.push({
                type: 'yoshi_af',
                location: { line, column },
                content: content,
                generatedPatterns: this.extractPatternsFromYoshiAf(content)
            });
        }

        // Find derive macros
        const deriveMatches = text.matchAll(/#\[derive\(.*YoshiError.*\)\]/gs);
        for (const match of deriveMatches) {
            const startIndex = match.index || 0;
            const lines = text.substring(0, startIndex).split('\n');
            const line = lines.length - 1;
            const column = lines[lines.length - 1]?.length || 0;

            macros.push({
                type: 'derive_yoshi_error',
                location: { line, column },
                content: match[0],
                generatedPatterns: this.extractPatternsFromDerive(match[0])
            });
        }

        return macros;
    }

    /**
     * Extract error enums from text
     */
    private extractErrorEnums(text: string): string[] {
        const errorEnums: string[] = [];

        // Look for enums with YoshiError derive
        const enumMatches = text.matchAll(/#\[derive\(.*YoshiError.*\)\]\s*(?:pub\s+)?enum\s+(\w+)/gs);
        for (const match of enumMatches) {
            const enumName = match[1];
            if (enumName) {
                errorEnums.push(enumName);
            }
        }

        return errorEnums;
    }

    /**
     * Extract autofix patterns from macros
     */
    private extractAutofixPatterns(macros: YoshiMacroInfo[]): string[] {
        const patterns: string[] = [];

        for (const macro of macros) {
            patterns.push(...macro.generatedPatterns);
        }

        return [...new Set(patterns)]; // Remove duplicates
    }

    /**
     * Extract patterns from yoshi_af! macro content
     */
    private extractPatternsFromYoshiAf(content: string): string[] {
        const patterns: string[] = [];

        // Detect unwrap patterns
        if (content.includes('.unwrap()')) {
            patterns.push('unwrap_detected');
        }

        // Detect expect patterns
        if (content.includes('.expect(')) {
            patterns.push('expect_detected');
        }

        // Detect panic patterns
        if (content.includes('panic!')) {
            patterns.push('panic_detected');
        }

        // Detect function patterns
        if (content.includes('fn ')) {
            patterns.push('function_enhancement');
        }

        // Detect async patterns
        if (content.includes('async ')) {
            patterns.push('async_enhancement');
        }

        return patterns;
    }

    /**
     * Extract patterns from derive macro
     */
    private extractPatternsFromDerive(content: string): string[] {
        const patterns: string[] = [];

        // Basic derive pattern
        patterns.push('derive_yoshi_error');

        // Check for specific attributes
        if (content.includes('signpost')) {
            patterns.push('signpost_usage');
        }

        if (content.includes('kind')) {
            patterns.push('kind_specification');
        }

        if (content.includes('confidence')) {
            patterns.push('confidence_specification');
        }

        return patterns;
    }

    /**
     * Analyze yoshi_af! content for pattern extraction
     */
    private analyzeYoshiAfContent(content: string): {
        type: string;
        confidence: number;
        errorKind: string;
        signpost: string;
    } | null {
        // Analyze the content to determine pattern type
        if (content.includes('.unwrap()')) {
            return {
                type: 'unwrap_replacement',
                confidence: 0.95,
                errorKind: 'Internal',
                signpost: 'Replace .unwrap() with proper error handling'
            };
        }

        if (content.includes('async ')) {
            return {
                type: 'async_enhancement',
                confidence: 0.88,
                errorKind: 'Async',
                signpost: 'Enhanced async function with error handling'
            };
        }

        if (content.includes('fn ')) {
            return {
                type: 'function_enhancement',
                confidence: 0.85,
                errorKind: 'Generic',
                signpost: 'Enhanced function with yoshi error handling'
            };
        }

        return null;
    }

    /**
     * Analyze derive content for pattern extraction
     */
    private analyzeDeriveContent(enumName: string, enumContent: string): {
        type: string;
        confidence: number;
        errorKind: string;
        signpost: string;
    } | null {
        // Count variants to determine complexity
        const variants = enumContent.split(',').filter(v => v.trim().length > 0);

        if (variants.length > 5) {
            return {
                type: 'complex_error_enum',
                confidence: 0.92,
                errorKind: 'Multiple',
                signpost: `Complex error enum ${enumName} with ${variants.length} variants`
            };
        } else {
            return {
                type: 'simple_error_enum',
                confidence: 0.88,
                errorKind: 'Simple',
                signpost: `Simple error enum ${enumName} with ${variants.length} variants`
            };
        }
    }

    /**
     * Check if a position is inside a yoshi macro
     */
    async isInsideYoshiMacro(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<boolean> {
        const text = document.getText();
        const offset = document.offsetAt(position);

        // Check for yoshi_af! macro
        const yoshiAfMatches = text.matchAll(/yoshi_af!\s*\{/gs);
        for (const match of yoshiAfMatches) {
            const startIndex = match.index || 0;
            const endIndex = this.findMatchingBrace(text, startIndex + match[0].length - 1);

            if (offset >= startIndex && offset <= endIndex) {
                return true;
            }
        }

        // Check for derive macro context
        const lines = text.split('\n');
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            if (line && line.includes('#[derive(') && line.includes('YoshiError')) {
                // Check if position is in the enum that follows
                for (let j = i + 1; j < Math.min(i + 10, lines.length); j++) {
                    if (position.line === j) {
                        return true;
                    }
                    const currentLine = lines[j];
                    if (currentLine && currentLine.includes('}')) {
                        break;
                    }
                }
            }
        }

        return false;
    }

    /**
     * Find matching brace for macro content
     */
    private findMatchingBrace(text: string, startIndex: number): number {
        let braceCount = 1;
        let index = startIndex + 1;

        while (index < text.length && braceCount > 0) {
            if (text[index] === '{') {
                braceCount++;
            } else if (text[index] === '}') {
                braceCount--;
            }
            index++;
        }

        return index - 1;
    }

    /**
     * Get macro information at a specific position
     */
    async getMacroInfoAtPosition(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<YoshiMacroInfo | null> {
        const macros = this.extractYoshiMacros(document.getText());

        for (const macro of macros) {
            // Simple proximity check - in a real implementation,
            // you'd want more precise range checking
            if (Math.abs(macro.location.line - position.line) <= 5) {
                return macro;
            }
        }

        return null;
    }

    /**
     * Extract enum name from derive macro content
     */
    private extractEnumName(content: string): string {
        const match = content.match(/enum\s+(\w+)/);
        return match?.[1] || 'UnknownEnum';
    }

    /**
     * Clear the macro cache
     */
    clearCache(): void {
        this.macroCache.clear();
    }
}
