// Module with internal dependencies - part of dependency chain
// This module depends on dependency-chain-b but is unused
// Should test cascading removal of dependency chains

import { unusedHelperB, UNUSED_CONFIG_B } from './dependency-chain-b.js';

export function unusedChainFunctionA(data) {
    // UNUSED FUNCTION - depends on chain-b
    // Should be removed along with its dependency
    const processed = unusedHelperB(data);
    
    return {
        ...processed,
        chainLevel: 'A',
        config: UNUSED_CONFIG_B,
        processedBy: 'dependency-chain-a'
    };
}

export function anotherUnusedChainFunction(input) {
    // UNUSED FUNCTION - also depends on chain-b
    const result = unusedChainFunctionA(input);
    
    return {
        ...result,
        doubleProcessed: true,
        finalProcessor: 'chain-a-secondary'
    };
}

export const UNUSED_CHAIN_CONFIG_A = {
    // UNUSED CONSTANT - configuration for chain A
    enabled: false,
    chainId: 'A',
    dependsOn: ['chain-b'],
    features: {
        advancedProcessing: true,
        caching: false,
        logging: true
    },
    thresholds: {
        maxItems: 1000,
        timeout: 5000,
        retries: 3
    }
};

export class UnusedChainProcessorA {
    // UNUSED CLASS - depends on chain-b utilities
    constructor(options = {}) {
        this.options = { ...UNUSED_CHAIN_CONFIG_A, ...options };
        this.chainB = unusedHelperB;
        this.processedItems = [];
    }
    
    processChain(items) {
        // Uses dependency from chain-b
        const results = items.map(item => {
            const processed = this.chainB(item);
            return {
                ...processed,
                chainProcessorA: true,
                timestamp: Date.now()
            };
        });
        
        this.processedItems.push(...results);
        return results;
    }
    
    getChainStats() {
        return {
            totalProcessed: this.processedItems.length,
            chainLevel: 'A',
            dependencyConfig: UNUSED_CONFIG_B,
            processorConfig: this.options
        };
    }
    
    clearChain() {
        this.processedItems = [];
    }
}