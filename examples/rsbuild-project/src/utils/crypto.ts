import { sha256 } from './hash.js';

export class CryptoUtils {
  static encrypt(data: string, key: string): string {
    console.log('Encrypting data with key...');
    const hash = sha256(data + key);
    return btoa(hash); // Simple base64 encoding
  }

  static decrypt(encryptedData: string, key: string): string {
    console.log('Decrypting data with key...');
    return atob(encryptedData); // Simple base64 decoding
  }

  static generateToken(): string {
    const timestamp = Date.now().toString();
    const random = Math.random().toString(36);
    return sha256(timestamp + random);
  }
}

export const cryptoConfig = {
  algorithm: 'SHA-256',
  keyLength: 256,
  iterations: 10000,
}; 