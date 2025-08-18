// Main feature module - all exports are used
// Represents core business logic that should be fully preserved

import { debounce, CORE_CONSTANTS } from './core-utils.js';

export function processUserData(userData) {
    // USED FUNCTION - core business logic
    return {
        id: userData.id,
        name: userData.name?.trim(),
        email: userData.email?.toLowerCase(),
        processedAt: new Date().toISOString(),
        version: CORE_CONSTANTS.API_VERSION
    };
}

export function createApiClient(baseUrl, options = {}) {
    // USED FUNCTION - essential API functionality
    const config = {
        timeout: CORE_CONSTANTS.DEFAULT_TIMEOUT,
        retries: CORE_CONSTANTS.MAX_RETRY_ATTEMPTS,
        ...options
    };
    
    return {
        get: (endpoint) => makeRequest('GET', endpoint, null, config),
        post: (endpoint, data) => makeRequest('POST', endpoint, data, config),
        put: (endpoint, data) => makeRequest('PUT', endpoint, data, config),
        delete: (endpoint) => makeRequest('DELETE', endpoint, null, config)
    };
}

async function makeRequest(method, endpoint, data, config) {
    // USED FUNCTION - internal helper for API client
    const url = `${config.baseUrl}/${endpoint}`;
    let attempts = 0;
    
    while (attempts < config.retries) {
        try {
            const response = await fetch(url, {
                method,
                headers: {
                    'Content-Type': 'application/json',
                    'X-API-Version': CORE_CONSTANTS.API_VERSION
                },
                body: data ? JSON.stringify(data) : undefined,
                signal: AbortSignal.timeout(config.timeout)
            });
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            return await response.json();
        } catch (error) {
            attempts++;
            if (attempts >= config.retries) {
                throw error;
            }
            
            // Exponential backoff
            await new Promise(resolve => 
                setTimeout(resolve, Math.pow(2, attempts) * 1000)
            );
        }
    }
}

export const debouncedSave = debounce((data) => {
    // USED FUNCTION - utilizes core utility
    console.log('Saving data:', data);
    // Simulate save operation
    return Promise.resolve({ success: true, timestamp: Date.now() });
}, 500);

export class DataManager {
    // USED CLASS - core business functionality
    constructor(apiClient) {
        this.api = apiClient;
        this.cache = new Map();
        this.isDirty = false;
    }
    
    async getData(id) {
        if (this.cache.has(id)) {
            return this.cache.get(id);
        }
        
        const data = await this.api.get(`data/${id}`);
        this.cache.set(id, data);
        return data;
    }
    
    updateData(id, updates) {
        const existing = this.cache.get(id) || {};
        const updated = { ...existing, ...updates };
        this.cache.set(id, updated);
        this.isDirty = true;
        
        // Use debounced save
        debouncedSave(updated);
    }
    
    async saveChanges() {
        if (!this.isDirty) return;
        
        const savePromises = [];
        for (const [id, data] of this.cache.entries()) {
            savePromises.push(this.api.put(`data/${id}`, data));
        }
        
        await Promise.all(savePromises);
        this.isDirty = false;
    }
}

export const FEATURE_CONFIG = {
    // USED CONSTANT - feature configuration
    enableBatching: true,
    batchSize: 10,
    autoSave: true,
    autoSaveInterval: 30000
};