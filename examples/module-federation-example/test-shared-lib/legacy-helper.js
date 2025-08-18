// Legacy helper module - completely unused
// Represents old code that should be removed entirely

export function legacyFormatDate(date) {
    // UNUSED FUNCTION - old date formatting
    const months = [
        'January', 'February', 'March', 'April', 'May', 'June',
        'July', 'August', 'September', 'October', 'November', 'December'
    ];
    
    const d = new Date(date);
    return `${months[d.getMonth()]} ${d.getDate()}, ${d.getFullYear()}`;
}

export function legacyParseQuery(queryString) {
    // UNUSED FUNCTION - old query string parser
    const params = {};
    const pairs = queryString.slice(1).split('&');
    
    for (const pair of pairs) {
        const [key, value] = pair.split('=');
        if (key) {
            params[decodeURIComponent(key)] = decodeURIComponent(value || '');
        }
    }
    
    return params;
}

export const LEGACY_CONSTANTS = {
    // UNUSED CONSTANT - old configuration
    API_BASE_URL: 'https://api-v1.legacy.com',
    SUPPORTED_FORMATS: ['xml', 'json'],
    MAX_RETRIES: 5,
    TIMEOUT: 10000,
    DEPRECATED_FEATURES: {
        xmlSupport: true,
        legacyAuth: true,
        oldDateFormat: true
    }
};

export class LegacyApiClient {
    // UNUSED CLASS - old API client
    constructor(config = {}) {
        this.config = { ...LEGACY_CONSTANTS, ...config };
        this.requestCount = 0;
    }
    
    legacyRequest(endpoint, options = {}) {
        this.requestCount++;
        
        const url = `${this.config.API_BASE_URL}/${endpoint}`;
        const headers = {
            'Content-Type': 'application/xml',
            'X-Legacy-Client': 'true',
            'X-API-Version': '1.0'
        };
        
        // Simulate old XMLHttpRequest usage
        return new Promise((resolve, reject) => {
            const xhr = new XMLHttpRequest();
            xhr.open(options.method || 'GET', url);
            
            Object.entries(headers).forEach(([key, value]) => {
                xhr.setRequestHeader(key, value);
            });
            
            xhr.onload = () => {
                if (xhr.status >= 200 && xhr.status < 300) {
                    resolve(this.parseLegacyResponse(xhr.responseText));
                } else {
                    reject(new Error(`Legacy API error: ${xhr.status}`));
                }
            };
            
            xhr.onerror = () => reject(new Error('Legacy network error'));
            xhr.send(options.data);
        });
    }
    
    parseLegacyResponse(responseText) {
        // Old response parser
        if (responseText.startsWith('<?xml')) {
            // Simulate XML parsing
            return { format: 'xml', data: responseText };
        }
        
        try {
            return JSON.parse(responseText);
        } catch (e) {
            return { error: 'Parse error', raw: responseText };
        }
    }
    
    getLegacyStats() {
        return {
            requestCount: this.requestCount,
            apiVersion: '1.0',
            deprecated: true
        };
    }
}