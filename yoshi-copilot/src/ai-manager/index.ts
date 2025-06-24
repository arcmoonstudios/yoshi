/**
 * AI Manager Module for Yoshi Copilot
 *
 * This module provides a unified interface for managing multiple AI providers
 * including GitHub Copilot and AskCodi, with intelligent fallback and
 * load balancing capabilities based on your xIntegrations2.txt specifications.
 */

// Export main provider manager
export { AIProviderManager } from './provider-manager';

// Export individual bridges
export { AskCodiBridge } from './askcodi-bridge';
export { GitHubCopilotBridge } from './github-copilot-bridge';

// Export Yoshi-specific provider (renamed from copilot-bridge)
export { YoshiCopilotProvider } from './yoshi-provider';

// Export types and interfaces
export type {
    AIProvider,
    AIRequest,
    AIResponse,
    AIProviderConfig
} from './provider-manager';

export type {
    AskCodiRequest,
    AskCodiResponse,
    AskCodiConfig
} from './askcodi-bridge';

export type {
    GitHubCopilotRequest,
    GitHubCopilotResponse,
    GitHubCopilotConfig
} from './github-copilot-bridge';
