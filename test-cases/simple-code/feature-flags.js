// Feature flag controlled functions
function expensiveFeature() {
  console.log('Running expensive feature...');
  return { result: 'expensive computation' };
}

function debugFeature() {
  console.log('[DEBUG] Debug feature active');
  console.trace('Stack trace for debugging');
  return { debug: true };
}

function experimentalFeature() {
  console.log('Running experimental feature...');
  return { experimental: true, version: '0.1.0' };
}

// Conditional compilation blocks
/* @common:if [condition="featureFlags.enableExpensiveFeature"] */
export function useExpensiveFeature() {
  return expensiveFeature();
}
/* @common:endif */

/* @common:if [condition="featureFlags.enableDebugMode"] */
export function useDebugFeature() {
  return debugFeature();
}
/* @common:endif */

/* @common:if [condition="featureFlags.enableExperimentalFeature"] */
export function useExperimentalFeature() {
  return experimentalFeature();
}
/* @common:endif */

// Always present
export function baseFeature() {
  return { base: true, message: 'This is always available' };
}

// Inline defines
const buildMode = /* @common:define-inline [value="build.mode" default="development"] */ "development";
const apiUrl = /* @common:define-inline [value="api.url" default="http://localhost:3000"] */ "http://localhost:3000";

export { buildMode, apiUrl }; 
