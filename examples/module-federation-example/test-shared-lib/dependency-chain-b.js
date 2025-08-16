// Module that is imported by dependency-chain-a but never used directly
// Should be removed when chain-a is removed (cascading removal)

export function unusedHelperB(data) {
    // UNUSED FUNCTION - only used by unused chain-a
    // Should be removed in cascading removal
    return {
        ...data,
        helperB: true,
        processedAt: new Date().toISOString(),
        chainLevel: 'B'
    };
}

export function anotherUnusedHelperB(input, options = {}) {
    // UNUSED FUNCTION - utility only for chain-a
    const defaults = {
        transform: true,
        validate: false,
        cache: true
    };
    
    const config = { ...defaults, ...options };
    
    if (config.validate) {
        if (!input || typeof input !== 'object') {
            throw new Error('Invalid input for helper B');
        }
    }
    
    let result = { ...input };
    
    if (config.transform) {
        result = {
            ...result,
            transformed: true,
            transformedBy: 'helper-b',
            transformedAt: Date.now()
        };
    }
    
    if (config.cache) {
        result.cacheKey = `helper-b-${JSON.stringify(input).length}`;
    }
    
    return result;
}

export const UNUSED_CONFIG_B = {
    // UNUSED CONSTANT - only used by chain-a
    chainLevel: 'B',
    isHelper: true,
    supportedOperations: ['transform', 'validate', 'cache'],
    maxPayloadSize: 1024 * 1024, // 1MB
    timeout: 30000,
    retryConfig: {
        attempts: 3,
        backoff: 'exponential',
        initialDelay: 1000
    }
};

export const UNUSED_UTILITIES_B = {
    // UNUSED CONSTANT - utility functions for chain-a
    serialize: (data) => JSON.stringify(data),
    deserialize: (str) => {
        try {
            return JSON.parse(str);
        } catch (e) {
            return null;
        }
    },
    hash: (input) => {
        let hash = 0;
        const str = typeof input === 'string' ? input : JSON.stringify(input);
        for (let i = 0; i < str.length; i++) {
            const char = str.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash; // Convert to 32-bit integer
        }
        return hash.toString(36);
    },
    validate: (data, schema) => {
        // Simple validation utility
        for (const [key, rules] of Object.entries(schema)) {
            const value = data[key];
            
            if (rules.required && value === undefined) {
                return { valid: false, error: `${key} is required` };
            }
            
            if (value !== undefined && rules.type && typeof value !== rules.type) {
                return { valid: false, error: `${key} must be of type ${rules.type}` };
            }
        }
        
        return { valid: true };
    }
};

export class UnusedHelperClassB {
    // UNUSED CLASS - only used by chain-a
    constructor(config = {}) {
        this.config = { ...UNUSED_CONFIG_B, ...config };
        this.cache = new Map();
        this.stats = {
            operations: 0,
            cacheHits: 0,
            errors: 0
        };
    }
    
    process(data, useCache = true) {
        this.stats.operations++;
        
        const cacheKey = UNUSED_UTILITIES_B.hash(data);
        
        if (useCache && this.cache.has(cacheKey)) {
            this.stats.cacheHits++;
            return this.cache.get(cacheKey);
        }
        
        try {
            const result = unusedHelperB(data);
            
            if (useCache) {
                this.cache.set(cacheKey, result);
            }
            
            return result;
        } catch (error) {
            this.stats.errors++;
            throw error;
        }
    }
    
    getStats() {
        return {
            ...this.stats,
            cacheSize: this.cache.size,
            hitRatio: this.stats.cacheHits / this.stats.operations
        };
    }
    
    clearCache() {
        this.cache.clear();
    }
}