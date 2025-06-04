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

/* @swc:if [condition="featureFlags.enableNewFeature"] */
export function newFeature() {
  // This function references the utility functions above
  if (!validateFeature()) {
    return null;
  }
  
  const config = getFeatureConfig();
  const message = formatMessage(`New feature v${config.version} is enabled!`);
  
  logFeatureUsage("newFeature");
  
  return {
    message,
    config,
    timestamp: new Date().toISOString()
  };
}
/* @swc:endif */

// Another conditional block with different condition
/* @swc:if [condition="featureFlags.newMobileUI"] */
function mobileUIHelper() {
  return "Mobile UI is active";
}

export function getMobileUI() {
  return mobileUIHelper();
}
/* @swc:endif */

// Always present code
export function alwaysPresent() {
  return "This function is always present";
}

const buildTarget =
  /* @swc:define-inline [value="build.target" default="development"] */ "development";

const apiEndpoint = 
  /* @swc:define-inline [value="api.endpoint" default="http://localhost:3000"] */ "http://localhost:3000";

/* @swc:if [condition="user.isLoggedIn"] */
function getUserData() {
  return { id: 1, name: "John Doe" };
}

export function getWelcomeMessage() {
  const user = getUserData();
  return `Welcome back, ${user.name}!`;
}
/* @swc:endif */

export { buildTarget, apiEndpoint };