// Partially used module - some exports used, others should be removed
// Only 'formatDate' and 'calculateAge' are used, rest should be tree-shaken

export function formatDate(date, format = 'YYYY-MM-DD') {
    // USED FUNCTION - should be preserved
    const d = new Date(date);
    const year = d.getFullYear();
    const month = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    
    return format
        .replace('YYYY', year)
        .replace('MM', month)
        .replace('DD', day);
}

export function calculateAge(birthDate) {
    // USED FUNCTION - should be preserved
    const today = new Date();
    const birth = new Date(birthDate);
    let age = today.getFullYear() - birth.getFullYear();
    const monthDiff = today.getMonth() - birth.getMonth();
    
    if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birth.getDate())) {
        age--;
    }
    
    return age;
}

export function unusedDateHelper(date) {
    // UNUSED FUNCTION - should be removed
    return new Date(date).toLocaleDateString('en-US', {
        weekday: 'long',
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

export function unusedTimeCalculator(startTime, endTime) {
    // UNUSED FUNCTION - should be removed
    const start = new Date(startTime);
    const end = new Date(endTime);
    const diff = end - start;
    
    return {
        milliseconds: diff,
        seconds: Math.floor(diff / 1000),
        minutes: Math.floor(diff / (1000 * 60)),
        hours: Math.floor(diff / (1000 * 60 * 60)),
        days: Math.floor(diff / (1000 * 60 * 60 * 24))
    };
}

export const UNUSED_DATE_FORMATS = {
    // UNUSED CONSTANT - should be removed
    iso: 'YYYY-MM-DDTHH:mm:ss.sssZ',
    american: 'MM/DD/YYYY',
    european: 'DD/MM/YYYY',
    readable: 'MMMM Do, YYYY'
};

export class UnusedDateRange {
    // UNUSED CLASS - should be removed
    constructor(start, end) {
        this.start = new Date(start);
        this.end = new Date(end);
    }
    
    contains(date) {
        const d = new Date(date);
        return d >= this.start && d <= this.end;
    }
    
    duration() {
        return this.end - this.start;
    }
}