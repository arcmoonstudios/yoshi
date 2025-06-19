/**
 * # Yoshi MCP VS Code Extension
 *
 * Main extension entry point providing local LLM inference capabilities
 * with complete Yoshi error handling integration.
 */

import * as vscode from 'vscode';
import { McpServerManager } from './mcp/server';
import { ModelManager } from './model/manager';
import { SecurityManager } from './security/manager';
import { ConfigurationManager } from './config/manager';

// Import WASM bindings - these will be available after build
// For development, we'll handle the case where WASM isn't built yet
let WasmInferenceEngine: any;
let WasmPerformanceMonitor: any;
let WasmUtils: any;

try {
    // Try to import the actual WASM bindings
    const wasmModule = require('../dist/wasm/yoshi_mcp');
    WasmInferenceEngine = wasmModule.WasmInferenceEngine;
    WasmPerformanceMonitor = wasmModule.WasmPerformanceMonitor;
    WasmUtils = wasmModule.WasmUtils;
} catch (error) {
    // Fallback to mock implementations if WASM isn't built yet
    console.warn('WASM module not found, using mock implementations:', error);

    // Mock implementations for development
    class MockWasmInferenceEngine {
        private config: string = '{}';
        private modelLoaded: boolean = false;

        constructor(config?: string) {
            if (config) {
                this.config = config;
            }
        }

        async init(): Promise<void> {
            await new Promise(resolve => setTimeout(resolve, 100));
        }

        async load_model(_modelPath: string): Promise<void> {
            await new Promise(resolve => setTimeout(resolve, 500));
            this.modelLoaded = true;
        }

        async is_model_loaded(): Promise<boolean> {
            return this.modelLoaded;
        }

        async generate_text(prompt: string, maxTokens: number): Promise<string> {
            await new Promise(resolve => setTimeout(resolve, 1000));
            return `Mock response for: "${prompt.substring(0, 50)}..." (${maxTokens} max tokens)`;
        }

        async update_config(config: string): Promise<void> {
            this.config = config;
        }

        get_config(): string {
            return this.config;
        }

        async get_metrics(): Promise<string> {
            return JSON.stringify({
                inference_count: 0,
                average_latency_ms: 0,
                total_tokens_generated: 0,
                memory_usage_mb: 0
            });
        }
    }

    class MockWasmPerformanceMonitor {
        private timings: Map<string, number> = new Map();

        start_timing(operation: string): void {
            this.timings.set(`${operation}_start`, Date.now());
        }

        end_timing(operation: string): number {
            const startTime = this.timings.get(`${operation}_start`);
            if (!startTime) {
                return 0;
            }
            const duration = Date.now() - startTime;
            this.timings.set(`${operation}_duration`, duration);
            return duration;
        }
    }

    class MockWasmUtils {
        static validate_config(config: string): void {
            try {
                const parsed = JSON.parse(config);
                if (typeof parsed !== 'object') {
                    throw new Error('Config must be an object');
                }
            } catch (error) {
                throw new Error(`Invalid JSON config: ${error}`);
            }
        }

        static log(message: string): void {
            console.log(`[WASM] ${message}`);
        }

        static error(message: string): void {
            console.error(`[WASM ERROR] ${message}`);
        }

        static get_system_info(): string {
            return JSON.stringify({
                platform: 'mock',
                arch: 'wasm32',
                memory_mb: 1024,
                cpu_cores: 4
            });
        }

        static estimate_memory_usage(contextLength: number, maxTokens: number): number {
            return (contextLength + maxTokens) * 4;
        }
    }

    WasmInferenceEngine = MockWasmInferenceEngine;
    WasmPerformanceMonitor = MockWasmPerformanceMonitor;
    WasmUtils = MockWasmUtils;
}

/**
 * Extension configuration interface
 */
interface ExtensionConfig {
    modelPath: string;
    maxTokens: number;
    temperature: number;
    enableGPU: boolean;
    cacheSize: number;
    autoDownloadModels: boolean;
}

/**
 * Main extension class managing all components
 */
export class YoshiMcpExtension {
    private wasmEngine: any | null = null;
    private mcpServer: McpServerManager | null = null;
    private modelManager: ModelManager | null = null;
    private securityManager: SecurityManager | null = null;
    private configManager: ConfigurationManager | null = null;
    private performanceMonitor: any | null = null;
    private statusBarItem: vscode.StatusBarItem | null = null;

    /**
     * Activate the extension
     */
    async activate(context: vscode.ExtensionContext): Promise<void> {
        try {
            // Initialize performance monitoring
            this.performanceMonitor = new WasmPerformanceMonitor();
            this.performanceMonitor.start_timing('extension_activation');

            // Initialize configuration manager
            this.configManager = new ConfigurationManager();
            const config = this.configManager.getConfiguration();

            // Initialize security manager
            this.securityManager = new SecurityManager();

            // Initialize model manager
            this.modelManager = new ModelManager(context.globalStorageUri.fsPath);

            // Initialize WASM inference engine
            await this.initializeWasmEngine(config);

            // Initialize MCP server
            this.mcpServer = new McpServerManager(this.wasmEngine!);
            await this.mcpServer.start();

            // Register commands
            this.registerCommands(context);

            // Register providers
            this.registerProviders(context);

            // Create status bar item
            this.createStatusBarItem(context);

            // Set up configuration change listener
            this.setupConfigurationListener();

            const activationTime = this.performanceMonitor.end_timing('extension_activation');

            vscode.window.showInformationMessage(
                `Yoshi MCP Extension activated in ${activationTime.toFixed(0)}ms`
            );

            WasmUtils.log(`Extension activated successfully in ${activationTime}ms`);

        } catch (error) {
            const errorMessage = `Failed to activate Yoshi MCP Extension: ${error}`;
            vscode.window.showErrorMessage(errorMessage);
            WasmUtils.error(errorMessage);
            throw error;
        }
    }

    /**
     * Initialize the WASM inference engine
     */
    private async initializeWasmEngine(config: ExtensionConfig): Promise<void> {
        const configJson = JSON.stringify({
            model_path: config.modelPath || null,
            max_tokens: config.maxTokens,
            temperature: config.temperature,
            enable_gpu: config.enableGPU,
            cache_size: config.cacheSize,
            auto_download_models: config.autoDownloadModels
        });

        // Validate configuration
        try {
            WasmUtils.validate_config(configJson);
        } catch (error) {
            throw new Error(`Invalid configuration: ${error}`);
        }

        // Create and initialize WASM engine
        this.wasmEngine = new WasmInferenceEngine(configJson);
        await this.wasmEngine.init();

        // Load model if specified
        if (config.modelPath) {
            try {
                await this.wasmEngine.load_model(config.modelPath);
                WasmUtils.log(`Model loaded: ${config.modelPath}`);
            } catch (error) {
                WasmUtils.error(`Failed to load model: ${error}`);
                vscode.window.showWarningMessage(`Failed to load model: ${error}`);
            }
        }
    }

    /**
     * Register extension commands
     */
    private registerCommands(context: vscode.ExtensionContext): void {
        // Generate text command
        const generateTextCommand = vscode.commands.registerCommand(
            'yoshiMcp.generateText',
            async () => {
                await this.handleGenerateText();
            }
        );

        // Select model command
        const selectModelCommand = vscode.commands.registerCommand(
            'yoshiMcp.selectModel',
            async () => {
                await this.handleSelectModel();
            }
        );

        // Download model command
        const downloadModelCommand = vscode.commands.registerCommand(
            'yoshiMcp.downloadModel',
            async () => {
                await this.handleDownloadModel();
            }
        );

        // Show status command
        const showStatusCommand = vscode.commands.registerCommand(
            'yoshiMcp.showStatus',
            async () => {
                await this.handleShowStatus();
            }
        );

        context.subscriptions.push(
            generateTextCommand,
            selectModelCommand,
            downloadModelCommand,
            showStatusCommand
        );
    }

    /**
     * Register language providers
     */
    private registerProviders(context: vscode.ExtensionContext): void {
        // Completion provider
        const completionProvider = vscode.languages.registerCompletionItemProvider(
            { scheme: 'file' },
            new YoshiMcpCompletionProvider(this.wasmEngine!),
            '.'
        );

        // Hover provider
        const hoverProvider = vscode.languages.registerHoverProvider(
            { scheme: 'file' },
            new YoshiMcpHoverProvider(this.wasmEngine!)
        );

        context.subscriptions.push(completionProvider, hoverProvider);
    }

    /**
     * Create status bar item
     */
    private createStatusBarItem(context: vscode.ExtensionContext): void {
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Right,
            100
        );

        this.statusBarItem.command = 'yoshiMcp.showStatus';
        this.updateStatusBarItem();
        this.statusBarItem.show();

        context.subscriptions.push(this.statusBarItem);
    }

    /**
     * Update status bar item
     */
    private async updateStatusBarItem(): Promise<void> {
        if (!this.statusBarItem || !this.wasmEngine) {
            return;
        }

        try {
            const isModelLoaded = await this.wasmEngine.is_model_loaded();

            if (isModelLoaded) {
                this.statusBarItem.text = "$(check) Yoshi MCP Ready";
                this.statusBarItem.tooltip = "Yoshi MCP Extension - Model loaded and ready";
            } else {
                this.statusBarItem.text = "$(warning) Yoshi MCP No Model";
                this.statusBarItem.tooltip = "Yoshi MCP Extension - No model loaded";
            }
        } catch (error) {
            this.statusBarItem.text = "$(error) Yoshi MCP Error";
            this.statusBarItem.tooltip = `Yoshi MCP Extension - Error: ${error}`;
        }
    }

    /**
     * Setup configuration change listener
     */
    private setupConfigurationListener(): void {
        vscode.workspace.onDidChangeConfiguration(async (event: vscode.ConfigurationChangeEvent) => {
            if (event.affectsConfiguration('yoshiMcp')) {
                const config = this.configManager!.getConfiguration();

                try {
                    // Update WASM engine configuration
                    const configJson = JSON.stringify({
                        model_path: config.modelPath || null,
                        max_tokens: config.maxTokens,
                        temperature: config.temperature,
                        enable_gpu: config.enableGPU,
                        cache_size: config.cacheSize,
                        auto_download_models: config.autoDownloadModels
                    });

                    await this.wasmEngine!.update_config(configJson);
                    await this.updateStatusBarItem();

                    WasmUtils.log('Configuration updated');
                } catch (error) {
                    WasmUtils.error(`Failed to update configuration: ${error}`);
                    vscode.window.showErrorMessage(`Failed to update configuration: ${error}`);
                }
            }
        });
    }

    /**
     * Handle generate text command
     */
    private async handleGenerateText(): Promise<void> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('No active editor');
            return;
        }

        if (!this.wasmEngine) {
            vscode.window.showErrorMessage('Inference engine not initialized');
            return;
        }

        const selection = editor.selection;
        const selectedText = editor.document.getText(selection);

        if (!selectedText.trim()) {
            vscode.window.showWarningMessage('Please select some text to use as a prompt');
            return;
        }

        try {
            // Validate input with security manager
            this.securityManager!.validateInput(selectedText);

            // Show progress
            await vscode.window.withProgress({
                location: vscode.ProgressLocation.Notification,
                title: "Generating text with Yoshi MCP...",
                cancellable: false
            }, async (progress: vscode.Progress<{ message?: string; increment?: number }>) => {
                progress.report({ increment: 0 });

                const config = this.configManager!.getConfiguration();
                const result = await this.wasmEngine!.generate_text(selectedText, config.maxTokens);

                progress.report({ increment: 100 });

                // Insert generated text
                await editor.edit((editBuilder: vscode.TextEditorEdit) => {
                    editBuilder.replace(selection, result);
                });

                vscode.window.showInformationMessage('Text generated successfully');
            });

        } catch (error) {
            const errorMessage = `Text generation failed: ${error}`;
            vscode.window.showErrorMessage(errorMessage);
            WasmUtils.error(errorMessage);
        }
    }

    /**
     * Handle select model command
     */
    private async handleSelectModel(): Promise<void> {
        if (!this.modelManager) {
            vscode.window.showErrorMessage('Model manager not initialized');
            return;
        }

        try {
            const models = await this.modelManager.listAvailableModels();

            if (models.length === 0) {
                vscode.window.showInformationMessage('No models available. Use "Download Model" to get started.');
                return;
            }

            const selectedModel = await vscode.window.showQuickPick(
                models.map(model => ({
                    label: model.name,
                    description: model.description,
                    detail: `Size: ${model.size}, Type: ${model.type}`,
                    model: model
                })),
                {
                    placeHolder: 'Select a model to load'
                }
            );

            if (selectedModel) {
                await vscode.window.withProgress({
                    location: vscode.ProgressLocation.Notification,
                    title: `Loading model: ${selectedModel.model.name}...`,
                    cancellable: false
                }, async () => {
                    await this.wasmEngine!.load_model(selectedModel.model.path);
                    await this.updateStatusBarItem();
                });

                vscode.window.showInformationMessage(`Model loaded: ${selectedModel.model.name}`);
            }

        } catch (error) {
            const errorMessage = `Failed to select model: ${error}`;
            vscode.window.showErrorMessage(errorMessage);
            WasmUtils.error(errorMessage);
        }
    }

    /**
     * Handle download model command
     */
    private async handleDownloadModel(): Promise<void> {
        // Implementation would go here
        vscode.window.showInformationMessage('Model download feature coming soon!');
    }

    /**
     * Handle show status command
     */
    private async handleShowStatus(): Promise<void> {
        if (!this.wasmEngine || !this.performanceMonitor) {
            vscode.window.showErrorMessage('Extension not fully initialized');
            return;
        }

        try {
            const isModelLoaded = await this.wasmEngine.is_model_loaded();
            const metrics = JSON.parse(await this.wasmEngine.get_metrics());
            const systemInfo = WasmUtils.get_system_info();

            const statusInfo = {
                modelLoaded: isModelLoaded,
                metrics: metrics,
                systemInfo: JSON.parse(systemInfo)
            };

            // Show status in a new document
            const doc = await vscode.workspace.openTextDocument({
                content: JSON.stringify(statusInfo, null, 2),
                language: 'json'
            });

            await vscode.window.showTextDocument(doc);

        } catch (error) {
            const errorMessage = `Failed to get status: ${error}`;
            vscode.window.showErrorMessage(errorMessage);
            WasmUtils.error(errorMessage);
        }
    }

    /**
     * Deactivate the extension
     */
    deactivate(): void {
        this.mcpServer?.stop();
        this.statusBarItem?.dispose();
        WasmUtils.log('Extension deactivated');
    }
}

// Completion provider implementation
class YoshiMcpCompletionProvider implements vscode.CompletionItemProvider {
    constructor(private _wasmEngine: any) {}

    async provideCompletionItems(
        _document: vscode.TextDocument,
        _position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        // Implementation would provide AI-powered completions
        return [];
    }
}

// Hover provider implementation
class YoshiMcpHoverProvider implements vscode.HoverProvider {
    constructor(private _wasmEngine: any) {}

    async provideHover(
        _document: vscode.TextDocument,
        _position: vscode.Position
    ): Promise<vscode.Hover | null> {
        // Implementation would provide AI-powered hover information
        return null;
    }
}

// Extension activation/deactivation functions
let extension: YoshiMcpExtension;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    extension = new YoshiMcpExtension();
    await extension.activate(context);
}

export function deactivate(): void {
    extension?.deactivate();
}
