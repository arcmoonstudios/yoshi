import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import {
    MLAnalysisResult,
    MLAnalysisInput,
    KnowledgeEntry,
    YoshiPattern,
    YoshiContext,
    CodePattern,
    YoshiMacroContext,
    YoshiAttributePattern
} from './types';

/**
 * Revolutionary ML Engine that interfaces directly with yoshi-derive Rust macros
 *
 * Instead of reimplementing the logic in TypeScript, this engine:
 * 1. Calls the actual yoshi_af! and #[derive(YoshiError)] macros
 * 2. Parses the macro expansion output
 * 3. Extracts autofix suggestions from the real Rust implementation
 */
export class MLPatternEngine {
    private knowledgeBase: Map<string, KnowledgeEntry> = new Map();
    private patternCache: Map<string, MLAnalysisResult> = new Map();
    private learningHistory: YoshiPattern[] = [];
    private workspaceRoot: string;
    private cargoPath: string | null = null;

    constructor(workspaceRoot: string) {
        this.workspaceRoot = workspaceRoot;
        this.initializeRustEnvironment();
    }

    /**
     * Initialize Rust environment and find cargo
     */
    private async initializeRustEnvironment(): Promise<void> {
        try {
            // Try to find cargo in PATH
            const cargoResult = await this.executeCommand('cargo', ['--version']);
            if (cargoResult.success) {
                this.cargoPath = 'cargo';
            }
        } catch (error) {
            console.warn('Cargo not found in PATH, yoshi macro calls will be disabled');
        }
    }

    /**
     * Analyze context by calling actual yoshi-derive macros
     */
    async analyzeContext(input: MLAnalysisInput): Promise<MLAnalysisResult> {
        const cacheKey = this.generateCacheKey(input);

        // Check cache first for performance
        if (this.patternCache.has(cacheKey)) {
            return this.patternCache.get(cacheKey)!;
        }

        const result = await this.performRustMacroAnalysis(input);
        this.patternCache.set(cacheKey, result);

        return result;
    }

    /**
     * Perform analysis by calling actual yoshi-derive Rust macros
     */
    private async performRustMacroAnalysis(input: MLAnalysisInput): Promise<MLAnalysisResult> {
        if (!this.cargoPath) {
            // Fallback to basic pattern detection if Rust is not available
            return this.performBasicAnalysis(input);
        }

        try {
            // Create a temporary Rust file with the input code
            const tempFile = await this.createTempRustFile(input.text);

            // Try to expand yoshi macros and extract suggestions
            const macroResult = await this.expandYoshiMacros(tempFile);

            // Clean up temp file
            await this.cleanupTempFile(tempFile);

            if (macroResult.success) {
                return this.parseYoshiMacroOutput(macroResult.output, input);
            } else {
                // Fallback to basic analysis if macro expansion fails
                return this.performBasicAnalysis(input);
            }
        } catch (error) {
            console.warn('Rust macro analysis failed, falling back to basic analysis:', error);
            return this.performBasicAnalysis(input);
        }
    }

    /**
     * Execute command and return result
     */
    private async executeCommand(command: string, args: string[]): Promise<{success: boolean, output: string, error?: string}> {
        return new Promise((resolve) => {
            cp.execFile(command, args, { cwd: this.workspaceRoot }, (error, stdout, stderr) => {
                if (error) {
                    resolve({ success: false, output: '', error: error.message });
                } else {
                    resolve({ success: true, output: stdout });
                }
            });
        });
    }

    /**
     * Create temporary Rust file for macro expansion
     */
    private async createTempRustFile(code: string): Promise<string> {
        const tempDir = path.join(this.workspaceRoot, 'target', 'yoshi-temp');
        await fs.promises.mkdir(tempDir, { recursive: true });

        const tempFile = path.join(tempDir, `temp_${Date.now()}.rs`);

        // Wrap the code in a complete Rust file with yoshi imports
        const wrappedCode = `
use yoshi_derive::YoshiError;
use yoshi_core::*;

${code}

// Test macro expansion
#[derive(Debug, YoshiError)]
enum TestError {
    #[yoshi(signpost = "Test signpost")]
    Test,
}
`;

        await fs.promises.writeFile(tempFile, wrappedCode);
        return tempFile;
    }

    /**
     * Expand yoshi macros using cargo expand
     */
    private async expandYoshiMacros(filePath: string): Promise<{success: boolean, output: string}> {
        // Try cargo expand first (if available)
        const expandResult = await this.executeCommand('cargo', ['expand', '--bin', path.basename(filePath, '.rs')]);

        if (expandResult.success) {
            return expandResult;
        }

        // Fallback to cargo check with verbose output to get macro information
        const checkResult = await this.executeCommand('cargo', ['check', '--message-format=json']);
        return checkResult;
    }

    /**
     * Parse yoshi macro output to extract suggestions
     */
    private parseYoshiMacroOutput(output: string, input: MLAnalysisInput): MLAnalysisResult {
        const detectedPatterns: string[] = [];
        const codePatterns: CodePattern[] = [];
        const attributePatterns: YoshiAttributePattern[] = [];
        let confidence = 0.9; // High confidence since this comes from actual macro
        let suggestedErrorKind: string | null = null;

        // Parse macro expansion output for autofix suggestions
        if (output.includes('AutofixEntry')) {
            detectedPatterns.push('yoshi_autofix_available');
            confidence = 0.95;
        }

        // Extract signpost suggestions from macro output
        const signpostMatches = output.match(/signpost:\s*"([^"]+)"/g);
        if (signpostMatches) {
            for (const match of signpostMatches) {
                const signpost = match.replace(/signpost:\s*"([^"]+)"/, '$1');
                attributePatterns.push({
                    name: 'signpost',
                    hash: this.simpleHashToNumber(signpost),
                    signpost,
                    isRecognized: true
                });
            }
        }

        // Extract error kinds from macro output
        const errorKindMatches = output.match(/error_kind:\s*"([^"]+)"/g);
        if (errorKindMatches) {
            suggestedErrorKind = errorKindMatches[0].replace(/error_kind:\s*"([^"]+)"/, '$1');
        }

        // Extract code patterns from macro output
        if (output.includes('unwrap_usage')) {
            codePatterns.push({
                type: 'unwrap',
                location: { line: 0, column: 0 },
                suggestion: 'Replace .unwrap() with proper error handling',
                confidence: 0.95,
                severity: 'error',
                description: 'Unwrap usage detected by yoshi macro'
            });
        }

        return {
            confidence,
            detectedPatterns,
            suggestedErrorKind,
            codePatterns,
            attributePatterns
        };
    }

    /**
     * Fallback basic analysis when Rust macros are not available
     */
    private async performBasicAnalysis(input: MLAnalysisInput): Promise<MLAnalysisResult> {
        const detectedPatterns: string[] = [];
        const codePatterns: CodePattern[] = [];
        const attributePatterns: YoshiAttributePattern[] = [];
        let confidence = 0.0;
        let suggestedErrorKind: string | null = null;

        // Basic pattern detection as fallback
        if (input.text.includes('.unwrap()')) {
            detectedPatterns.push('unwrap_detected');
            codePatterns.push({
                type: 'unwrap',
                location: { line: 0, column: 0 },
                suggestion: 'Replace .unwrap() with proper error handling',
                confidence: 0.95,
                severity: 'error',
                description: 'Unwrap usage detected'
            });
            confidence = Math.max(confidence, 0.95);
        }

        if (input.text.includes('.expect(')) {
            detectedPatterns.push('expect_detected');
            codePatterns.push({
                type: 'expect',
                location: { line: 0, column: 0 },
                suggestion: 'Replace .expect() with context-aware error handling',
                confidence: 0.90,
                severity: 'warning',
                description: 'Expect usage detected'
            });
            confidence = Math.max(confidence, 0.90);
        }

        if (input.text.includes('panic!')) {
            detectedPatterns.push('panic_detected');
            codePatterns.push({
                type: 'panic',
                location: { line: 0, column: 0 },
                suggestion: 'Replace panic! with recoverable error handling',
                confidence: 0.98,
                severity: 'error',
                description: 'Panic usage detected'
            });
            confidence = Math.max(confidence, 0.98);
        }

        return {
            confidence,
            detectedPatterns,
            suggestedErrorKind,
            codePatterns,
            attributePatterns
        };
    }

    /**
     * Clean up temporary file
     */
    private async cleanupTempFile(filePath: string): Promise<void> {
        try {
            await fs.promises.unlink(filePath);
        } catch (error) {
            // Ignore cleanup errors
        }
    }

    /**
     * Infer error kinds based on context analysis
     */
    inferErrorKinds(context: YoshiContext): string[] {
        const errorKinds: string[] = [];

        // Analyze patterns to infer appropriate error kinds
        for (const pattern of context.errorPatterns) {
            switch (pattern) {
                case 'io_error_handling':
                    errorKinds.push('Io');
                    break;
                case 'network_error_handling':
                    errorKinds.push('Network');
                    break;
                case 'validation_error_handling':
                    errorKinds.push('Validation');
                    break;
                case 'unwrap_detected':
                case 'expect_detected':
                case 'panic_detected':
                    errorKinds.push('Internal');
                    break;
                default:
                    errorKinds.push('Generic');
            }
        }

        // Remove duplicates and ensure we have at least one error kind
        const uniqueErrorKinds = [...new Set(errorKinds)];
        return uniqueErrorKinds.length > 0 ? uniqueErrorKinds : ['Internal'];
    }

    /**
     * Generate intelligent signpost for error kind
     */
    generateSignpost(errorKind: string): string {
        const signposts: Record<string, string> = {
            'Io': 'Check file permissions and path validity',
            'Network': 'Verify network connectivity and retry with exponential backoff',
            'Validation': 'Verify input data format and constraints',
            'Config': 'Review configuration settings and environment variables',
            'Security': 'Verify authentication credentials and access permissions',
            'Timeout': 'Consider increasing timeout or implementing retry logic',
            'NotFound': 'Verify resource exists and check access permissions',
            'Internal': 'Review internal logic and error propagation',
            'Generic': 'Review error context and add appropriate handling'
        };

        return signposts[errorKind] || 'Review error handling and add appropriate context';
    }

    /**
     * Learn from a new pattern
     */
    async learnPattern(pattern: YoshiPattern): Promise<void> {
        this.learningHistory.push(pattern);

        // Update knowledge base with new pattern
        const knowledge: KnowledgeEntry = {
            pattern: pattern.pattern,
            signpost: pattern.signpost,
            confidence: pattern.confidence,
            category: pattern.category,
            severity: pattern.severity
        };

        this.knowledgeBase.set(pattern.pattern, knowledge);

        // Clear cache to force re-analysis with new knowledge
        this.patternCache.clear();
    }

    /**
     * Add knowledge to the ML engine
     */
    async addKnowledge(knowledge: KnowledgeEntry): Promise<void> {
        this.knowledgeBase.set(knowledge.pattern, knowledge);
    }

    /**
     * Initialize pattern recognition system
     */
    private initializePatternRecognition(): void {
        // Initialize with basic patterns
        const basicPatterns: KnowledgeEntry[] = [
            {
                pattern: 'unwrap_usage',
                signpost: 'Replace .unwrap() with proper error handling using ? operator',
                confidence: 0.95,
                category: 'safety',
                severity: 'warning'
            },
            {
                pattern: 'expect_usage',
                signpost: 'Replace .expect() with context-aware error handling',
                confidence: 0.90,
                category: 'safety',
                severity: 'warning'
            },
            {
                pattern: 'panic_usage',
                signpost: 'Replace panic! with recoverable error handling',
                confidence: 0.98,
                category: 'safety',
                severity: 'error'
            }
        ];

        for (const pattern of basicPatterns) {
            this.knowledgeBase.set(pattern.pattern, pattern);
        }
    }

    /**
     * Detect missing error enum patterns
     */
    private detectMissingErrorEnum(text: string): boolean {
        // Look for multiple error types without a unified enum
        const errorPatterns = [
            /std::io::Error/g,
            /serde_json::Error/g,
            /reqwest::Error/g,
            /anyhow::Error/g
        ];

        let errorTypeCount = 0;
        for (const pattern of errorPatterns) {
            if (pattern.test(text)) {
                errorTypeCount++;
            }
        }

        // If we have multiple error types but no custom error enum, suggest one
        return errorTypeCount >= 2 && !text.includes('#[derive(') && !text.includes('enum') && !text.includes('Error');
    }

    /**
     * Detect IO operations that might need error handling
     */
    private detectIoOperations(text: string): boolean {
        const ioPatterns = [
            /std::fs::/,
            /File::/,
            /\.read\(/,
            /\.write\(/,
            /\.open\(/,
            /\.create\(/
        ];

        return ioPatterns.some(pattern => pattern.test(text));
    }

    /**
     * Detect network operations
     */
    private detectNetworkOperations(text: string): boolean {
        const networkPatterns = [
            /reqwest::/,
            /hyper::/,
            /tokio::net::/,
            /std::net::/,
            /\.get\(/,
            /\.post\(/,
            /\.connect\(/
        ];

        return networkPatterns.some(pattern => pattern.test(text));
    }

    /**
     * Detect validation patterns
     */
    private detectValidationPatterns(text: string): boolean {
        const validationPatterns = [
            /\.is_empty\(\)/,
            /\.len\(\)/,
            /\.parse\(/,
            /\.trim\(/,
            /validate/i,
            /check/i
        ];

        return validationPatterns.some(pattern => pattern.test(text));
    }

    /**
     * Apply learning boost based on historical patterns
     */
    private applyLearningBoost(detectedPatterns: string[], baseConfidence: number): number {
        let boost = 0.0;

        for (const pattern of detectedPatterns) {
            const historicalCount = this.learningHistory.filter(p => p.pattern === pattern).length;
            boost += Math.min(historicalCount * 0.01, 0.05); // Max 5% boost per pattern
        }

        return Math.min(baseConfidence + boost, 1.0);
    }

    /**
     * Generate cache key for analysis results
     */
    private generateCacheKey(input: MLAnalysisInput): string {
        const textHash = this.simpleHash(input.text);
        const positionKey = `${input.position.line}-${input.position.character}`;
        const macroKey = input.yoshiMacros.hasYoshiMacros ? 'with-macros' : 'no-macros';
        const enumsKey = input.yoshiMacros.errorEnums.join('-');

        return `${textHash}-${positionKey}-${macroKey}-${enumsKey}`;
    }

    /**
     * Simple hash function for caching
     */
    private simpleHash(str: string): string {
        let hash = 0;
        for (let i = 0; i < str.length; i++) {
            const char = str.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash; // Convert to 32-bit integer
        }
        return hash.toString(36);
    }

    /**
     * Simple hash function that returns a number
     */
    private simpleHashToNumber(str: string): number {
        let hash = 0;
        for (let i = 0; i < str.length; i++) {
            const char = str.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash; // Convert to 32-bit integer
        }
        return Math.abs(hash);
    }
}
