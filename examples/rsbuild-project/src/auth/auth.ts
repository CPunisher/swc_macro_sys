import { CryptoUtils } from '../utils/crypto.js';
import { logger, validateEmail } from '../utils/common.js';
import { UserAPI } from './user-api.js';

export interface User {
  id: string;
  email: string;
  name: string;
  role: 'user' | 'admin';
}

export class AuthService {
  private static instance: AuthService;
  private currentUser: User | null = null;
  private userAPI: UserAPI;

  private constructor() {
    this.userAPI = new UserAPI();
  }

  static getInstance(): AuthService {
    if (!AuthService.instance) {
      AuthService.instance = new AuthService();
    }
    return AuthService.instance;
  }

  async login(email: string, password: string): Promise<User | null> {
    logger.info(`Attempting login for ${email}`);
    
    if (!validateEmail(email)) {
      logger.error('Invalid email format');
      return null;
    }

    try {
      const token = CryptoUtils.generateToken();
      const encryptedPassword = CryptoUtils.encrypt(password, token);
      
      const user = await this.userAPI.authenticate(email, encryptedPassword);
      
      if (user) {
        this.currentUser = user;
        localStorage.setItem('auth_token', token);
        logger.info(`Login successful for ${email}`);
      }
      
      return user;
    } catch (error) {
      logger.error(`Login failed: ${error}`);
      return null;
    }
  }

  logout(): void {
    this.currentUser = null;
    localStorage.removeItem('auth_token');
    logger.info('User logged out');
  }

  getCurrentUser(): User | null {
    return this.currentUser;
  }

  isAuthenticated(): boolean {
    return this.currentUser !== null;
  }
} 