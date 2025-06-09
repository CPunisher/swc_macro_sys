// Complex nested conditional compilation scenario

// Utility functions that may be dead code
function heavyComputation() {
  console.log('Performing heavy computation...');
  return Array.from({length: 1000}, (_, i) => i * i);
}

function networkOperation() {
  console.log('Performing network operation...');
  return Promise.resolve({ data: 'network result' });
}

function uiRendering() {
  console.log('Rendering complex UI...');
  return { rendered: true, elements: 50 };
}

// Nested feature conditions
/* @common:if [condition="platform.isMobile"] */
  /* @common:if [condition="featureFlags.enableMobileOptimizations"] */
  export function mobileOptimizedFeature() {
    return uiRendering();
  }
  /* @common:endif */

  /* @common:if [condition="user.isPremium"] */
  export function premiumMobileFeature() {
    return heavyComputation();
  }
  /* @common:endif */
/* @common:endif */

/* @common:if [condition="platform.isDesktop"] */
  /* @common:if [condition="featureFlags.enableDesktopFeatures"] */
  export function desktopFeature() {
    const computation = heavyComputation();
    const network = networkOperation();
    return { computation, network };
  }
  /* @common:endif */

  /* @common:if [condition="user.isAdmin"] */
  export function adminDesktopFeature() {
    console.log('Admin desktop feature');
    return { admin: true, platform: 'desktop' };
  }
  /* @common:endif */
/* @common:endif */

// Complex conditions with multiple checks
/* @common:if [condition="featureFlags.enableAdvancedFeatures"] */
  /* @common:if [condition="user.permissions.canAccessAdvanced"] */
    /* @common:if [condition="environment.isProduction"] */
    export function productionAdvancedFeature() {
      return {
        advanced: true,
        production: true,
        computation: heavyComputation(),
        ui: uiRendering()
      };
    }
    /* @common:endif */
  /* @common:endif */
/* @common:endif */

// Always present base functionality
export function baseFunction() {
  return { message: 'Base functionality always available' };
}

// Inline defines with conditions
const mobileConfig = /* @common:define-inline [value="mobile.config" default="{}"] */ "{}";
const desktopConfig = /* @common:define-inline [value="desktop.config" default="{}"] */ "{}";

export { mobileConfig, desktopConfig }; 