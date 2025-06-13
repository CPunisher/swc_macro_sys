import { debounce, throttle } from 'lodash-es';

export function formatDate(date: Date): string {
  return date.toISOString().split('T')[0];
}

export function formatCurrency(amount: number, currency = 'USD'): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
  }).format(amount);
}

export const logger = {
  info: (message: string) => console.log(`[INFO] ${new Date().toISOString()}: ${message}`),
  warn: (message: string) => console.warn(`[WARN] ${new Date().toISOString()}: ${message}`),
  error: (message: string) => console.error(`[ERROR] ${new Date().toISOString()}: ${message}`),
};

export const debounceClick = debounce((fn: () => void) => fn(), 300);
export const throttleScroll = throttle((fn: () => void) => fn(), 100);

export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

export const constants = {
  MAX_FILE_SIZE: 10 * 1024 * 1024, // 10MB
  SUPPORTED_FORMATS: ['jpg', 'png', 'pdf'],
  API_TIMEOUT: 30000,
}; 