// Your exact example: function defined without macro, call wrapped with macro

function experimentalFeature() {
  console.log('This is an experimental feature that should be eliminated if not called');
  return {
    experimental: true,
    version: '0.1.0',
    warning: 'Use at your own risk'
  };
}

function anotherFeature() {
  console.log('Another feature that may or may not be called');
  return { feature: 'another', status: 'active' };
}

// Main application
export function main() {
  console.log('Application started');
  
  /* @common:if [condition="featureFlags.enableExperimentalFeature"] */
  experimentalFeature();
  /* @common:endif */
  
  /* @common:if [condition="featureFlags.enableAnotherFeature"] */
  anotherFeature();
  /* @common:endif */
  
  return 'Application completed';
}

// Always present function
export function baselineFunction() {
  return 'Always available';
} 