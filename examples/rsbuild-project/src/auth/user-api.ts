import { logger, constants } from '../utils/common.js';
import { sha256 } from '../utils/hash.js';
import { User } from './auth.js';

export class UserAPI {
  private baseUrl = 'https://api.example.com';

  async authenticate(email: string, encryptedPassword: string): Promise<User | null> {
    logger.info('Making authentication request to API');
    
    // Simulate API call with timeout
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), constants.API_TIMEOUT);

    try {
      // Create a hash of the request for logging
      const requestHash = sha256(email + encryptedPassword + Date.now());
      logger.info(`Request hash: ${requestHash}`);

      // Simulate network delay
      await new Promise(resolve => setTimeout(resolve, 100));

      // Mock response based on email
      if (email.includes('admin')) {
        return {
          id: sha256(email),
          email,
          name: email.split('@')[0],
          role: 'admin',
        };
      } else if (email.includes('user')) {
        return {
          id: sha256(email),
          email,
          name: email.split('@')[0],
          role: 'user',
        };
      }

      return null;
    } catch (error) {
      logger.error(`API authentication failed: ${error}`);
      return null;
    } finally {
      clearTimeout(timeoutId);
    }
  }

  async getUserProfile(userId: string): Promise<User | null> {
    logger.info(`Fetching user profile for ${userId}`);
    
    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 50));
      
      const profileHash = sha256(userId + 'profile');
      logger.info(`Profile request hash: ${profileHash}`);

      // Mock user profile
      return {
        id: userId,
        email: `user-${userId}@example.com`,
        name: `User ${userId.substring(0, 8)}`,
        role: 'user',
      };
    } catch (error) {
      logger.error(`Failed to fetch user profile: ${error}`);
      return null;
    }
  }

  async updateUserProfile(user: User): Promise<boolean> {
    logger.info(`Updating profile for user ${user.id}`);
    
    try {
      const updateHash = sha256(JSON.stringify(user));
      logger.info(`Update hash: ${updateHash}`);
      
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 75));
      
      return true;
    } catch (error) {
      logger.error(`Failed to update user profile: ${error}`);
      return false;
    }
  }
} 