import * as vscode from 'vscode';

/**
 * GitHub Copilot Bridge for Yoshi Copilot
 *
 * This bridge interfaces with GitHub Copilot to provide AI-powered
 * code generation and assistance capabilities.
 */

export interface GitHubCopilotRequest {
    action: 'generate' | 'explain' | 'optimize' | 'refactor' | 'debug';
    code?: string;
    prompt?: string;
    language?: string;
    context?: string;
    optimizationType?: 'performance' | 'readability' | 'memory' | 'security';
}

export interface GitHubCopilotResponse {
    success: boolean;
    result?: {
        code?: string;
        explanation?: string;
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

export interface GitHubCopilotConfig {
    enabled: boolean;
    maxResponseLength: number;
    timeout: number;
    enableLogging: boolean;
    useInlineCompletions: boolean;
    useChatAPI: boolean;
}

export class GitHubCopilotBridge {
    private extension: vscode.Extension<any> | undefined;
    private config: GitHubCopilotConfig;
    private outputChannel: vscode.OutputChannel;
    private isInitialized = false;

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        this.config = this.loadConfiguration();
        this.initialize();
    }

    /**
     * Initialize the GitHub Copilot bridge
     */
    private async initialize(): Promise<void> {
        try {
            // Find the GitHub Copilot extension
            this.extension = vscode.extensions.getExtension('github.copilot');

            if (!this.extension) {
                this.outputChannel.appendLine('⚠️ GitHub Copilot extension not found. Install from marketplace for AI assistance.');
                return;
            }

            // Activate the extension if not already active
            if (!this.extension.isActive) {
                await this.extension.activate();
                this.outputChannel.appendLine('🚀 GitHub Copilot extension activated');
            }

            // Discover available GitHub Copilot commands
            await this.discoverCopilotCommands();

            this.isInitialized = true;
            this.outputChannel.appendLine('✅ GitHub Copilot bridge initialized successfully');
        } catch (error) {
            this.outputChannel.appendLine(`❌ Failed to initialize GitHub Copilot bridge: ${error}`);
        }
    }

    /**
     * Check if GitHub Copilot is available and configured
     */
    public isAvailable(): boolean {
        return this.isInitialized && (this.extension?.isActive ?? false) && this.config.enabled;
    }

    /**
     * Process request using GitHub Copilot
     */
    public async processRequest(request: GitHubCopilotRequest): Promise<GitHubCopilotResponse> {
        if (!this.isAvailable()) {
            return {
                success: false,
                error: 'GitHub Copilot is not available. Please install and configure the GitHub Copilot extension.'
            };
        }

        try {
            const startTime = Date.now();

            // Try different strategies to call GitHub Copilot
            let result = await this.tryInlineCompletions(request);

            if (!result.success && this.config.useChatAPI) {
                result = await this.tryChatAPI(request);
            }

            if (!result.success) {
                result = await this.tryCommandExecution(request);
            }

            // Add metadata
            if (result.success && result.result) {
                result.metadata = {
                    processingTime: Date.now() - startTime,
                    model: 'github-copilot'
                };
            }

            if (this.config.enableLogging) {
                this.outputChannel.appendLine(`🤖 GitHub Copilot ${request.action}: ${result.success ? 'Success' : 'Failed'}`);
            }

            return result;
        } catch (error) {
            return {
                success: false,
                error: `GitHub Copilot API error: ${error instanceof Error ? error.message : String(error)}`
            };
        }
    }

    /**
     * Strategy 1: Try to use GitHub Copilot's inline completions
     */
    private async tryInlineCompletions(request: GitHubCopilotRequest): Promise<GitHubCopilotResponse> {
        try {
            if (!this.config.useInlineCompletions) {
                return { success: false, error: 'Inline completions disabled' };
            }

            // Create a temporary document for inline completions
            const document = await this.createTempDocument(request);
            const position = new vscode.Position(document.lineCount - 1, 0);

            // Request inline completions
            const completions = await vscode.commands.executeCommand<vscode.InlineCompletionList>(
                'vscode.executeInlineCompletionProvider',
                document.uri,
                position,
                {
                    triggerKind: vscode.InlineCompletionTriggerKind.Invoke,
                    selectedCompletionInfo: undefined
                }
            );

            if (completions && completions.items.length > 0) {
                const completion = completions.items[0];
                if (completion) {
                    const insertText = typeof completion.insertText === 'string'
                        ? completion.insertText
                        : completion.insertText?.value || '';

                    return {
                        success: true,
                        result: {
                            code: insertText,
                            suggestions: ['Generated using GitHub Copilot inline completions']
                        },
                        confidence: 0.85
                    };
                }
            }

            return { success: false, error: 'No inline completions available' };
        } catch (error) {
            return { success: false, error: `Inline completions error: ${error}` };
        }
    }

    /**
     * Strategy 2: Try to use GitHub Copilot Chat API
     */
    private async tryChatAPI(request: GitHubCopilotRequest): Promise<GitHubCopilotResponse> {
        try {
            // Check if Copilot Chat is available
            const chatExtension = vscode.extensions.getExtension('github.copilot-chat');
            if (!chatExtension?.isActive) {
                return { success: false, error: 'GitHub Copilot Chat not available' };
            }

            // Try to use the chat API
            const chatPrompt = this.buildChatPrompt(request);

            // Execute chat command (this is a simplified approach)
            const result = await vscode.commands.executeCommand('github.copilot.chat.ask', chatPrompt);

            if (result) {
                const parsedResult = this.parseChatResponse(result, request);
                return {
                    success: true,
                    result: parsedResult || {
                        explanation: 'No result from GitHub Copilot Chat',
                        suggestions: []
                    },
                    confidence: 0.90
                };
            }

            return { success: false, error: 'Chat API returned no result' };
        } catch (error) {
            return { success: false, error: `Chat API error: ${error}` };
        }
    }

    /**
     * Strategy 3: Try to execute GitHub Copilot commands
     */
    private async tryCommandExecution(request: GitHubCopilotRequest): Promise<GitHubCopilotResponse> {
        try {
            // Common GitHub Copilot command patterns
            const commandMappings = {
                generate: ['github.copilot.generate', 'github.copilot.generateCode'],
                explain: ['github.copilot.explain', 'github.copilot.explainCode'],
                optimize: ['github.copilot.optimize', 'github.copilot.optimizeCode'],
                refactor: ['github.copilot.refactor', 'github.copilot.refactorCode'],
                debug: ['github.copilot.debug', 'github.copilot.debugCode']
            };

            const commands = commandMappings[request.action] || [];

            for (const command of commands) {
                try {
                    const result = await vscode.commands.executeCommand(command, {
                        prompt: request.prompt,
                        code: request.code,
                        language: request.language,
                        context: request.context
                    });

                    if (result) {
                        const normalizedResult = this.normalizeResult(result);
                        return {
                            success: true,
                            result: normalizedResult || {
                                explanation: 'No result from GitHub Copilot command',
                                suggestions: []
                            },
                            confidence: 0.80
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
     * Create temporary document for inline completions
     */
    private async createTempDocument(request: GitHubCopilotRequest): Promise<vscode.TextDocument> {
        const content = this.buildDocumentContent(request);
        const document = await vscode.workspace.openTextDocument({
            content,
            language: request.language || 'rust'
        });
        return document;
    }

    /**
     * Build document content for inline completions
     */
    private buildDocumentContent(request: GitHubCopilotRequest): string {
        let content = '';

        if (request.context) {
            content += `// Context: ${request.context}\n`;
        }

        if (request.prompt) {
            content += `// ${request.prompt}\n`;
        }

        if (request.code) {
            content += request.code + '\n';
        }

        // Add a trigger for completion
        content += '\n// TODO: ';

        return content;
    }

    /**
     * Build chat prompt for Copilot Chat
     */
    private buildChatPrompt(request: GitHubCopilotRequest): string {
        let prompt = '';

        switch (request.action) {
            case 'generate':
                prompt = `Generate ${request.language || 'Rust'} code for: ${request.prompt}`;
                break;
            case 'explain':
                prompt = `Explain this ${request.language || 'Rust'} code:\n\`\`\`\n${request.code}\n\`\`\``;
                break;
            case 'optimize':
                prompt = `Optimize this ${request.language || 'Rust'} code for performance:\n\`\`\`\n${request.code}\n\`\`\``;
                break;
            case 'refactor':
                prompt = `Refactor this ${request.language || 'Rust'} code for better readability:\n\`\`\`\n${request.code}\n\`\`\``;
                break;
            case 'debug':
                prompt = `Help debug this ${request.language || 'Rust'} code:\n\`\`\`\n${request.code}\n\`\`\``;
                break;
            default:
                prompt = request.prompt || 'Help with this code';
        }

        if (request.context) {
            prompt += `\n\nContext: ${request.context}`;
        }

        return prompt;
    }

    /**
     * Parse chat response from Copilot Chat
     */
    private parseChatResponse(response: any, _request: GitHubCopilotRequest): GitHubCopilotResponse['result'] {
        if (typeof response === 'string') {
            // Extract code blocks from markdown response
            const codeBlockRegex = /```(?:\w+)?\n([\s\S]*?)\n```/g;
            const matches = response.match(codeBlockRegex);

            if (matches && matches.length > 0) {
                const code = matches[0].replace(/```(?:\w+)?\n/, '').replace(/\n```$/, '');
                return {
                    code,
                    explanation: response,
                    suggestions: ['Generated using GitHub Copilot Chat']
                };
            }

            return {
                explanation: response,
                suggestions: ['Response from GitHub Copilot Chat']
            };
        }

        return this.normalizeResult(response);
    }

    /**
     * Normalize different result formats from GitHub Copilot
     */
    private normalizeResult(result: any): GitHubCopilotResponse['result'] {
        if (typeof result === 'string') {
            return { code: result };
        }

        if (result && typeof result === 'object') {
            return {
                code: result.code || result.generatedCode || result.text,
                explanation: result.explanation || result.description,
                suggestions: result.suggestions || result.recommendations,
                notes: result.notes || result.comments
            };
        }

        return {};
    }

    /**
     * Discover available GitHub Copilot commands
     */
    private async discoverCopilotCommands(): Promise<void> {
        try {
            const allCommands = await vscode.commands.getCommands();
            const copilotCommands = allCommands.filter(cmd =>
                cmd.toLowerCase().includes('copilot') ||
                cmd.toLowerCase().includes('github.copilot')
            );

            if (this.config.enableLogging) {
                this.outputChannel.appendLine(`🔍 Discovered GitHub Copilot commands: ${copilotCommands.join(', ')}`);
            }
        } catch (error) {
            this.outputChannel.appendLine(`⚠️ Failed to discover GitHub Copilot commands: ${error}`);
        }
    }

    /**
     * Load configuration from VS Code settings
     */
    private loadConfiguration(): GitHubCopilotConfig {
        const config = vscode.workspace.getConfiguration('yoshiCopilot.githubCopilot');

        return {
            enabled: config.get('enabled', true),
            maxResponseLength: config.get('maxResponseLength', 10000),
            timeout: config.get('timeout', 30000),
            enableLogging: config.get('enableLogging', true),
            useInlineCompletions: config.get('useInlineCompletions', true),
            useChatAPI: config.get('useChatAPI', true)
        };
    }

    /**
     * Update configuration
     */
    public updateConfiguration(newConfig: Partial<GitHubCopilotConfig>): void {
        this.config = { ...this.config, ...newConfig };
    }

    /**
     * Get current configuration
     */
    public getConfiguration(): GitHubCopilotConfig {
        return { ...this.config };
    }

    /**
     * Test GitHub Copilot connection
     */
    public async testConnection(): Promise<{ success: boolean; message: string }> {
        if (!this.extension) {
            return { success: false, message: 'GitHub Copilot extension not found' };
        }

        if (!this.extension.isActive) {
            return { success: false, message: 'GitHub Copilot extension not active' };
        }

        try {
            const testResult = await this.processRequest({
                action: 'generate',
                prompt: 'console.log("Hello, World!");',
                language: 'javascript'
            });

            return {
                success: testResult.success,
                message: testResult.success ? 'GitHub Copilot connection successful' : testResult.error || 'Connection failed'
            };
        } catch (error) {
            return { success: false, message: `Connection test failed: ${error}` };
        }
    }
}
