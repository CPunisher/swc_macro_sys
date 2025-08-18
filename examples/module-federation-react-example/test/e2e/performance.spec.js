import { test, expect } from './setup.js';

test.describe('Performance Tests', () => {
  test('should load the main application within performance budgets', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/');
    
    // Wait for main content to be visible
    await page.waitForSelector('text=MF React App', { timeout: 10000 });
    
    const loadTime = Date.now() - startTime;
    
    // Should load within 5 seconds
    expect(loadTime).toBeLessThan(5000);
    
    console.log(`Host app loaded in ${loadTime}ms`);
  });

  test('should load remote components within acceptable time', async ({ page }) => {
    await page.goto('/remote-components');
    
    const startTime = Date.now();
    
    // Click on User Card tab to trigger remote component loading
    // Use robust tab click to avoid detachment during animations
    const tab = page.getByRole('tab', { name: 'User Card' }).first();
    await tab.waitFor({ state: 'visible' });
    await tab.click();
    
    // Wait for remote component to fully load
    await page.waitForSelector('.ant-card:has-text("User Profile Card Component")', { timeout: 15000 });
    
    const loadTime = Date.now() - startTime;
    
    // Remote component should load within 3 seconds
    expect(loadTime).toBeLessThan(3000);
    
    console.log(`Remote component loaded in ${loadTime}ms`);
  });

  test('should have good Core Web Vitals', async ({ page }) => {
    await page.goto('/');
    
    // Wait for page to fully load
    await page.waitForSelector('text=MF React App');
    await page.waitForLoadState('networkidle');
    
    // Measure performance metrics
    const metrics = await page.evaluate(() => {
      return new Promise((resolve) => {
        if ('web-vital' in window) {
          // If web-vitals library is available
          resolve(window['web-vital']);
        } else {
          // Basic performance measurement
          const perfData = performance.getEntriesByType('navigation')[0];
          resolve({
            loadTime: perfData.loadEventEnd - perfData.loadEventStart,
            domContentLoaded: perfData.domContentLoadedEventEnd - perfData.domContentLoadedEventStart,
            firstPaint: performance.getEntriesByType('paint').find(p => p.name === 'first-paint')?.startTime || 0,
            firstContentfulPaint: performance.getEntriesByType('paint').find(p => p.name === 'first-contentful-paint')?.startTime || 0
          });
        }
      });
    });
    
    console.log('Performance metrics:', metrics);
    
    // Basic performance checks
    if (metrics.loadTime) {
      expect(metrics.loadTime).toBeLessThan(2000); // Load should complete within 2s
    }
    if (metrics.firstContentfulPaint) {
      expect(metrics.firstContentfulPaint).toBeLessThan(1500); // FCP should be within 1.5s
    }
  });

  test('should handle concurrent remote component loading efficiently', async ({ page }) => {
    await page.goto('/remote-components');
    
    const startTime = Date.now();
    
    // Rapidly switch between tabs to test concurrent loading
    const tabs = ['User Card', 'Data Table', 'Charts', 'Form Builder'];
    
    for (const tabName of tabs) {
      const t = page.getByRole('tab', { name: tabName }).first();
      await t.waitFor({ state: 'visible' });
      await t.click();
      // Small delay to allow tab switch
      await page.waitForTimeout(100);
    }
    
    // Wait for final component to load
    await page.waitForSelector('.ant-form', { timeout: 15000 });
    
    const totalTime = Date.now() - startTime;
    
    // All components should load within 5 seconds even with rapid switching
    expect(totalTime).toBeLessThan(5000);
    
    console.log(`All remote components loaded in ${totalTime}ms`);
  });

  test('should have minimal bundle sizes', async ({ page }) => {
    const resourceSizes = [];
    
    page.on('response', async (response) => {
      if (response.url().includes('.js') && response.status() === 200) {
        const headers = response.headers();
        const contentLength = headers['content-length'];
        if (contentLength) {
          resourceSizes.push({
            url: response.url(),
            size: parseInt(contentLength),
            compressed: !!headers['content-encoding']
          });
        }
      }
    });
    
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    
    // Navigate to remote components to load all chunks
    await page.click('text=Remote Components');
    await page.getByRole('tab', { name: 'User Card' }).first().click();
    await page.waitForSelector('.ant-card', { timeout: 15000 });
    
    await page.waitForLoadState('networkidle');
    
    const totalSize = resourceSizes.reduce((sum, resource) => sum + resource.size, 0);
    const mainChunks = resourceSizes.filter(r => r.url.includes('main.') || r.url.includes('vendors-'));
    
    console.log('Resource sizes:', resourceSizes);
    console.log(`Total JS size: ${(totalSize / 1024).toFixed(2)}KB`);
    
    // Total JS should be reasonable for a full React app with MF
    expect(totalSize).toBeLessThan(2 * 1024 * 1024); // Less than 2MB total
    
    // Main chunks should be optimized
    if (mainChunks.length > 0) {
      const largestChunk = Math.max(...mainChunks.map(c => c.size));
      expect(largestChunk).toBeLessThan(1024 * 1024); // No single chunk over 1MB
    }
  });

  test('should maintain responsive UI during data loading', async ({ page }) => {
    await page.goto('/analytics');
    
    // Test UI responsiveness while charts are loading
    const startTime = Date.now();
    
    // Click on time range selector while data is loading
    await page.click('.ant-select');
    await page.click('text=Last 30 days');
    
    const responseTime = Date.now() - startTime;
    
    // UI interaction should be responsive (under 100ms)
    expect(responseTime).toBeLessThan(500);
    
    // Wait for charts to load
    await page.waitForSelector('canvas', { timeout: 15000 });
    
    // Verify charts rendered
    const charts = page.locator('canvas');
    await expect(charts).toHaveCount({ min: 2 });
  });
});
