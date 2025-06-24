import * as vscode from 'vscode';

/**
 * AskCodi API Bridge for Yoshi Copilot
 *
 * This bridge interfaces with the AskCodi VS Code extension to provide
 * AI-powered code generation, explanation, and optimization capabilities
 * as an alternative to GitHub Copilot.
 */

export interface AskCodiRequest {
    action: 'generate' | 'explain' | 'optimize' | 'refactor' | 'debug';
    code?: string;
    prompt?: string;
    language?: string;
    context?: string;
    optimizationType?: 'performance' | 'readability' | 'memory' | 'security';
}

export interface AskCodiResponse {
    success: boolean;
    result?: {
        code?: string;
        explanation?: string;
        optimizedCode?: string;
        suggestions?: string[];
        notes?: string;
    };
    error?: string;
    confidence?: number;
    metadata?: {
        model?: string;
        tokens?: number;
        processingTime?: number;
    };
}

export interface AskCodiConfig {
    enabled: boolean;
    apiKey?: string;
    maxResponseLength: number;
    timeout: number;
    enableLogging: boolean;
    preferredModel?: string;
    fallbackToGitHubCopilot: boolean;
}

export class AskCodiBridge {
    private extension: vscode.Extension<any> | undefined;
    private config: AskCodiConfig;
    private outputChannel: vscode.OutputChannel;
    private isInitialized = false;

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        this.config = this.loadConfiguration();
        this.initialize();
    }

    /**
     * Initialize the AskCodi bridge
     */
    private async initialize(): Promise<void> {
        try {
            // Find the AskCodi extension
            this.extension = vscode.extensions.getExtension('AskCodi.askcodi-vscode');

            if (!this.extension) {
                this.outputChannel.appendLine('⚠️ AskCodi extension not found. Install from marketplace for enhanced AI capabilities.');
                return;
            }

            // Activate the extension if not already active
            if (!this.extension.isActive) {
                await this.extension.activate();
                this.outputChannel.appendLine('🚀 AskCodi extension activated');
            }

            // Discover available AskCodi commands
            await this.discoverAskCodiCommands();

            this.isInitialized = true;
            this.outputChannel.appendLine('✅ AskCodi bridge initialized successfully');
        } catch (error) {
            this.outputChannel.appendLine(`❌ Failed to initialize AskCodi bridge: ${error}`);
        }
    }

    /**
     * Check if AskCodi is available and configured
     */
    public isAvailable(): boolean {
        return this.isInitialized && (this.extension?.isActive ?? false) && this.config.enabled;
    }

    /**
     * Generate code using AskCodi
     */
    public async generateCode(request: AskCodiRequest): Promise<AskCodiResponse> {
        if (!this.isAvailable()) {
            return {
                success: false,
                error: 'AskCodi is not available. Please install and configure the AskCodi extension.'
            };
        }

        try {
            const startTime = Date.now();

            // Try different strategies to call AskCodi
            let result = await this.tryExtensionAPI(request);

            if (!result.success) {
                result = await this.tryCommandExecution(request);
            }

            if (!result.success) {
                result = await this.tryWebviewCommunication(request);
            }

            // Add metadata
            if (result.success && result.result) {
                result.metadata = {
                    processingTime: Date.now() - startTime,
                    model: 'askcodi-ai'
                };
            }

            if (this.config.enableLogging) {
                this.outputChannel.appendLine(`🤖 AskCodi ${request.action}: ${result.success ? 'Success' : 'Failed'}`);
            }

            return result;
        } catch (error) {
            return {
                success: false,
                error: `AskCodi API error: ${error instanceof Error ? error.message : String(error)}`
            };
        }
    }

    /**
     * Explain code using AskCodi
     */
    public async explainCode(code: string, language?: string): Promise<AskCodiResponse> {
        return this.generateCode({
            action: 'explain',
            code,
            language: language || 'rust'
        });
    }

    /**
     * Optimize code using AskCodi
     */
    public async optimizeCode(code: string, language?: string, optimizationType?: string): Promise<AskCodiResponse> {
        return this.generateCode({
            action: 'optimize',
            code,
            language: language || 'rust',
            optimizationType: optimizationType as any
        });
    }

    /**
     * Refactor code using AskCodi
     */
    public async refactorCode(code: string, language?: string, context?: string): Promise<AskCodiResponse> {
        return this.generateCode({
            action: 'refactor',
            code,
            language: language || 'rust',
            context: context || ''
        });
    }

    /**
     * Debug code using AskCodi
     */
    public async debugCode(code: string, language?: string, context?: string): Promise<AskCodiResponse> {
        return this.generateCode({
            action: 'debug',
            code,
            language: language || 'rust',
            context: context || ''
        });
    }

    /**
     * Strategy 1: Try to use AskCodi's exported API
     */
    private async tryExtensionAPI(request: AskCodiRequest): Promise<AskCodiResponse> {
        try {
            if (!this.extension?.exports) {
                return { success: false, error: 'No extension API available' };
            }

            const api = this.extension.exports;

            // Try common API method names
            const methodNames = [
                'generateCode',
                'explainCode',
                'optimizeCode',
                'processRequest',
                'askCodi'
            ];

            for (const methodName of methodNames) {
                if (typeof api[methodName] === 'function') {
                    const result = await api[methodName](request);
                    if (result) {
                        const normalizedResult = this.normalizeResult(result);
                        return {
                            success: true,
                            result: normalizedResult || {
                                explanation: 'No result from AskCodi API',
                                suggestions: []
                            },
                            confidence: 0.9
                        };
                    }
                }
            }

            return { success: false, error: 'No compatible API methods found' };
        } catch (error) {
            return { success: false, error: `Extension API error: ${error}` };
        }
    }

    /**
     * Strategy 2: Try to execute AskCodi commands
     */
    private async tryCommandExecution(request: AskCodiRequest): Promise<AskCodiResponse> {
        try {
            // Common AskCodi command patterns
            const commandMappings = {
                generate: ['askcodi.generateCode', 'askcodi.generate', 'askcodi.codeGeneration'],
                explain: ['askcodi.explainCode', 'askcodi.explain', 'askcodi.codeExplanation'],
                optimize: ['askcodi.optimizeCode', 'askcodi.optimize', 'askcodi.codeOptimization'],
                refactor: ['askcodi.refactorCode', 'askcodi.refactor'],
                debug: ['askcodi.debugCode', 'askcodi.debug']
            };

            const commands = commandMappings[request.action] || [];

            for (const command of commands) {
                try {
                    const result = await vscode.commands.executeCommand(command, {
                        prompt: request.prompt,
                        code: request.code,
                        language: request.language,
                        context: request.context,
                    });

                    if (result) {
                        const normalizedResult = this.normalizeResult(result);
                        return {
                            success: true,
                            result: normalizedResult || {
                                explanation: 'No result from AskCodi command',
                                suggestions: []
                            },
                            confidence: 0.85
                        };
                    }
                } catch (commandError) {
                    // Continue to next command
                    continue;
                }
            }

            return { success: false, error: 'No compatible commands found' };
        } catch (error) {
            return { success: false, error: `Command execution error: ${error}` };
        }
    }

    /**
     * Strategy 3: Try webview communication (if AskCodi uses webviews)
     */
    private async tryWebviewCommunication(_request: AskCodiRequest): Promise<AskCodiResponse> {
        try {
            // This would require more specific knowledge of AskCodi's webview implementation
            // For now, return a placeholder
            return { success: false, error: 'Webview communication not implemented yet' };
        } catch (error) {
            return { success: false, error: `Webview communication error: ${error}` };
        }
    }

    /**
     * Normalize different result formats from AskCodi
     */
    private normalizeResult(result: any): AskCodiResponse['result'] {
        if (typeof result === 'string') {
            return { code: result };
        }

        if (result && typeof result === 'object') {
            return {
                code: result.code || result.generatedCode || result.text,
                explanation: result.explanation || result.description,
                optimizedCode: result.optimizedCode || result.optimized,
                suggestions: result.suggestions || result.recommendations,
                notes: result.notes || result.comments
            };
        }

        return {};
    }

    /**
     * Discover available AskCodi commands
     */
    private async discoverAskCodiCommands(): Promise<void> {
        try {
            const allCommands = await vscode.commands.getCommands();
            const askCodiCommands = allCommands.filter(cmd =>
                cmd.toLowerCase().includes('askcodi') ||
                cmd.toLowerCase().includes('ask-codi')
            );

            if (this.config.enableLogging) {
                this.outputChannel.appendLine(`🔍 Discovered AskCodi commands: ${askCodiCommands.join(', ')}`);
            }
        } catch (error) {
            this.outputChannel.appendLine(`⚠️ Failed to discover AskCodi commands: ${error}`);
        }
    }

    /**
     * Load configuration from VS Code settings
     */
    private loadConfiguration(): AskCodiConfig {
        const config = vscode.workspace.getConfiguration('yoshiCopilot.askcodi');

        return {
            enabled: config.get('enabled', true),
            apiKey: config.get('apiKey', ''),
            maxResponseLength: config.get('maxResponseLength', 10000),
            timeout: config.get('timeout', 30000),
            enableLogging: config.get('enableLogging', true),
            preferredModel: config.get('preferredModel', ''),
            fallbackToGitHubCopilot: config.get('fallbackToGitHubCopilot', true)
        };
    }

    /**
     * Update configuration
     */
    public updateConfiguration(newConfig: Partial<AskCodiConfig>): void {
        this.config = { ...this.config, ...newConfig };
    }

    /**
     * Get current configuration
     */
    public getConfiguration(): AskCodiConfig {
        return { ...this.config };
    }

    /**
     * Test AskCodi connection
     */
    public async testConnection(): Promise<{ success: boolean; message: string }> {
        if (!this.extension) {
            return { success: false, message: 'AskCodi extension not found' };
        }

        if (!this.extension.isActive) {
            return { success: false, message: 'AskCodi extension not active' };
        }

        try {
            const testResult = await this.generateCode({
                action: 'generate',
                prompt: 'console.log("Hello, World!");',
                language: 'javascript'
            });

            return {
                success: testResult.success,
                message: testResult.success ? 'AskCodi connection successful' : testResult.error || 'Connection failed'
            };
        } catch (error) {
            return { success: false, message: `Connection test failed: ${error}` };
        }
    }
}
