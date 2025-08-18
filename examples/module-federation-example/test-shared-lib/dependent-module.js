// Module that depends on both chain modules but has one used function
// Tests partial dependency resolution - some dependencies removed, others kept

import { unusedChainFunctionA } from './dependency-chain-a.js';
import { shallowNestedFunction } from './nested/used-shallow.js';

export function usedDependentFunction(data) {
    // USED FUNCTION - depends on nested/used-shallow.js
    // This function and its dependency should be preserved
    const processed = shallowNestedFunction(data);
    
    return {
        ...processed,
        dependentProcessor: true,
        processedBy: 'dependent-module'
    };
}

export function unusedDependentFunction(data) {
    // UNUSED FUNCTION - depends on dependency-chain-a
    // This function and its dependency chain should be removed
    const chainResult = unusedChainFunctionA(data);
    
    return {
        ...chainResult,
        finalProcessing: true,
        processedBy: 'dependent-module-unused'
    };
}

export function anotherUsedFunction(input) {
    // USED FUNCTION - independent functionality
    // Should be preserved
    return {
        input,
        processed: true,
        independent: true,
        timestamp: Date.now()
    };
}

export const DEPENDENT_CONFIG = {
    // USED CONSTANT - configuration for dependent module
    name: 'dependent-module',
    version: '1.0.0',
    dependencies: {
        shallow: 'nested/used-shallow.js',
        chainA: 'dependency-chain-a.js' // This dependency should be removable
    },
    features: {
        processUsed: true,
        processUnused: false // This feature uses unused dependency
    }
};

export class DependentProcessor {
    // USED CLASS - mixes used and unused dependencies
    constructor(options = {}) {
        this.options = { ...DEPENDENT_CONFIG, ...options };
        this.processedCount = 0;
    }
    
    processUsed(data) {
        // USED METHOD - uses used dependency
        this.processedCount++;
        return usedDependentFunction(data);
    }
    
    processIndependent(data) {
        // USED METHOD - independent processing
        this.processedCount++;
        return anotherUsedFunction(data);
    }
    
    processUnused(data) {
        // UNUSED METHOD - uses unused dependency chain
        // This method and its dependency should be removed
        return unusedDependentFunction(data);
    }
    
    getStats() {
        // USED METHOD - returns statistics
        return {
            processedCount: this.processedCount,
            config: this.options,
            capabilities: ['processUsed', 'processIndependent']
        };
    }
}