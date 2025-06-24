import * as vscode from 'vscode';
import { AskCodiBridge, AskCodiRequest, AskCodiResponse } from './askcodi-bridge';
import { GitHubCopilotBridge, GitHubCopilotRequest } from './github-copilot-bridge';

/**
 * AI Provider Manager for Yoshi Copilot
 *
 * Manages multiple AI providers (GitHub Copilot, AskCodi) and provides
 * intelligent fallback and load balancing capabilities.
 */

export type AIProvider = 'github-copilot' | 'askcodi' | 'auto';

export interface AIRequest {
    action: 'generate' | 'explain' | 'optimize' | 'refactor' | 'debug' | 'yoshi-enhance';
    code?: string;
    prompt?: string;
    language?: string;
    context?: string;
    yoshiContext?: {
        hasYoshiMacros: boolean;
        errorPatterns: string[];
        suggestedErrorKind?: string;
    };
}

export interface AIResponse {
    success: boolean;
    provider: AIProvider;
    result?: {
        code?: string;
        explanation?: string;
        suggestions?: string[];
        yoshiEnhancements?: {
            macroSuggestions: string[];
            errorHandlingImprovements: string[];
            signpostRecommendations: string[];
        };
    };
    error?: string;
    confidence?: number;
    fallbackUsed?: boolean;
    metadata?: {
        processingTime: number;
        tokensUsed?: number;
        model?: string;
    };
}

export interface AIProviderConfig {
    primaryProvider: AIProvider;
    enableFallback: boolean;
    fallbackOrder: AIProvider[];
    loadBalancing: boolean;
    yoshiSpecificProvider?: AIProvider;
    confidenceThreshold: number;
}

export class AIProviderManager {
    private askCodiBridge: AskCodiBridge;
    private gitHubCopilotBridge: GitHubCopilotBridge;
    private config: AIProviderConfig;
    private outputChannel: vscode.OutputChannel;
    private providerStats: Map<AIProvider, { requests: number; successes: number; avgResponseTime: number }> = new Map();

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        this.askCodiBridge = new AskCodiBridge(outputChannel);
        this.gitHubCopilotBridge = new GitHubCopilotBridge(outputChannel);
        this.config = this.loadConfiguration();
        this.initializeProviderStats();
    }

    /**
     * Process AI request with intelligent provider selection
     */
    public async processRequest(request: AIRequest): Promise<AIResponse> {
        const startTime = Date.now();

        // Determine the best provider for this request
        const selectedProvider = this.selectProvider(request);

        this.outputChannel.appendLine(`🤖 Processing ${request.action} request with ${selectedProvider}`);

        // Try primary provider
        let response = await this.executeRequest(request, selectedProvider);

        // Handle fallback if needed
        if (!response.success && this.config.enableFallback) {
            response = await this.handleFallback(request, selectedProvider);
        }

        // Update statistics
        this.updateProviderStats(response.provider, response.success, Date.now() - startTime);

        // Add Yoshi-specific enhancements if applicable
        if (response.success && request.yoshiContext?.hasYoshiMacros) {
            response = await this.addYoshiEnhancements(response, request);
        }

        return response;
    }

    /**
     * Select the best provider for a given request
     */
    private selectProvider(request: AIRequest): AIProvider {
        // Use Yoshi-specific provider for Yoshi-related requests
        if (request.yoshiContext?.hasYoshiMacros && this.config.yoshiSpecificProvider) {
            return this.config.yoshiSpecificProvider;
        }

        // Use configured primary provider
        if (this.config.primaryProvider !== 'auto') {
            return this.config.primaryProvider;
        }

        // Auto-selection based on request type and provider performance
        return this.autoSelectProvider(request);
    }

    /**
     * Auto-select provider based on performance and request characteristics
     */
    private autoSelectProvider(request: AIRequest): AIProvider {
        const providers: AIProvider[] = ['github-copilot', 'askcodi'];

        // Score providers based on performance and suitability
        const scores = providers.map(provider => {
            const stats = this.providerStats.get(provider);
            if (!stats || stats.requests === 0) {
                return { provider, score: 0.5 }; // Neutral score for untested providers
            }

            const successRate = stats.successes / stats.requests;
            const speedScore = Math.max(0, 1 - (stats.avgResponseTime / 10000)); // Penalize slow responses

            // Adjust score based on request type
            let typeScore = 1.0;
            if (request.action === 'generate' && provider === 'github-copilot') {
                typeScore = 1.2; // GitHub Copilot is excellent for code generation
            } else if (request.action === 'explain' && provider === 'askcodi') {
                typeScore = 1.1; // AskCodi might be better for explanations
            }

            return {
                provider,
                score: (successRate * 0.6 + speedScore * 0.4) * typeScore
            };
        });

        // Return the highest scoring provider
        scores.sort((a, b) => b.score - a.score);
        return scores[0]?.provider || 'github-copilot';
    }

    /**
     * Execute request with specific provider
     */
    private async executeRequest(request: AIRequest, provider: AIProvider): Promise<AIResponse> {
        try {
            switch (provider) {
                case 'askcodi':
                    return await this.executeAskCodiRequest(request);

                case 'github-copilot':
                    return await this.executeGitHubCopilotRequest(request);

                default:
                    return {
                        success: false,
                        provider,
                        error: `Unknown provider: ${provider}`
                    };
            }
        } catch (error) {
            return {
                success: false,
                provider,
                error: `Provider execution error: ${error instanceof Error ? error.message : String(error)}`
            };
        }
    }

    /**
     * Execute request using AskCodi
     */
    private async executeAskCodiRequest(request: AIRequest): Promise<AIResponse> {
        if (!this.askCodiBridge.isAvailable()) {
            return {
                success: false,
                provider: 'askcodi',
                error: 'AskCodi is not available'
            };
        }

        const askCodiRequest: AskCodiRequest = {
            action: request.action as any,
            code: request.code || '',
            prompt: request.prompt || '',
            language: request.language || 'rust',
            context: request.context || ''
        };

        const response = await this.askCodiBridge.generateCode(askCodiRequest);

        return {
            success: response.success,
            provider: 'askcodi',
            result: response.result ? {
                ...(response.result.code && { code: response.result.code }),
                ...(response.result.explanation && { explanation: response.result.explanation }),
                suggestions: response.result.suggestions || []
            } : {
                explanation: 'No result from AskCodi',
                suggestions: []
            },
            ...(response.error && { error: response.error }),
            confidence: response.confidence || 0.5,
            metadata: {
                processingTime: response.metadata?.processingTime || 0,
                ...(response.metadata?.tokens !== undefined && { tokensUsed: response.metadata.tokens }),
                ...(response.metadata?.model !== undefined && { model: response.metadata.model })
            }
        };
    }

    /**
     * Execute request using GitHub Copilot
     */
    private async executeGitHubCopilotRequest(request: AIRequest): Promise<AIResponse> {
        // Convert AIRequest to GitHubCopilotRequest, mapping yoshi-enhance to generate
        const copilotRequest: GitHubCopilotRequest = {
            action: request.action === 'yoshi-enhance' ? 'generate' : request.action as any,
            ...(request.code && { code: request.code }),
            ...(request.prompt && { prompt: request.prompt }),
            ...(request.language && { language: request.language }),
            ...(request.context && { context: request.context })
        };

        const response = await this.gitHubCopilotBridge.processRequest(copilotRequest);
        return {
            success: response.success,
            provider: 'github-copilot',
            result: response.result ? {
                ...(response.result.code && { code: response.result.code }),
                ...(response.result.explanation && { explanation: response.result.explanation }),
                suggestions: response.result.suggestions || []
            } : {
                explanation: 'No result from GitHub Copilot',
                suggestions: []
            },
            ...(response.error && { error: response.error }),
            confidence: response.confidence || 0.5,
            metadata: {
                processingTime: response.metadata?.processingTime || 0,
                ...(response.metadata?.tokens !== undefined && { tokensUsed: response.metadata.tokens }),
                ...(response.metadata?.model !== undefined && { model: response.metadata.model })
            }
        };
    }

    /**
     * Handle fallback to alternative providers
     */
    private async handleFallback(request: AIRequest, failedProvider: AIProvider): Promise<AIResponse> {
        const fallbackProviders = this.config.fallbackOrder.filter(p => p !== failedProvider);

        for (const provider of fallbackProviders) {
            this.outputChannel.appendLine(`🔄 Falling back to ${provider}`);

            const response = await this.executeRequest(request, provider);
            if (response.success) {
                response.fallbackUsed = true;
                return response;
            }
        }

        return {
            success: false,
            provider: failedProvider,
            error: 'All providers failed',
            fallbackUsed: true
        };
    }

    /**
     * Add Yoshi-specific enhancements to the response
     */
    private async addYoshiEnhancements(response: AIResponse, request: AIRequest): Promise<AIResponse> {
        if (!response.result) {
            response.result = {};
        }

        // Generate Yoshi-specific suggestions based on the context
        const yoshiEnhancements = {
            macroSuggestions: this.generateMacroSuggestions(request),
            errorHandlingImprovements: this.generateErrorHandlingImprovements(request),
            signpostRecommendations: this.generateSignpostRecommendations(request)
        };

        response.result.yoshiEnhancements = yoshiEnhancements;

        return response;
    }

    /**
     * Generate Yoshi macro suggestions
     */
    private generateMacroSuggestions(request: AIRequest): string[] {
        const suggestions: string[] = [];

        if (request.code?.includes('.unwrap()')) {
            suggestions.push('Consider using yoshi_af! macro for autonomous error handling');
        }

        if (request.code?.includes('panic!')) {
            suggestions.push('Replace panic! with #[derive(YoshiError)] for better error management');
        }

        if (request.yoshiContext?.errorPatterns.includes('missing_error_enum')) {
            suggestions.push('Add #[derive(YoshiError)] to create comprehensive error types');
        }

        return suggestions;
    }

    /**
     * Generate error handling improvements
     */
    private generateErrorHandlingImprovements(request: AIRequest): string[] {
        const improvements: string[] = [];

        if (request.yoshiContext?.suggestedErrorKind) {
            improvements.push(`Consider using ${request.yoshiContext.suggestedErrorKind} error kind`);
        }

        improvements.push('Implement proper Result<T, E> return types');
        improvements.push('Add meaningful error signposts for better debugging');

        return improvements;
    }

    /**
     * Generate signpost recommendations
     */
    private generateSignpostRecommendations(_request: AIRequest): string[] {
        return [
            'Add descriptive signpost messages for error variants',
            'Include context information in error signposts',
            'Use actionable language in signpost descriptions'
        ];
    }

    /**
     * Initialize provider statistics
     */
    private initializeProviderStats(): void {
        const providers: AIProvider[] = ['github-copilot', 'askcodi'];
        providers.forEach(provider => {
            this.providerStats.set(provider, {
                requests: 0,
                successes: 0,
                avgResponseTime: 0
            });
        });
    }

    /**
     * Update provider statistics
     */
    private updateProviderStats(provider: AIProvider, success: boolean, responseTime: number): void {
        const stats = this.providerStats.get(provider);
        if (!stats) return;

        stats.requests++;
        if (success) stats.successes++;

        // Update average response time
        stats.avgResponseTime = (stats.avgResponseTime * (stats.requests - 1) + responseTime) / stats.requests;

        this.providerStats.set(provider, stats);
    }

    /**
     * Load configuration from VS Code settings
     */
    private loadConfiguration(): AIProviderConfig {
        const config = vscode.workspace.getConfiguration('yoshiCopilot.aiProvider');

        return {
            primaryProvider: config.get('primaryProvider', 'auto'),
            enableFallback: config.get('enableFallback', true),
            fallbackOrder: config.get('fallbackOrder', ['github-copilot', 'askcodi']),
            loadBalancing: config.get('loadBalancing', true),
            yoshiSpecificProvider: config.get('yoshiSpecificProvider', 'askcodi'),
            confidenceThreshold: config.get('confidenceThreshold', 0.7)
        };
    }

    /**
     * Get provider statistics
     */
    public getProviderStats(): Map<AIProvider, { requests: number; successes: number; avgResponseTime: number }> {
        return new Map(this.providerStats);
    }

    /**
     * Test all available providers
     */
    public async testAllProviders(): Promise<{ [key in AIProvider]?: { success: boolean; message: string } }> {
        const results: { [key in AIProvider]?: { success: boolean; message: string } } = {};

        // Test AskCodi
        results.askcodi = await this.askCodiBridge.testConnection();

        // Test GitHub Copilot
        results['github-copilot'] = await this.gitHubCopilotBridge.testConnection();

        return results;
    }

    /**
     * Update configuration
     */
    public updateConfiguration(newConfig: Partial<AIProviderConfig>): void {
        this.config = { ...this.config, ...newConfig };
    }
}
