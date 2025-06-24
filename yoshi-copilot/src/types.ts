/**
 * Type definitions for the revolutionary Yoshi Copilot
 *
 * These types interface with the actual Rust macro implementations
 * rather than reimplementing the logic in TypeScript
 */

// Rust macro integration types
export interface RustMacroCall {
    macroType: 'yoshi_af' | 'derive_yoshi_error';
    sourceCode: string;
    fileName: string;
    expandedOutput?: string;
}

export interface RustMacroResult {
    success: boolean;
    output?: string;
    error?: string;
    diagnostics: RustDiagnostic[];
    suggestions: RustSuggestion[];
}

export interface MacroExpansion {
    originalCode: string;
    expandedCode: string;
    optimizations: string[];
    detectedPatterns: string[];
}

export interface RustSuggestion {
    message: string;
    replacement: string;
    span: { start: number; end: number };
    applicability: 'MachineApplicable' | 'MaybeIncorrect' | 'HasPlaceholders';
}

export interface RustDiagnostic {
    message: string;
    severity: 'error' | 'warning' | 'info' | 'hint';
    spans: Array<{ start: number; end: number; file: string }>;
    code?: string;
    suggestions: RustSuggestion[];
}

// Types for interfacing with yoshi-derive analysis
export interface YoshiDeriveAnalysis {
    enumName?: string;
    variants: YoshiVariantInfo[];
    autofixEntries: AutofixEntry[];
    generatedMethods?: string[];
    traitImplementations?: string[];
    defaultSeverity: number;
    attributeHashes: Map<string, number>;
}

export interface YoshiVariantInfo {
    name: string;
    fields: FieldInfo[];
    attributes: VariantAttribute[];
}

export interface FieldInfo {
    name?: string;
    fieldType: string;
    attributes: VariantAttribute[];
}

export interface VariantAttribute {
    name: string;
    value?: string;
    arguments?: string[];
}

export interface AutofixEntry {
    variantName: string;
    suggestion: string;
    category: string;
    severity: string;
    confidence: number;
    hash?: number;
}

// Types for calling the actual Rust yoshi_af! macro
export interface YoshiAfCall {
    functionCode: string;
    fileName: string;
    context?: string;
}

export interface YoshiAfResult {
    success: boolean;
    optimizedCode?: string;
    detectedPatterns: string[];
    appliedOptimizations: string[];
    suggestions: AutofixSuggestion[];
    errors?: string[];
}

// Legacy types for backward compatibility (to be migrated to Rust calls)
export interface YoshiAttributePattern {
    name: string;
    hash: number;
    signpost?: string;
    isRecognized: boolean;
}

export interface AttributeHash {
    hash: number;
    pattern: string;
    signpost?: string;
}

export interface YoshiErrorKind {
    name: string;
    description: string;
    severity: number;
    transient: boolean;
    patterns: string[];
}

export interface YoshiFieldAnalysis {
    ident?: string;
    ty: string;
    source: boolean;
    backtrace: boolean;
    context?: string;
    shell: boolean;
    skip: boolean;
    sensitive: boolean;
    formatWith?: string;
    transform?: string;
}

export interface YoshiVariantOpts {
    ident: string;
    fields: YoshiFieldAnalysis[];
    display?: string;
    kind?: string;
    severity?: number;
    signpost?: string;
    suggestion?: string;
    transient: boolean;
    transparent: boolean;
    from: boolean;
    skip: boolean;
    code?: number;
    category?: string;
    docUrl?: string;
}

export interface AttributeHash {
    hash: number;
    pattern: string;
    signpost?: string;
}

export interface YoshiErrorKind {
    name: string;
    description: string;
    severity: number;
    transient: boolean;
    patterns: string[];
}

export interface YoshiFieldAnalysis {
    ident?: string;
    ty: string;
    source: boolean;
    backtrace: boolean;
    context?: string;
    shell: boolean;
    skip: boolean;
    sensitive: boolean;
    formatWith?: string;
    transform?: string;
}

export interface YoshiVariantOpts {
    ident: string;
    fields: YoshiFieldAnalysis[];
    display?: string;
    kind?: string;
    severity?: number;
    signpost?: string;
    suggestion?: string;
    transient: boolean;
    transparent: boolean;
    from: boolean;
    skip: boolean;
    code?: number;
    category?: string;
    docUrl?: string;
}

export interface CodePattern {
    type: 'unwrap' | 'expect' | 'panic' | 'todo' | 'fixme' | 'unreachable' | 'unimplemented' | 'unsafe' | 'unknown';
    location: { line: number; column: number };
    suggestion: string;
    confidence: number;
    severity: 'error' | 'warning' | 'info';
    description: string;
}

export interface YoshiContext {
    isErrorHandlingContext: boolean;
    confidence: number;
    errorPatterns: string[];
    yoshiMacroContext: YoshiMacroContext;
    suggestedErrorKind: string | null;
    availableAutofixes: AutofixSuggestion[];
    codebasePatterns: string[];
    detectedAttributes: YoshiAttributePattern[];
    inferredKind: YoshiErrorKind | null;
    fieldAnalysis: YoshiFieldAnalysis[];
}

export interface YoshiMacroContext {
    hasYoshiMacros: boolean;
    errorEnums: string[];
    autofixPatterns: string[];
    deriveMacros: YoshiDeriveMacroInfo[];
    afMacros: YoshiAfMacroInfo[];
}

export interface YoshiDeriveMacroInfo {
    location: { line: number; column: number };
    enumName: string;
    variants: YoshiVariantOpts[];
    attributes: YoshiAttributePattern[];
    defaultSeverity: number;
    defaultKind?: string;
    errorCodeBase?: number;
}

export interface YoshiAfMacroInfo {
    location: { line: number; column: number };
    content: string;
    detectedPatterns: CodePattern[];
    optimizations: string[];
}

export interface YoshiSuggestion {
    code: string;
    confidence: number;
    reasoning: string;
    pattern: string;
    errorKind: string;
    mlConfidence: number;
    sourceField?: string;
    displayFormat?: string;
    autofixEntry?: AutofixEntry;
}

export interface AutofixEntry {
    variantName: string;
    suggestion: string;
    category: string;
    severity: string;
    confidence: number;
}

export interface AutofixSuggestion {
    suggestion: string;
    confidence: number;
    errorKind: string;
    code: string;
    title: string;
    description: string;
    source: string;
    impact: string;
    severity: string;
    quickFix: boolean;
    autofixEntry?: AutofixEntry;
    quickFixes?: string[];
}

export interface ErrorContext {
    isErrorContext: boolean;
    autofixes: AutofixSuggestion[];
}

export interface YoshiPattern {
    id: string;
    pattern: string;
    confidence: number;
    errorKind: string;
    signpost: string;
    hash?: number;
    severity: string;
    category: string;
}

export interface MLAnalysisResult {
    confidence: number;
    detectedPatterns: string[];
    suggestedErrorKind: string | null;
    codePatterns: CodePattern[];
    attributePatterns: YoshiAttributePattern[];
    inferredKind?: YoshiErrorKind;
    fieldAnalysis?: YoshiFieldAnalysis[];
}

export interface MLAnalysisInput {
    text: string;
    position: any;
    diagnostics: any[];
    yoshiMacros: YoshiMacroContext;
}

export interface KnowledgeEntry {
    pattern: string;
    signpost: string;
    confidence: number;
    hash?: number;
    category: string;
    severity: string;
}

export interface DiagnosticInfo {
    message: string;
    severity: string;
    range: any;
    source: string;
    errorType?: string;
    variant?: string;
    autofixAvailable?: boolean;
    quickFixCount?: number;
    metadataCount?: number;
}

export interface YoshiMacroInfo {
    type: 'yoshi_af' | 'derive_yoshi_error';
    location: { line: number; column: number };
    content: string;
    generatedPatterns: string[];
    attributes?: YoshiAttributePattern[];
    variants?: YoshiVariantOpts[];
    errorKind?: string;
}

export interface LearningMetrics {
    patternsLearned: number;
    confidenceImprovement: number;
    suggestionsAccepted: number;
    errorsFixed: number;
    hashBasedDetections: number;
    attributePatternsLearned: number;
}

export interface ConfigurationSettings {
    enableMLSuggestions: boolean;
    confidenceThreshold: number;
    enableAutonomousOptimization: boolean;
    learningMode: 'aggressive' | 'moderate' | 'conservative';
    enableRealTimeSuggestions: boolean;
    rustAnalyzerIntegration: boolean;

    // P.R.I.M.E. Framework Settings
    enablePRIMEFramework: boolean;
    primeRecursionDepth: number;
    enableResearchAugmentation: boolean;
    enableAutonomousCodeGen: boolean;

    // GitHub Copilot Integration
    enableCopilotIntegration: boolean;
    copilotConfidenceThreshold: number;

    // Advanced Features
    enableASTOptimization: boolean;
    enableRealTimeResearch: boolean;
    maxConcurrentAnalyses: number;

    // Hash-based pattern recognition
    enableHashBasedDetection: boolean;
    enableAdvancedInference: boolean;
    enableSourceFieldDetection: boolean;
}

export interface InsightData {
    patternsLearned: number;
    confidenceLevel: number;
    suggestionsGenerated: number;
    errorsFixed: number;
    topPatterns: string[];
    recentActivity: ActivityEntry[];
    primeAnalyses?: number;
    revolutionaryConfidence?: number;
    autonomousOptimizations?: number;
    copilotIntegrations?: number;
    certificationLevel?: string;
    memoryOptimized?: number;
    cpuOptimized?: number;
    errorsPrevent?: number;
    codeQuality?: number;
}

export interface ActivityEntry {
    timestamp: Date;
    action: string;
    pattern: string;
    confidence: number;
}

export interface ErrorAnalysisResult {
    errorType: string;
    severity: 'low' | 'medium' | 'high' | 'critical';
    suggestions: string[];
    autoFixAvailable: boolean;
    confidence: number;
    inferredKind?: YoshiErrorKind;
    sourceFields?: YoshiFieldAnalysis[];
}

export interface CodebaseAnalysis {
    totalFiles: number;
    rustFiles: number;
    yoshiUsage: number;
    errorPatterns: Map<string, number>;
    complexityScore: number;
    recommendations: string[];
    hashBasedPatterns: Map<number, string>;
    attributePatterns: YoshiAttributePattern[];
}

export interface SuggestionMetadata {
    yoshiPattern: string;
    errorKind: string;
    mlConfidence: number;
    sourceAnalysis: string;
    applicabilityScore: number;
    hash?: number;
    autofixEntry?: AutofixEntry;
}

export interface CompletionRequest {
    document: any;
    position: any;
    context: any;
}

export interface CompletionResponse {
    completions: CompletionItem[];
}

export interface CompletionItem {
    text: string;
    confidence: number;
    reasoning: string;
    metadata: SuggestionMetadata;
}

// Advanced pattern recognition types that mirror the Rust implementation
export interface PatternDetectionResult {
    patterns: CodePattern[];
    attributeHashes: AttributeHash[];
    inferredErrorKinds: YoshiErrorKind[];
    confidence: number;
    suggestions: AutofixSuggestion[];
}

export interface HashBasedPatternAnalysis {
    hashMatches: Map<number, string>;
    signposts: Map<string, string>;
    confidence: number;
    detectedAttributes: YoshiAttributePattern[];
}

export interface AdvancedInferenceResult {
    inferredKind: string;
    inferredSeverity: number;
    inferredTransient: boolean;
    sourceField?: YoshiFieldAnalysis;
    displayFormat?: string;
    signpost?: string;
    confidence: number;
}

export interface AutofixTriggerEvent {
    triggerType: string;
    patternType: string;
    functionName?: string;
    filePath: string;
    line: number;
    column: number;
    hashBased: boolean;
    confidence: number;
    signpost?: string;
    severity: string;
}

export interface CompileTimeOptimization {
    optimizationType: string;
    description: string;
    performanceGain: string;
    memoryImpact?: number;
    cpuImpact?: number;
    confidence: number;
    applied: boolean;
}

export interface YoshiMacroAnalysisResult {
    deriveMacros: YoshiDeriveMacroInfo[];
    afMacros: YoshiAfMacroInfo[];
    patterns: CodePattern[];
    attributePatterns: YoshiAttributePattern[];
    optimizations: CompileTimeOptimization[];
    confidence: number;
    errorCoverage: number;
    recommendations: string[];
}

