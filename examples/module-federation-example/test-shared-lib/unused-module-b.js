// Another completely unused module - should be removed entirely
// Contains utilities and helpers that are never imported

export const UNUSED_CONFIG = {
    apiUrl: 'https://api.unused.com',
    timeout: 5000,
    retries: 3,
    features: {
        advancedMode: true,
        debugMode: false,
        experimentalFeatures: ['feature1', 'feature2']
    }
};

export function unusedApiCall(endpoint, options = {}) {
    const config = { ...UNUSED_CONFIG, ...options };
    
    return fetch(`${config.apiUrl}/${endpoint}`, {
        timeout: config.timeout,
        headers: {
            'Content-Type': 'application/json',
            'X-API-Key': 'unused-key'
        }
    });
}

export function unusedValidator(data, schema) {
    // Complex validation logic that's never used
    const errors = [];
    
    for (const [key, rules] of Object.entries(schema)) {
        const value = data[key];
        
        if (rules.required && (value === undefined || value === null)) {
            errors.push(`${key} is required`);
        }
        
        if (rules.type && typeof value !== rules.type) {
            errors.push(`${key} must be of type ${rules.type}`);
        }
        
        if (rules.minLength && value.length < rules.minLength) {
            errors.push(`${key} must be at least ${rules.minLength} characters`);
        }
    }
    
    return { isValid: errors.length === 0, errors };
}

export class UnusedDataProcessor {
    constructor(options = {}) {
        this.options = options;
        this.cache = new Map();
    }
    
    process(data) {
        const cacheKey = JSON.stringify(data);
        
        if (this.cache.has(cacheKey)) {
            return this.cache.get(cacheKey);
        }
        
        const processed = this.transform(data);
        this.cache.set(cacheKey, processed);
        
        return processed;
    }
    
    transform(data) {
        return data.map(item => ({
            ...item,
            id: Math.random().toString(36),
            processedAt: new Date().toISOString()
        }));
    }
}