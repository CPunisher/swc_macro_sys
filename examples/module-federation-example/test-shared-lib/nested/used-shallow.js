// Nested module that is used - should be preserved
// Tests handling of nested directory structures

export function shallowNestedFunction(data) {
    // USED FUNCTION - from nested structure
    return {
        processed: data,
        level: 'shallow',
        timestamp: Date.now(),
        path: 'nested/used-shallow.js'
    };
}

export function transformNestedData(input) {
    // USED FUNCTION - data transformation in nested module
    if (Array.isArray(input)) {
        return input.map(item => ({
            ...item,
            nestedTransform: true,
            level: 'shallow'
        }));
    }
    
    return {
        ...input,
        nestedTransform: true,
        level: 'shallow'
    };
}

export const NESTED_CONFIG = {
    // USED CONSTANT - configuration in nested structure
    level: 'shallow',
    enabled: true,
    features: ['transform', 'process', 'validate']
};

export class ShallowNestedProcessor {
    // USED CLASS - business logic in nested structure
    constructor(options = {}) {
        this.options = { ...NESTED_CONFIG, ...options };
        this.processedCount = 0;
    }
    
    process(data) {
        this.processedCount++;
        
        return {
            ...shallowNestedFunction(data),
            processedBy: 'ShallowNestedProcessor',
            processCount: this.processedCount
        };
    }
    
    getStats() {
        return {
            totalProcessed: this.processedCount,
            processor: 'shallow-nested',
            options: this.options
        };
    }
}