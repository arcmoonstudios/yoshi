/**
 * # Model Manager for Yoshi MCP
 *
 * Manages local model files, downloads, and metadata
 * with integration to the Rust model management system.
 */

import * as fs from 'fs';
import * as path from 'path';

/**
 * Model information interface
 */
export interface ModelInfo {
    id: string;
    name: string;
    path: string;
    size: string;
    type: string;
    description: string;
    architecture: string;
    parameters: string;
    quantization: string;
}

/**
 * Model download progress interface
 */
export interface DownloadProgress {
    downloaded: number;
    total: number;
    percentage: number;
    speed: string;
}

/**
 * Model manager class
 */
export class ModelManager {
    private modelsDirectory: string;
    private modelCache: Map<string, ModelInfo> = new Map();

    constructor(storageDirectory: string) {
        this.modelsDirectory = path.join(storageDirectory, 'models');
        this.ensureModelsDirectory();
        this.refreshModelCache();
    }

    /**
     * Ensure models directory exists
     */
    private ensureModelsDirectory(): void {
        if (!fs.existsSync(this.modelsDirectory)) {
            fs.mkdirSync(this.modelsDirectory, { recursive: true });
        }
    }

    /**
     * Refresh model cache from filesystem
     */
    private refreshModelCache(): void {
        this.modelCache.clear();

        try {
            const files = fs.readdirSync(this.modelsDirectory);

            for (const file of files) {
                if (file.endsWith('.gguf')) {
                    const modelInfo = this.analyzeModelFile(file);
                    if (modelInfo) {
                        this.modelCache.set(modelInfo.id, modelInfo);
                    }
                }
            }
        } catch (error) {
            console.error('Failed to refresh model cache:', error);
        }
    }

    /**
     * Analyze a model file and extract metadata
     */
    private analyzeModelFile(filename: string): ModelInfo | null {
        try {
            const filePath = path.join(this.modelsDirectory, filename);
            const stats = fs.statSync(filePath);

            // Parse model information from filename
            const { name, architecture, quantization } = this.parseModelFilename(filename);

            return {
                id: filename,
                name,
                path: filePath,
                size: this.formatFileSize(stats.size),
                type: 'GGUF',
                description: `${architecture} model with ${quantization} quantization`,
                architecture,
                parameters: 'Unknown', // Would be extracted from GGUF header in production
                quantization
            };
        } catch (error) {
            console.error(`Failed to analyze model file ${filename}:`, error);
            return null;
        }
    }

    /**
     * Parse model information from filename
     */
    private parseModelFilename(filename: string): { name: string; architecture: string; quantization: string } {
        const baseName = filename.replace('.gguf', '');

        let architecture = 'Unknown';
        if (baseName.toLowerCase().includes('qwen')) {
            architecture = 'Qwen';
        } else if (baseName.toLowerCase().includes('llama')) {
            architecture = 'Llama';
        } else if (baseName.toLowerCase().includes('mistral')) {
            architecture = 'Mistral';
        }

        let quantization = 'Unknown';
        if (baseName.includes('q4_0')) {
            quantization = 'Q4_0';
        } else if (baseName.includes('q8_0')) {
            quantization = 'Q8_0';
        } else if (baseName.includes('q5_1')) {
            quantization = 'Q5_1';
        }

        return {
            name: baseName,
            architecture,
            quantization
        };
    }

    /**
     * Format file size for display
     */
    private formatFileSize(bytes: number): string {
        const units = ['B', 'KB', 'MB', 'GB', 'TB'];
        let size = bytes;
        let unitIndex = 0;

        while (size >= 1024 && unitIndex < units.length - 1) {
            size /= 1024;
            unitIndex++;
        }

        return `${size.toFixed(1)} ${units[unitIndex]}`;
    }

    /**
     * List available models
     */
    async listAvailableModels(): Promise<ModelInfo[]> {
        this.refreshModelCache();
        return Array.from(this.modelCache.values());
    }

    /**
     * Get model by ID
     */
    getModel(modelId: string): ModelInfo | undefined {
        return this.modelCache.get(modelId);
    }

    /**
     * Check if model exists
     */
    hasModel(modelId: string): boolean {
        return this.modelCache.has(modelId);
    }

    /**
     * Delete a model
     */
    async deleteModel(modelId: string): Promise<void> {
        const model = this.modelCache.get(modelId);
        if (!model) {
            throw new Error(`Model not found: ${modelId}`);
        }

        try {
            fs.unlinkSync(model.path);
            this.modelCache.delete(modelId);
        } catch (error) {
            throw new Error(`Failed to delete model: ${error}`);
        }
    }

    /**
     * Get recommended models list
     */
    getRecommendedModels(): Array<{
        id: string;
        name: string;
        description: string;
        size: string;
        downloadUrl: string;
        huggingFaceRepo: string;
    }> {
        return [
            {
                id: 'qwen2.5-7b-instruct-q4_0',
                name: 'Qwen2.5 7B Instruct Q4_0',
                description: 'High-quality instruction-following model, good balance of size and performance',
                size: '4.2 GB',
                downloadUrl: 'https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF/resolve/main/qwen2.5-7b-instruct-q4_0.gguf',
                huggingFaceRepo: 'Qwen/Qwen2.5-7B-Instruct-GGUF'
            },
            {
                id: 'llama-3.2-3b-instruct-q4_0',
                name: 'Llama 3.2 3B Instruct Q4_0',
                description: 'Smaller, faster model suitable for quick responses',
                size: '1.9 GB',
                downloadUrl: 'https://huggingface.co/meta-llama/Llama-3.2-3B-Instruct-GGUF/resolve/main/llama-3.2-3b-instruct-q4_0.gguf',
                huggingFaceRepo: 'meta-llama/Llama-3.2-3B-Instruct-GGUF'
            },
            {
                id: 'mistral-7b-instruct-q4_0',
                name: 'Mistral 7B Instruct Q4_0',
                description: 'Excellent code generation and reasoning capabilities',
                size: '4.1 GB',
                downloadUrl: 'https://huggingface.co/mistralai/Mistral-7B-Instruct-v0.3-GGUF/resolve/main/mistral-7b-instruct-q4_0.gguf',
                huggingFaceRepo: 'mistralai/Mistral-7B-Instruct-v0.3-GGUF'
            }
        ];
    }

    /**
     * Download a model with progress tracking
     */
    async downloadModel(
        modelId: string,
        downloadUrl: string,
        progressCallback?: (progress: DownloadProgress) => void
    ): Promise<string> {
        const filename = `${modelId}.gguf`;
        const filePath = path.join(this.modelsDirectory, filename);

        // Check if model already exists
        if (fs.existsSync(filePath)) {
            throw new Error(`Model already exists: ${modelId}`);
        }

        try {
            // This would be implemented with proper HTTP download logic
            // For now, just simulate the download
            if (progressCallback) {
                for (let i = 0; i <= 100; i += 10) {
                    progressCallback({
                        downloaded: i * 1024 * 1024,
                        total: 100 * 1024 * 1024,
                        percentage: i,
                        speed: '10 MB/s'
                    });
                    await new Promise(resolve => setTimeout(resolve, 100));
                }
            }

            // Create a placeholder file for demonstration
            fs.writeFileSync(filePath, 'placeholder model data');

            // Refresh cache
            this.refreshModelCache();

            return filePath;
        } catch (error) {
            // Clean up partial download
            if (fs.existsSync(filePath)) {
                fs.unlinkSync(filePath);
            }
            throw new Error(`Failed to download model: ${error}`);
        }
    }

    /**
     * Verify model integrity
     */
    async verifyModel(modelId: string): Promise<boolean> {
        const model = this.modelCache.get(modelId);
        if (!model) {
            return false;
        }

        try {
            // Check if file exists and is readable
            fs.accessSync(model.path, fs.constants.R_OK);

            // In production, this would verify the GGUF format and checksum
            return true;
        } catch (error) {
            return false;
        }
    }

    /**
     * Get models directory path
     */
    getModelsDirectory(): string {
        return this.modelsDirectory;
    }

    /**
     * Get total models size
     */
    getTotalModelsSize(): string {
        let totalBytes = 0;

        for (const model of this.modelCache.values()) {
            try {
                const stats = fs.statSync(model.path);
                totalBytes += stats.size;
            } catch (error) {
                // Ignore errors for individual files
            }
        }

        return this.formatFileSize(totalBytes);
    }

    /**
     * Clean up invalid models
     */
    async cleanupInvalidModels(): Promise<string[]> {
        const removedModels: string[] = [];

        for (const [modelId, model] of this.modelCache.entries()) {
            if (!await this.verifyModel(modelId)) {
                try {
                    if (fs.existsSync(model.path)) {
                        fs.unlinkSync(model.path);
                    }
                    this.modelCache.delete(modelId);
                    removedModels.push(modelId);
                } catch (error) {
                    console.error(`Failed to remove invalid model ${modelId}:`, error);
                }
            }
        }

        return removedModels;
    }
}
