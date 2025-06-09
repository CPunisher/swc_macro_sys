export function sha256(input: string): string {
  // Simple hash function simulation (in real app would use crypto.subtle)
  let hash = 0;
  for (let i = 0; i < input.length; i++) {
    const char = input.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32bit integer
  }
  return Math.abs(hash).toString(16);
}

export function md5(input: string): string {
  // Another simple hash simulation
  let hash = 0;
  for (let i = 0; i < input.length; i++) {
    const char = input.charCodeAt(i);
    hash = ((hash << 3) - hash) + char;
    hash = hash & hash;
  }
  return Math.abs(hash).toString(16);
}

export const hashConfig = {
  defaultAlgorithm: 'SHA-256',
  saltLength: 16,
}; 