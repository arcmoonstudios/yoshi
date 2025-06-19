/**
 * # Configuration Manager for Yoshi MCP
 * 
 * Manages VS Code configuration settings for the Yoshi MCP extension
 * with validation and type safety.
 */

import * as vscode from 'vscode';

/**
 * Extension configuration interface
 */
export interface YoshiMcpConfiguration {
    modelPath: string;
    maxTokens: number;
    temperature: number;
    enableGPU: boolean;
    cacheSize: number;
    autoDownloadModels: boolean;
}

/**
 * Configuration manager class
 */
export class ConfigurationManager {
    private static readonly CONFIG_SECTION = 'yoshiMcp';

    /**
     * Get current configuration
     */
    getConfiguration(): YoshiMcpConfiguration {
        const config = vscode.workspace.getConfiguration(ConfigurationManager.CONFIG_SECTION);

        return {
            modelPath: config.get<string>('modelPath', ''),
            maxTokens: config.get<number>('maxTokens', 200),
            temperature: config.get<number>('temperature', 0.7),
            enableGPU: config.get<boolean>('enableGPU', true),
            cacheSize: config.get<number>('cacheSize', 100),
            autoDownloadModels: config.get<boolean>('autoDownloadModels', false)
        };
    }

    /**
     * Update configuration value
     */
    async updateConfiguration<K extends keyof YoshiMcpConfiguration>(
        key: K,
        value: YoshiMcpConfiguration[K],
        target: vscode.ConfigurationTarget = vscode.ConfigurationTarget.Global
    ): Promise<void> {
        const config = vscode.workspace.getConfiguration(ConfigurationManager.CONFIG_SECTION);
        await config.update(key, value, target);
    }

    /**
     * Validate configuration
     */
    validateConfiguration(config: YoshiMcpConfiguration): string[] {
        const errors: string[] = [];

        if (config.maxTokens <= 0 || config.maxTokens > 4096) {
            errors.push('maxTokens must be between 1 and 4096');
        }

        if (config.temperature < 0.0 || config.temperature > 2.0) {
            errors.push('temperature must be between 0.0 and 2.0');
        }

        if (config.cacheSize <= 0 || config.cacheSize > 1000) {
            errors.push('cacheSize must be between 1 and 1000');
        }

        if (config.modelPath && !this.isValidModelPath(config.modelPath)) {
            errors.push('modelPath must be a valid file path ending with .gguf');
        }

        return errors;
    }

    /**
     * Check if model path is valid
     */
    private isValidModelPath(path: string): boolean {
        return path.endsWith('.gguf') && path.length > 5;
    }

    /**
     * Get default configuration
     */
    getDefaultConfiguration(): YoshiMcpConfiguration {
        return {
            modelPath: '',
            maxTokens: 200,
            temperature: 0.7,
            enableGPU: true,
            cacheSize: 100,
            autoDownloadModels: false
        };
    }

    /**
     * Reset configuration to defaults
     */
    async resetToDefaults(): Promise<void> {
        const defaults = this.getDefaultConfiguration();
        const config = vscode.workspace.getConfiguration(ConfigurationManager.CONFIG_SECTION);

        for (const [key, value] of Object.entries(defaults)) {
            await config.update(key, value, vscode.ConfigurationTarget.Global);
        }
    }
}
