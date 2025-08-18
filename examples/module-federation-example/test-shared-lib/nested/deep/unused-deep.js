// Deep nested module that is completely unused - should be removed entirely
// Tests deep nested directory tree-shaking

export function deepUnusedFunction(complexData) {
    // UNUSED FUNCTION - deep in nested structure
    const processed = complexData.map(item => {
        return {
            ...item,
            deepProcessed: true,
            level: 'deep',
            nestedPath: 'nested/deep/unused-deep.js',
            complexTransformation: {
                original: item,
                transformed: true,
                metadata: {
                    processedAt: new Date().toISOString(),
                    processor: 'deep-nested',
                    version: '1.0.0'
                }
            }
        };
    });
    
    return processed.sort((a, b) => 
        a.complexTransformation.metadata.processedAt.localeCompare(
            b.complexTransformation.metadata.processedAt
        )
    );
}

export function expensiveDeepCalculation(matrix) {
    // UNUSED FUNCTION - computationally expensive
    const result = [];
    
    for (let i = 0; i < matrix.length; i++) {
        result[i] = [];
        for (let j = 0; j < matrix[i].length; j++) {
            let sum = 0;
            for (let k = 0; k < matrix.length; k++) {
                sum += matrix[i][k] * matrix[k][j];
            }
            result[i][j] = sum;
        }
    }
    
    return result;
}

export const DEEP_NESTED_CONSTANTS = {
    // UNUSED CONSTANT - deep configuration
    MAX_DEPTH: 10,
    PROCESSING_MODES: {
        FAST: 'fast',
        THOROUGH: 'thorough',
        EXPERIMENTAL: 'experimental'
    },
    CACHE_SETTINGS: {
        enabled: true,
        maxSize: 1000,
        ttl: 600000,
        strategy: 'lru'
    },
    PERFORMANCE_THRESHOLDS: {
        warning: 1000,
        error: 5000,
        critical: 10000
    }
};

export class DeepNestedAnalyzer {
    // UNUSED CLASS - complex deep nested functionality
    constructor(config = {}) {
        this.config = { ...DEEP_NESTED_CONSTANTS, ...config };
        this.cache = new Map();
        this.metrics = {
            totalAnalyses: 0,
            cacheHits: 0,
            averageProcessingTime: 0
        };
    }
    
    analyze(data, mode = 'fast') {
        const startTime = performance.now();
        this.metrics.totalAnalyses++;
        
        const cacheKey = this.generateCacheKey(data, mode);
        
        if (this.cache.has(cacheKey)) {
            this.metrics.cacheHits++;
            return this.cache.get(cacheKey);
        }
        
        let result;
        
        switch (mode) {
            case 'fast':
                result = this.fastAnalysis(data);
                break;
            case 'thorough':
                result = this.thoroughAnalysis(data);
                break;
            case 'experimental':
                result = this.experimentalAnalysis(data);
                break;
            default:
                throw new Error(`Unknown analysis mode: ${mode}`);
        }
        
        const processingTime = performance.now() - startTime;
        this.updateMetrics(processingTime);
        
        if (this.cache.size < this.config.CACHE_SETTINGS.maxSize) {
            this.cache.set(cacheKey, result);
        }
        
        return result;
    }
    
    fastAnalysis(data) {
        return {
            summary: 'Fast analysis completed',
            dataPoints: data.length,
            mode: 'fast',
            complexity: 'low'
        };
    }
    
    thoroughAnalysis(data) {
        const patterns = this.detectPatterns(data);
        const anomalies = this.findAnomalies(data);
        const correlations = this.calculateCorrelations(data);
        
        return {
            summary: 'Thorough analysis completed',
            dataPoints: data.length,
            mode: 'thorough',
            complexity: 'high',
            patterns,
            anomalies,
            correlations
        };
    }
    
    experimentalAnalysis(data) {
        // Simulated experimental algorithms
        return {
            summary: 'Experimental analysis completed',
            dataPoints: data.length,
            mode: 'experimental',
            complexity: 'experimental',
            confidence: Math.random(),
            experimentalMetrics: {
                noveltyScore: Math.random(),
                uncertaintyIndex: Math.random(),
                explorationFactor: Math.random()
            }
        };
    }
    
    detectPatterns(data) {
        // Simulate pattern detection
        return data.slice(0, 5).map(item => ({
            pattern: `pattern_${Math.random().toString(36).substr(2, 9)}`,
            confidence: Math.random(),
            frequency: Math.floor(Math.random() * 100)
        }));
    }
    
    findAnomalies(data) {
        // Simulate anomaly detection
        return data.filter(() => Math.random() > 0.9).map(item => ({
            anomaly: item,
            severity: Math.random(),
            type: ['outlier', 'unusual_pattern', 'data_quality'][
                Math.floor(Math.random() * 3)
            ]
        }));
    }
    
    calculateCorrelations(data) {
        // Simulate correlation calculation
        const correlations = {};
        const fields = Object.keys(data[0] || {});
        
        for (let i = 0; i < fields.length; i++) {
            for (let j = i + 1; j < fields.length; j++) {
                const key = `${fields[i]}_${fields[j]}`;
                correlations[key] = (Math.random() - 0.5) * 2; // -1 to 1
            }
        }
        
        return correlations;
    }
    
    generateCacheKey(data, mode) {
        const dataHash = JSON.stringify(data).length; // Simple hash
        return `${mode}_${dataHash}_${data.length}`;
    }
    
    updateMetrics(processingTime) {
        const totalTime = this.metrics.averageProcessingTime * (this.metrics.totalAnalyses - 1);
        this.metrics.averageProcessingTime = (totalTime + processingTime) / this.metrics.totalAnalyses;
    }
    
    getMetrics() {
        return {
            ...this.metrics,
            cacheHitRatio: this.metrics.cacheHits / this.metrics.totalAnalyses,
            cacheSize: this.cache.size
        };
    }
}