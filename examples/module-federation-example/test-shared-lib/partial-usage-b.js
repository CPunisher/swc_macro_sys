// Another partially used module - mixed usage patterns
// Only 'isValidEmail' and 'generateId' are used

export function isValidEmail(email) {
    // USED FUNCTION - should be preserved
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

export function generateId(prefix = '') {
    // USED FUNCTION - should be preserved
    const timestamp = Date.now().toString(36);
    const random = Math.random().toString(36).substr(2);
    return prefix ? `${prefix}_${timestamp}_${random}` : `${timestamp}_${random}`;
}

export function unusedPasswordValidator(password) {
    // UNUSED FUNCTION - should be removed
    const checks = {
        minLength: password.length >= 8,
        hasUppercase: /[A-Z]/.test(password),
        hasLowercase: /[a-z]/.test(password),
        hasNumbers: /\d/.test(password),
        hasSpecialChars: /[!@#$%^&*(),.?":{}|<>]/.test(password)
    };
    
    return {
        isValid: Object.values(checks).every(check => check),
        checks
    };
}

export function unusedSlugify(text) {
    // UNUSED FUNCTION - should be removed
    return text
        .toLowerCase()
        .trim()
        .replace(/[^\w\s-]/g, '')
        .replace(/[\s_-]+/g, '-')
        .replace(/^-+|-+$/g, '');
}

export const UNUSED_VALIDATION_RULES = {
    // UNUSED CONSTANT - should be removed
    email: {
        required: true,
        pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    },
    password: {
        required: true,
        minLength: 8,
        pattern: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/
    },
    phone: {
        pattern: /^\+?[\d\s-()]+$/
    }
};

export function unusedFormValidator(formData, rules) {
    // UNUSED FUNCTION - should be removed
    const errors = {};
    
    for (const [field, rule] of Object.entries(rules)) {
        const value = formData[field];
        
        if (rule.required && (!value || value.trim() === '')) {
            errors[field] = `${field} is required`;
            continue;
        }
        
        if (value && rule.pattern && !rule.pattern.test(value)) {
            errors[field] = `${field} format is invalid`;
            continue;
        }
        
        if (value && rule.minLength && value.length < rule.minLength) {
            errors[field] = `${field} must be at least ${rule.minLength} characters`;
        }
    }
    
    return {
        isValid: Object.keys(errors).length === 0,
        errors
    };
}

export default function unusedDefaultUtility(data) {
    // UNUSED DEFAULT EXPORT - should be removed
    return {
        ...data,
        processed: true,
        timestamp: new Date().toISOString()
    };
}