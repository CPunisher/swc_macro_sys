// Example consumer that imports only specific functions
// This simulates how the library would actually be used
// Only these imports should be preserved in optimization

// Import specific functions that are actually used
import { 
    formatDate, 
    calculateAge 
} from './partial-usage-a.js';

import { 
    isValidEmail, 
    generateId 
} from './partial-usage-b.js';

import { 
    debounce, 
    throttle, 
    deepClone, 
    merge,
    CORE_CONSTANTS,
    EventEmitter 
} from './core-utils.js';

import { 
    processUserData, 
    createApiClient, 
    debouncedSave,
    DataManager,
    FEATURE_CONFIG 
} from './main-feature.js';

import { 
    shallowNestedFunction, 
    transformNestedData,
    NESTED_CONFIG,
    ShallowNestedProcessor 
} from './nested/used-shallow.js';

import { 
    usedDependentFunction, 
    anotherUsedFunction,
    DEPENDENT_CONFIG,
    DependentProcessor 
} from './dependent-module.js';

// Example usage demonstrating actual consumption patterns
export class ApplicationExample {
    constructor() {
        this.eventEmitter = new EventEmitter();
        this.apiClient = createApiClient('https://api.example.com');
        this.dataManager = new DataManager(this.apiClient);
        this.nestedProcessor = new ShallowNestedProcessor();
        this.dependentProcessor = new DependentProcessor();
        
        this.setupEventHandlers();
    }
    
    setupEventHandlers() {
        // Use debounced functions
        this.debouncedValidateEmail = debounce(this.validateEmail.bind(this), 300);
        this.throttledSave = throttle(this.saveData.bind(this), 1000);
        
        // Setup event listeners
        this.eventEmitter.on('dataChanged', this.throttledSave);
        this.eventEmitter.on('userInput', this.debouncedValidateEmail);
    }
    
    async processUser(userData) {
        // Use multiple imported functions
        const id = generateId('user');
        const processedData = processUserData({
            ...userData,
            id,
            email: userData.email?.toLowerCase()
        });
        
        // Use nested processing
        const nestedResult = this.nestedProcessor.process(processedData);
        const dependentResult = this.dependentProcessor.processUsed(nestedResult);
        
        // Use date formatting
        if (userData.birthDate) {
            dependentResult.formattedBirthDate = formatDate(userData.birthDate);
            dependentResult.age = calculateAge(userData.birthDate);
        }
        
        return dependentResult;
    }
    
    validateEmail(email) {
        return isValidEmail(email);
    }
    
    async saveData(data) {
        // Use data manager and deep clone
        const clonedData = deepClone(data);
        this.dataManager.updateData(clonedData.id, clonedData);
        
        // Use debounced save from main-feature
        await debouncedSave(clonedData);
        
        this.eventEmitter.emit('dataSaved', clonedData);
    }
    
    mergeConfiguration(userConfig) {
        // Use merge utility
        return merge(
            deepClone(CORE_CONSTANTS),
            deepClone(FEATURE_CONFIG),
            deepClone(NESTED_CONFIG),
            deepClone(DEPENDENT_CONFIG),
            userConfig
        );
    }
    
    processNestedData(data) {
        // Use nested functions
        const shallow = shallowNestedFunction(data);
        const transformed = transformNestedData(shallow);
        const dependent = anotherUsedFunction(transformed);
        
        return dependent;
    }
    
    getApplicationStats() {
        return {
            nestedStats: this.nestedProcessor.getStats(),
            dependentStats: this.dependentProcessor.getStats(),
            dataManagerStats: this.dataManager.isDirty,
            configuration: this.mergeConfiguration({})
        };
    }
}

// Example standalone usage
export function exampleUsage() {
    const app = new ApplicationExample();
    
    // Example user processing
    const userData = {
        name: 'John Doe',
        email: 'JOHN.DOE@EXAMPLE.COM',
        birthDate: '1990-05-15'
    };
    
    return app.processUser(userData);
}

// Export specific utility combinations that consumers might use
export const utilities = {
    formatDate,
    calculateAge,
    isValidEmail,
    generateId,
    debounce,
    throttle,
    deepClone,
    merge
};

export const processors = {
    ShallowNestedProcessor,
    DependentProcessor,
    DataManager
};

export const constants = {
    CORE_CONSTANTS,
    FEATURE_CONFIG,
    NESTED_CONFIG,
    DEPENDENT_CONFIG
};

// This file represents realistic usage patterns and shows which
// functions from the test library are actually needed