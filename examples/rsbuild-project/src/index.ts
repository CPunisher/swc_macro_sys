import './index.css';
import { AuthService, User } from './auth/auth.js';
import { UserAPI } from './auth/user-api.js';
import { ComponentRenderer } from './ui/components.js';
import { UILibrary } from './ui/ui-library.js';
import { logger, debounceClick, throttleScroll } from './utils/common.js';
import { CryptoUtils, cryptoConfig } from './utils/crypto.js';

class Application {
  private authService: AuthService;
  private userAPI: UserAPI;
  private componentRenderer: ComponentRenderer;
  private uiLibrary: UILibrary;
  private rootElement: HTMLElement;

  constructor() {
    this.authService = AuthService.getInstance();
    this.userAPI = new UserAPI();
    this.componentRenderer = new ComponentRenderer();
    this.uiLibrary = new UILibrary();
    this.rootElement = document.querySelector('#root') || document.body;
    
    logger.info('Application initialized');
    logger.info(`Crypto config: ${JSON.stringify(cryptoConfig)}`);
  }

  async initialize(): Promise<void> {
    logger.info('Starting application initialization...');
    
    this.setupUI();
    this.setupEventListeners();
    
    // Try to restore session
    const token = localStorage.getItem('auth_token');
    if (token) {
      logger.info('Found existing auth token, attempting to restore session');
      // Simulate session restoration
      await this.handleLogin('user@example.com', 'password123');
    }

    logger.info('Application initialization complete');
  }

  private setupUI(): void {
    // Clear existing content
    this.rootElement.innerHTML = '';

    // Create main container
    const container = this.uiLibrary.createContainer('app-container');
    
    // Create header
    const header = this.uiLibrary.createHeader('Complex Dependency Demo App', 'h1');
    container.appendChild(header);

    // Create auth section
    const authSection = this.createAuthSection();
    container.appendChild(authSection);

    // Create user info section (initially hidden)
    const userSection = this.createUserSection();
    userSection.style.display = 'none';
    userSection.id = 'user-section';
    container.appendChild(userSection);

    // Create demo data section
    const dataSection = this.createDataSection();
    container.appendChild(dataSection);

    this.rootElement.appendChild(container);
  }

  private createAuthSection(): HTMLElement {
    const section = this.uiLibrary.createContainer('auth-section');
    const title = this.uiLibrary.createHeader('Authentication', 'h2');
    
    const emailInput = this.uiLibrary.createInput('email', 'Enter email');
    emailInput.id = 'email-input';
    emailInput.value = 'user@example.com'; // Pre-fill for demo
    
    const passwordInput = this.uiLibrary.createInput('password', 'Enter password');
    passwordInput.id = 'password-input';
    passwordInput.value = 'password123'; // Pre-fill for demo

    const loginButton = this.componentRenderer.renderClickableButton(
      'Login',
      () => this.handleLoginClick()
    );

    const logoutButton = this.componentRenderer.renderClickableButton(
      'Logout',
      () => this.handleLogout()
    );
    logoutButton.style.display = 'none';
    logoutButton.id = 'logout-button';

    section.appendChild(title);
    section.appendChild(emailInput);
    section.appendChild(passwordInput);
    section.appendChild(loginButton);
    section.appendChild(logoutButton);

    return section;
  }

  private createUserSection(): HTMLElement {
    const section = this.uiLibrary.createContainer('user-section');
    const title = this.uiLibrary.createHeader('User Information', 'h2');
    
    const userCardContainer = document.createElement('div');
    userCardContainer.id = 'user-card-container';
    
    const priceDisplay = this.componentRenderer.renderPriceDisplay(99.99, 'USD');
    
    section.appendChild(title);
    section.appendChild(userCardContainer);
    section.appendChild(priceDisplay);

    return section;
  }

  private createDataSection(): HTMLElement {
    const section = this.uiLibrary.createContainer('data-section');
    const title = this.uiLibrary.createHeader('Sample Data', 'h2');
    
    // Create sample data for table
    const sampleData = [
      { id: 1, name: 'Alice', email: 'alice@example.com', role: 'admin' },
      { id: 2, name: 'Bob', email: 'bob@example.com', role: 'user' },
      { id: 3, name: 'Charlie', email: 'charlie@example.com', role: 'user' },
    ];
    
    const dataTable = this.componentRenderer.renderDataTable(sampleData);
    
    section.appendChild(title);
    section.appendChild(dataTable);

    return section;
  }

  private setupEventListeners(): void {
    // Setup throttled scroll listener
    window.addEventListener('scroll', () => {
      throttleScroll(() => {
        logger.info(`Scroll position: ${window.scrollY}`);
      });
    });

    // Setup window resize listener
    window.addEventListener('resize', () => {
      debounceClick(() => {
        logger.info(`Window resized: ${window.innerWidth}x${window.innerHeight}`);
      });
    });
  }

  private async handleLoginClick(): Promise<void> {
    const emailInput = document.getElementById('email-input') as HTMLInputElement;
    const passwordInput = document.getElementById('password-input') as HTMLInputElement;
    
    if (!emailInput || !passwordInput) {
      logger.error('Login form elements not found');
      return;
    }

    await this.handleLogin(emailInput.value, passwordInput.value);
  }

  private async handleLogin(email: string, password: string): Promise<void> {
    logger.info(`Attempting login for ${email}`);
    
    try {
      const user = await this.authService.login(email, password);
      
      if (user) {
        this.showUserSection(user);
        this.toggleAuthButtons(true);
        logger.info('Login successful');
      } else {
        logger.error('Login failed');
        alert('Login failed. Please check your credentials.');
      }
    } catch (error) {
      logger.error(`Login error: ${error}`);
      alert('An error occurred during login.');
    }
  }

  private handleLogout(): void {
    this.authService.logout();
    this.hideUserSection();
    this.toggleAuthButtons(false);
    logger.info('User logged out');
  }

  private showUserSection(user: User): void {
    const userSection = document.getElementById('user-section');
    const userCardContainer = document.getElementById('user-card-container');
    
    if (userSection && userCardContainer) {
      // Clear existing user card
      userCardContainer.innerHTML = '';
      
      // Create and add new user card
      const userCard = this.componentRenderer.renderUserCard(user);
      userCardContainer.appendChild(userCard);
      
      userSection.style.display = 'block';
    }
  }

  private hideUserSection(): void {
    const userSection = document.getElementById('user-section');
    if (userSection) {
      userSection.style.display = 'none';
    }
  }

  private toggleAuthButtons(isLoggedIn: boolean): void {
    const loginButton = document.querySelector('.ui-button') as HTMLElement;
    const logoutButton = document.getElementById('logout-button') as HTMLElement;
    
    if (loginButton) {
      loginButton.style.display = isLoggedIn ? 'none' : 'inline-block';
    }
    
    if (logoutButton) {
      logoutButton.style.display = isLoggedIn ? 'inline-block' : 'none';
    }
  }

  // Demonstrate crypto utilities
  private demonstrateCrypto(): void {
    const testData = 'sensitive user data';
    const key = 'secret-key-123';
    
    const encrypted = CryptoUtils.encrypt(testData, key);
    const decrypted = CryptoUtils.decrypt(encrypted, key);
    const token = CryptoUtils.generateToken();
    
    logger.info(`Crypto demo - Original: ${testData}`);
    logger.info(`Crypto demo - Encrypted: ${encrypted}`);
    logger.info(`Crypto demo - Decrypted: ${decrypted}`);
    logger.info(`Crypto demo - Generated token: ${token}`);
  }
}

// Initialize the application
document.addEventListener('DOMContentLoaded', async () => {
  const app = new Application();
  await app.initialize();
  
  // Run crypto demonstration
  (app as any).demonstrateCrypto();
  
  logger.info('DOM content loaded and application started');
});
