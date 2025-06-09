// Utility functions that would only be used by newFeature
function formatMessage(message) {
  return `[NEW] ${message}`;
}

function validateFeature() {
  return true;
}

function logFeatureUsage(featureName) {
  console.log(`Feature ${featureName} was used at ${new Date().toISOString()}`);
}

function getFeatureConfig() {
  return {
    enabled: true,
    version: "1.0.0",
    metadata: {
      author: "dev-team",
      created: "2024-01-01"
    }
  };
}

/* @common:if [condition="featureFlags.enableExpensiveFeature"] */
export function useExpensiveFeature() {
  // This function references the utility functions above
  if (!validateFeature()) {
    return null;
  }
  
  const config = getFeatureConfig();
  const message = formatMessage(`Expensive feature v${config.version} is enabled!`);
  
  logFeatureUsage("expensiveFeature");
  
  return {
    message,
    config,
    timestamp: new Date().toISOString()
  };
}
/* @common:endif */

/* @common:if [condition="featureFlags.enableDebugMode"] */
export function useDebugFeature() {
  console.log('Debug mode is active');
  return { debug: true, mode: 'development' };
}
/* @common:endif */

/* @common:if [condition="featureFlags.enableExperimentalFeature"] */
export function useExperimentalFeature() {
  return { experimental: true, warning: 'Use at your own risk' };
}
/* @common:endif */

// Base functionality that should always be present
export function baseFeature() {
  return "Base functionality always available";
}

// Always present code
export function alwaysPresent() {
  return "This function is always present";
}

const buildMode =
  /* @common:define-inline [value="build.mode" default="development"] */ "development";

const apiUrl = 
  /* @common:define-inline [value="api.url" default="http://localhost:3000"] */ "http://localhost:3000";

/* @common:if [condition="user.isLoggedIn"] */
function getUserData() {
  return { id: 1, name: "John Doe" };
}

export function getWelcomeMessage() {
  const user = getUserData();
  return `Welcome back, ${user.name}!`;
}
/* @common:endif */

export { buildMode, apiUrl };