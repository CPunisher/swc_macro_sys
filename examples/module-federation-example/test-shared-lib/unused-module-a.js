// Completely unused module - should be removed entirely
// Contains various functions that are never imported or used

export function unusedFunction1() {
    return 'This function is never called';
}

export function unusedFunction2(param) {
    console.log('Unused function with parameter:', param);
    return param * 2;
}

export const UNUSED_CONSTANT = 'This constant is never referenced';

export class UnusedClass {
    constructor(value) {
        this.value = value;
    }
    
    unusedMethod() {
        return this.value + ' processed';
    }
}

export default function unusedDefaultExport() {
    return 'Default export that is never used';
}

// Complex function that would normally be expensive to include
export function expensiveUnusedFunction(data) {
    // Simulate expensive operations
    const result = data.map(item => {
        return {
            ...item,
            processed: true,
            timestamp: Date.now(),
            hash: Math.random().toString(36)
        };
    });
    
    return result.sort((a, b) => a.timestamp - b.timestamp);
}