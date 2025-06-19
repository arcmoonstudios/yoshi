/**
 * # Security Manager for Yoshi MCP
 * 
 * Provides security validation and input sanitization
 * for the VS Code extension environment.
 */

/**
 * Security policy configuration
 */
export interface SecurityPolicy {
    maxPromptLength: number;
    maxOutputLength: number;
    allowedFileExtensions: string[];
    blockedPatterns: string[];
    maxInferenceTimeMs: number;
}

/**
 * Input validation result
 */
export interface ValidationResult {
    isValid: boolean;
    errors: string[];
    warnings: string[];
    sanitizedInput?: string;
}

/**
 * Security manager class
 */
export class SecurityManager {
    private policy: SecurityPolicy;

    constructor(customPolicy?: Partial<SecurityPolicy>) {
        this.policy = {
            maxPromptLength: 10000,
            maxOutputLength: 50000,
            allowedFileExtensions: ['.gguf'],
            blockedPatterns: [
                'system(',
                'exec(',
                'eval(',
                'import os',
                'subprocess',
                '__import__',
                'file://',
                'javascript:',
                '<script',
                'document.cookie',
                'localStorage',
                'sessionStorage'
            ],
            maxInferenceTimeMs: 30000,
            ...customPolicy
        };
    }

    /**
     * Validate user input
     */
    validateInput(input: string): ValidationResult {
        const errors: string[] = [];
        const warnings: string[] = [];

        // Check input length
        if (input.length > this.policy.maxPromptLength) {
            errors.push(`Input too long: ${input.length} characters (max: ${this.policy.maxPromptLength})`);
        }

        // Check for blocked patterns
        const foundPatterns = this.findBlockedPatterns(input);
        if (foundPatterns.length > 0) {
            errors.push(`Blocked patterns found: ${foundPatterns.join(', ')}`);
        }

        // Check for excessive repetition
        if (this.hasExcessiveRepetition(input)) {
            warnings.push('Input contains excessive repetitive content');
        }

        // Check for potential injection attempts
        const injectionAttempts = this.detectInjectionAttempts(input);
        if (injectionAttempts.length > 0) {
            errors.push(`Potential injection attempts: ${injectionAttempts.join(', ')}`);
        }

        // Sanitize input if valid
        let sanitizedInput: string | undefined;
        if (errors.length === 0) {
            sanitizedInput = this.sanitizeInput(input);
        }

        return {
            isValid: errors.length === 0,
            errors,
            warnings,
            sanitizedInput
        };
    }

    /**
     * Find blocked patterns in input
     */
    private findBlockedPatterns(input: string): string[] {
        const lowerInput = input.toLowerCase();
        return this.policy.blockedPatterns.filter(pattern => 
            lowerInput.includes(pattern.toLowerCase())
        );
    }

    /**
     * Check for excessive repetition
     */
    private hasExcessiveRepetition(input: string): boolean {
        const words = input.split(/\s+/);
        if (words.length < 10) {
            return false;
        }

        // Check for repeated sequences
        const windowSize = 5;
        let repetitionCount = 0;

        for (let i = 0; i <= words.length - windowSize * 2; i++) {
            const window1 = words.slice(i, i + windowSize);
            const window2 = words.slice(i + windowSize, i + windowSize * 2);
            
            if (this.arraysEqual(window1, window2)) {
                repetitionCount++;
                if (repetitionCount > 3) {
                    return true;
                }
            }
        }

        return false;
    }

    /**
     * Detect potential injection attempts
     */
    private detectInjectionAttempts(input: string): string[] {
        const injectionPatterns = [
            /\b(union|select|insert|update|delete|drop|create|alter)\s+/i,
            /<\s*script[^>]*>/i,
            /javascript\s*:/i,
            /on\w+\s*=/i,
            /\$\{.*\}/,
            /`.*`/,
            /\{\{.*\}\}/
        ];

        const attempts: string[] = [];
        
        for (const pattern of injectionPatterns) {
            if (pattern.test(input)) {
                attempts.push(pattern.source);
            }
        }

        return attempts;
    }

    /**
     * Sanitize input text
     */
    private sanitizeInput(input: string): string {
        return input
            // Remove control characters except whitespace
            .replace(/[\x00-\x08\x0B\x0C\x0E-\x1F\x7F]/g, '')
            // Normalize whitespace
            .replace(/\s+/g, ' ')
            // Trim
            .trim()
            // Limit length
            .substring(0, this.policy.maxPromptLength);
    }

    /**
     * Validate file path
     */
    validateFilePath(filePath: string): ValidationResult {
        const errors: string[] = [];
        const warnings: string[] = [];

        // Check for path traversal
        if (filePath.includes('..') || filePath.includes('~')) {
            errors.push('Path traversal detected');
        }

        // Check for absolute paths (security risk)
        if (filePath.startsWith('/') || /^[A-Za-z]:/.test(filePath)) {
            warnings.push('Absolute path detected');
        }

        // Check file extension
        const extension = filePath.split('.').pop()?.toLowerCase();
        if (extension && !this.policy.allowedFileExtensions.includes(`.${extension}`)) {
            errors.push(`File extension not allowed: .${extension}`);
        }

        // Check for suspicious characters
        if (/[<>"|*?]/.test(filePath)) {
            errors.push('Suspicious characters in file path');
        }

        return {
            isValid: errors.length === 0,
            errors,
            warnings,
            sanitizedInput: errors.length === 0 ? this.sanitizeFilePath(filePath) : undefined
        };
    }

    /**
     * Sanitize file path
     */
    private sanitizeFilePath(filePath: string): string {
        return filePath
            // Remove dangerous characters
            .replace(/[<>"|*?]/g, '')
            // Remove path traversal attempts
            .replace(/\.\./g, '')
            // Normalize separators
            .replace(/[/\\]+/g, '/')
            // Remove leading/trailing slashes
            .replace(/^\/+|\/+$/g, '');
    }

    /**
     * Validate output length
     */
    validateOutput(output: string): ValidationResult {
        const errors: string[] = [];
        const warnings: string[] = [];

        if (output.length > this.policy.maxOutputLength) {
            errors.push(`Output too long: ${output.length} characters (max: ${this.policy.maxOutputLength})`);
        }

        // Check for potential data leakage patterns
        const leakagePatterns = [
            /password\s*[:=]\s*\S+/i,
            /api[_-]?key\s*[:=]\s*\S+/i,
            /token\s*[:=]\s*\S+/i,
            /secret\s*[:=]\s*\S+/i
        ];

        for (const pattern of leakagePatterns) {
            if (pattern.test(output)) {
                warnings.push(`Potential sensitive data in output: ${pattern.source}`);
            }
        }

        return {
            isValid: errors.length === 0,
            errors,
            warnings,
            sanitizedInput: errors.length === 0 ? output : undefined
        };
    }

    /**
     * Check if inference time is within limits
     */
    validateInferenceTime(startTime: number, endTime: number): ValidationResult {
        const duration = endTime - startTime;
        const errors: string[] = [];

        if (duration > this.policy.maxInferenceTimeMs) {
            errors.push(`Inference time exceeded: ${duration}ms (max: ${this.policy.maxInferenceTimeMs}ms)`);
        }

        return {
            isValid: errors.length === 0,
            errors,
            warnings: []
        };
    }

    /**
     * Get current security policy
     */
    getPolicy(): SecurityPolicy {
        return { ...this.policy };
    }

    /**
     * Update security policy
     */
    updatePolicy(updates: Partial<SecurityPolicy>): void {
        this.policy = { ...this.policy, ...updates };
    }

    /**
     * Generate security report
     */
    generateSecurityReport(): {
        policy: SecurityPolicy;
        recommendations: string[];
    } {
        const recommendations: string[] = [];

        if (this.policy.maxPromptLength > 20000) {
            recommendations.push('Consider reducing maxPromptLength for better security');
        }

        if (this.policy.blockedPatterns.length < 5) {
            recommendations.push('Consider adding more blocked patterns for enhanced security');
        }

        if (this.policy.maxInferenceTimeMs > 60000) {
            recommendations.push('Consider reducing maxInferenceTimeMs to prevent resource exhaustion');
        }

        return {
            policy: this.getPolicy(),
            recommendations
        };
    }

    /**
     * Helper method to compare arrays
     */
    private arraysEqual<T>(a: T[], b: T[]): boolean {
        if (a.length !== b.length) return false;
        return a.every((val, index) => val === b[index]);
    }
}
