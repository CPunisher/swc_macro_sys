// Dead Code Elimination Test Cases
// Functions are defined without macros, but calls are conditionally wrapped

// Heavy computation function that should be eliminated if not called
function heavyMathCalculation(n) {
  console.log(`Performing expensive math calculation for ${n}`);
  let result = 0;
  for (let i = 0; i < n * 1000; i++) {
    result += Math.sqrt(i * Math.PI);
  }
  return result;
}

// Expensive UI rendering function
function renderComplexDashboard(data) {
  console.log('Rendering complex dashboard with expensive operations');
  return {
    charts: data.length * 5,
    widgets: data.length * 3,
    animations: 'complex-3d-transitions',
    renderTime: '2.5s'
  };
}

// Debug logging function
function performDetailedLogging(message, context) {
  console.log(`[DETAILED DEBUG] ${new Date().toISOString()}`);
  console.log(`Message: ${message}`);
  console.log(`Context:`, JSON.stringify(context, null, 2));
  console.trace('Full stack trace for debugging');
  return { logged: true, timestamp: Date.now() };
}

// Experimental analytics function
function trackExperimentalMetrics(event, metadata) {
  console.log('Tracking experimental analytics - expensive operation');
  return {
    event,
    metadata,
    experimentalId: Math.random().toString(36),
    processingTime: '150ms'
  };
}

// Helper function used by multiple features
function validateUserPermissions(userId) {
  console.log(`Validating permissions for user ${userId}`);
  return { hasAccess: true, level: 'admin' };
}

// Main application logic
export function runApplication() {
  console.log('Application started - base functionality');
  
  const userData = { id: 1, name: 'Test User' };
  
  // Conditional expensive math calculation
  /* @common:if [condition="featureFlags.enableHeavyMath"] */
  const mathResult = heavyMathCalculation(100);
  console.log('Math calculation result:', mathResult);
  /* @common:endif */
  
  // Conditional UI rendering
  /* @common:if [condition="featureFlags.enableComplexUI"] */
  const permissions = validateUserPermissions(userData.id);
  if (permissions.hasAccess) {
    const dashboard = renderComplexDashboard([1, 2, 3, 4, 5]);
    console.log('Dashboard rendered:', dashboard);
  }
  /* @common:endif */
  
  // Conditional debug logging
  /* @common:if [condition="featureFlags.enableDetailedLogging"] */
  performDetailedLogging('Application initialized', userData);
  /* @common:endif */
  
  // Conditional experimental analytics
  /* @common:if [condition="featureFlags.enableExperimentalAnalytics"] */
  trackExperimentalMetrics('app_start', { version: '1.0.0', user: userData.id });
  /* @common:endif */
  
  return {
    status: 'running',
    user: userData.name
  };
}

// Isolated function calls for testing
export function testIsolatedCalls() {
  // This function call should be removed, and since it's the only call to heavyMathCalculation,
  // the function definition should also be eliminated
  /* @common:if [condition="featureFlags.enableExperimentalFeature"] */
  const result = heavyMathCalculation(50);
  console.log('Experimental calculation:', result);
  /* @common:endif */
  
  return 'Test completed';
}

// Multiple conditional calls to the same function
export function testMultipleConditionalCalls() {
  let results = [];
  
  /* @common:if [condition="featureFlags.enableFeatureA"] */
  results.push(trackExperimentalMetrics('feature_a', { enabled: true }));
  /* @common:endif */
  
  /* @common:if [condition="featureFlags.enableFeatureB"] */
  results.push(trackExperimentalMetrics('feature_b', { enabled: true }));
  /* @common:endif */
  
  /* @common:if [condition="featureFlags.enableFeatureC"] */
  results.push(trackExperimentalMetrics('feature_c', { enabled: true }));
  /* @common:endif */
  
  return results;
}

// Always present baseline function
export function baselineFunction() {
  return 'This function should always be present';
} 