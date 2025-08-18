// Setup file for Vitest tests
import { beforeAll, afterAll } from 'vitest';

// Mock globals for webpack/rspack runtime
beforeAll(() => {
  global.self = {
    webpackChunkhost: [],
    webpackChunkremote: []
  };
  
  global.__webpack_require__ = {
    r: () => {},
    d: () => {},
    o: () => true
  };
  
  global.__webpack_exports__ = {};
  
  // Mock Module Federation runtime
  global.__webpack_init_sharing__ = () => Promise.resolve();
  global.__webpack_share_scopes__ = { default: {} };
});

afterAll(() => {
  // Cleanup globals
  delete global.self;
  delete global.__webpack_require__;
  delete global.__webpack_exports__;
  delete global.__webpack_init_sharing__;
  delete global.__webpack_share_scopes__;
});