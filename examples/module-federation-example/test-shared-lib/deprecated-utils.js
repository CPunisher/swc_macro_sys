// Deprecated utilities module - completely unused
// Another legacy module that should be entirely removed

export function deprecatedStringUtils(str) {
    // UNUSED FUNCTION - old string utilities
    return {
        original: str,
        reversed: str.split('').reverse().join(''),
        uppercase: str.toUpperCase(),
        lowercase: str.toLowerCase(),
        capitalized: str.charAt(0).toUpperCase() + str.slice(1),
        wordCount: str.split(/\s+/).length,
        charCount: str.length
    };
}

export function deprecatedArrayUtils(arr) {
    // UNUSED FUNCTION - old array utilities
    return {
        original: arr,
        sorted: [...arr].sort(),
        reversed: [...arr].reverse(),
        unique: [...new Set(arr)],
        flattened: arr.flat(Infinity),
        length: arr.length,
        sum: arr.filter(x => typeof x === 'number').reduce((a, b) => a + b, 0)
    };
}

export const DEPRECATED_SETTINGS = {
    // UNUSED CONSTANT - old settings
    version: '0.9.0',
    deprecated: true,
    replacedBy: 'core-utils.js',
    removalDate: '2024-12-31',
    migrationGuide: 'https://docs.example.com/migration',
    supportLevel: 'none',
    features: {
        stringProcessing: true,
        arrayProcessing: true,
        dateProcessing: false,
        fileProcessing: false
    }
};

export function deprecatedMathUtils(numbers) {
    // UNUSED FUNCTION - old math utilities
    const sorted = [...numbers].sort((a, b) => a - b);
    const sum = numbers.reduce((a, b) => a + b, 0);
    const mean = sum / numbers.length;
    
    return {
        sum,
        mean,
        median: sorted[Math.floor(sorted.length / 2)],
        min: Math.min(...numbers),
        max: Math.max(...numbers),
        range: Math.max(...numbers) - Math.min(...numbers),
        variance: numbers.reduce((acc, num) => acc + Math.pow(num - mean, 2), 0) / numbers.length
    };
}

export class DeprecatedProcessor {
    // UNUSED CLASS - old processing class
    constructor() {
        this.deprecationWarnings = [];
        this.usageCount = 0;
        console.warn('DeprecatedProcessor is deprecated and will be removed');
    }
    
    process(data) {
        this.usageCount++;
        this.deprecationWarnings.push({
            timestamp: Date.now(),
            method: 'process',
            message: 'This method is deprecated'
        });
        
        if (typeof data === 'string') {
            return deprecatedStringUtils(data);
        }
        
        if (Array.isArray(data)) {
            if (data.every(x => typeof x === 'number')) {
                return deprecatedMathUtils(data);
            }
            return deprecatedArrayUtils(data);
        }
        
        return {
            error: 'Unsupported data type',
            type: typeof data,
            deprecated: true
        };
    }
    
    getDeprecationInfo() {
        return {
            ...DEPRECATED_SETTINGS,
            warnings: this.deprecationWarnings,
            usageCount: this.usageCount
        };
    }
}