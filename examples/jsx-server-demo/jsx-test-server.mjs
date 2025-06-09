import { transformSync } from '@swc/core';
import React from 'react';
import { renderToString } from 'react-dom/server';
import vm from 'vm';
import { createRequire } from 'module';
import fs from 'fs';

const require = createRequire(import.meta.url);

// Complex JSX component with nested conditionals mirroring App.tsx patterns
const jsxComponent = `
const React = require('react');

function ComplexAppRouter({ userType = "free", platform = "desktop", deviceCapabilities = {}, abTestVariant = "grid" }) {
  const isMobile = platform === "mobile";
  const isDesktop = platform === "desktop";
  const isTablet = platform === "tablet";
  
  // Simulate feature flags from App.tsx
  const hasAdvancedAnalytics = userType === "premium" || userType === "enterprise";
  const hasCollaboration = userType === "enterprise";
  const hasMobileCamera = isMobile && deviceCapabilities.camera;
  const hasDesktopShortcuts = isDesktop;
  const has3DVisualization = hasAdvancedAnalytics;
  const hasNotifications = true;
  const isAdmin = userType === "admin";
  
  // Navigation component
  const Navigation = () => (
    <nav style={{ 
      padding: '10px', 
      backgroundColor: '#f8f9fa', 
      borderBottom: '1px solid #dee2e6',
      marginBottom: '20px' 
    }}>
      <h2 style={{ margin: '0 0 10px 0', color: '#495057' }}>
        SWC Demo - Complex Conditional Routing
      </h2>
      {/* @common:if [condition="platform.isMobile"] */}
      {isMobile ? (
        <div style={{ fontSize: '14px', color: '#6c757d' }}>
          📱 Mobile Interface Active
          {/* @common:if [condition="platform.hasVibration"] */}
          {deviceCapabilities.vibration && (
            <span style={{ marginLeft: '10px', color: '#28a745' }}>
              ✨ Haptic feedback available
            </span>
          )}
          {/* @common:endif */}
        </div>
      ) : (
        <div style={{ fontSize: '14px', color: '#6c757d' }}>
          <>🖥️ Desktop Interface Active</>
          {/* @common:if [condition="platform.isDesktop"] */}
          {isDesktop && (
            <span style={{ marginLeft: '10px', color: '#007bff' }}>
              ⌨️ Keyboard shortcuts enabled
            </span>
          )} 
          {/* @common:endif */}
        </div>
      )}
      {/* @common:endif */}
    </nav>
  );

  // Dashboard Layout based on A/B test variant
  const DashboardContent = () => (
    <div style={{ padding: '15px', border: '1px solid #e9ecef', borderRadius: '8px', marginBottom: '20px' }}>
      <h3 style={{ color: '#343a40', marginBottom: '15px' }}>
        Dashboard ({abTestVariant} layout)
      </h3>
      
      {/* @common:if [condition="abTests.dashboardLayout === 'grid'"] */}
      {abTestVariant === "grid" ? (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '15px' }}>
          <div style={{ padding: '10px', backgroundColor: '#e3f2fd', borderRadius: '6px' }}>
            <strong>Grid Item 1</strong>
            <p style={{ fontSize: '12px', margin: '5px 0' }}>Grid layout active</p>
          </div>
          <div style={{ padding: '10px', backgroundColor: '#f3e5f5', borderRadius: '6px' }}>
            <strong>Grid Item 2</strong>
            <p style={{ fontSize: '12px', margin: '5px 0' }}>Optimized for desktop</p>
          </div>
          {/* @common:if [condition="featureFlags.advanced-analytics"] */}
          {hasAdvancedAnalytics && (
            <div style={{ padding: '10px', backgroundColor: '#e8f5e8', borderRadius: '6px' }}>
              <strong>Analytics Panel</strong>
              <p style={{ fontSize: '12px', margin: '5px 0' }}>Premium analytics enabled</p>
              {/* @common:if [condition="featureFlags.3d-visualization"] */}
              {has3DVisualization && (
                <div style={{ marginTop: '8px', fontSize: '11px', color: '#28a745' }}>
                  🎯 3D visualization ready
                </div>
              )}
              {/* @common:endif */}
            </div>
          )}
          {/* @common:endif */}
        </div>
      ) : (
        /* @common:if [condition="abTests.dashboardLayout === 'list'"] */
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ padding: '12px', backgroundColor: '#fff3cd', border: '1px solid #ffeaa7', borderRadius: '6px' }}>
            <strong>List Item 1</strong> - Linear layout active
          </div>
          <div style={{ padding: '12px', backgroundColor: '#d1ecf1', border: '1px solid #bee5eb', borderRadius: '6px' }}>
            <strong>List Item 2</strong> - Mobile-optimized
          </div>
          {/* @common:if [condition="featureFlags.advanced-analytics"] */}
          {hasAdvancedAnalytics && (
            <div style={{ padding: '12px', backgroundColor: '#d4edda', border: '1px solid #c3e6cb', borderRadius: '6px' }}>
              <strong>Analytics List View</strong> - Premium features
            </div>
          )}
          {/* @common:endif */}
        </div>
        /* @common:endif */
      )}
      {/* @common:endif */}
    </div>
  );

  // Platform-specific features
  const PlatformFeatures = () => (
    <div style={{ marginBottom: '20px' }}>
      {/* @common:if [condition="platform.isMobile"] */}
      {isMobile ? (
        <div style={{ padding: '15px', backgroundColor: '#f8d7da', border: '1px solid #f5c6cb', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#721c24' }}>📱 Mobile Features</h4>
          
          {/* @common:if [condition="featureFlags.mobile-camera"] */}
          {hasMobileCamera && (
            <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#d1ecf1', borderRadius: '4px' }}>
              📷 Camera access enabled
              {/* @common:if [condition="platform.hasWakeLock"] */}
              {deviceCapabilities.wakeLock && (
                <div style={{ fontSize: '12px', color: '#0c5460', marginTop: '4px' }}>
                  🔒 Screen wake lock available
                </div>
              )}
              {/* @common:endif */}
            </div>
          )}
          {/* @common:endif */}
          
          {/* @common:if [condition="platform.hasDeviceOrientation"] */}
          {deviceCapabilities.orientation && (
            <div style={{ padding: '8px', backgroundColor: '#fff3cd', borderRadius: '4px' }}>
              🧭 Device orientation: {/* @common:define-inline [value="device.orientation" default="portrait"] */"portrait"}
            </div>
          )}
          {/* @common:endif */}
        </div>
      ) : (
        /* @common:if [condition="platform.isDesktop"] */
        <div style={{ padding: '15px', backgroundColor: '#d4edda', border: '1px solid #c3e6cb', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#155724' }}>🖥️ Desktop Features</h4>
          
          {/* @common:if [condition="featureFlags.desktop-shortcuts"] */}
          {hasDesktopShortcuts && (
            <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#e2e3e5', borderRadius: '4px' }}>
              ⌨️ Keyboard shortcuts active
              {/* @common:if [condition="platform.hasWebGL"] */}
              {deviceCapabilities.webgl && (
                <div style={{ fontSize: '12px', color: '#383d41', marginTop: '4px' }}>
                  🎮 WebGL acceleration enabled
                </div>
              )}
              {/* @common:endif */}
            </div>
          )}
          {/* @common:endif */}
          
          {/* @common:if [condition="featureFlags.advanced-analytics"] */}
          {hasAdvancedAnalytics && (
            <div style={{ padding: '8px', backgroundColor: '#cce5ff', borderRadius: '4px' }}>
              📊 Desktop analytics dashboard
            </div>
          )}
          {/* @common:endif */}
        </div>
        /* @common:endif */
      )}
      {/* @common:endif */}
    </div>
  );

  // User-specific features  
  const UserFeatures = () => (
    <div style={{ marginBottom: '20px' }}>
      {/* @common:if [condition="user.type === 'enterprise'"] */}
      {userType === "enterprise" ? (
        <div style={{ padding: '15px', backgroundColor: '#e7f3ff', border: '1px solid #b3d9ff', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#004085' }}>🏢 Enterprise Features</h4>
          
          {/* @common:if [condition="featureFlags.real-time-collaboration"] */}
          {hasCollaboration && (
            <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#d1f2eb', borderRadius: '4px' }}>
              👥 Real-time collaboration enabled
              {/* @common:if [condition="featureFlags.video-calling"] */}
              {true && (
                <div style={{ fontSize: '12px', color: '#0e6655', marginTop: '4px' }}>
                  📹 Video calling available
                </div>
              )}
              {/* @common:endif */}
            </div>
          )}
          {/* @common:endif */}
          
          {/* @common:if [condition="featureFlags.advanced-analytics"] */}
          {hasAdvancedAnalytics && (
            <div style={{ padding: '8px', backgroundColor: '#fff2cc', borderRadius: '4px' }}>
              📈 Enterprise analytics & reporting
            </div>
          )}
          {/* @common:endif */}
        </div>
      ) : userType === "premium" ? (
        /* @common:if [condition="user.type === 'premium'"] */
        <div style={{ padding: '15px', backgroundColor: '#fff0f5', border: '1px solid #ffb3d1', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#6d1650' }}>⭐ Premium Features</h4>
          
          {/* @common:if [condition="featureFlags.advanced-analytics"] */}
          {hasAdvancedAnalytics && (
            <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#e1f5fe', borderRadius: '4px' }}>
              📊 Advanced analytics
            </div>
          )}
          {/* @common:endif */}
          
          {/* @common:if [condition="featureFlags.ai-suggestions"] */}
          <div style={{ padding: '8px', backgroundColor: '#f3e5f5', borderRadius: '4px' }}>
            🤖 AI-powered suggestions
          </div>
          {/* @common:endif */}
        </div>
        /* @common:endif */
      ) : (
        /* @common:if [condition="user.type === 'free'"] */
        <div style={{ padding: '15px', backgroundColor: '#f8f9fa', border: '1px solid #dee2e6', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#495057' }}>🆓 Free Tier</h4>
          <div style={{ padding: '8px', backgroundColor: '#e9ecef', borderRadius: '4px' }}>
            Basic features available. Upgrade for premium features!
          </div>
        </div>
        /* @common:endif */
      )}
    </div>
  );

  return (
    <div style={{ maxWidth: '800px', margin: '0 auto', padding: '20px', fontFamily: 'Arial, sans-serif' }}>
      <Navigation />
      <DashboardContent />
      <PlatformFeatures />
      <UserFeatures />
      
      {/* @common:if [condition="user.isAdmin"] */}
      {isAdmin && (
        <div style={{ 
          padding: '15px', 
          backgroundColor: '#fff3cd', 
          border: '2px solid #ffc107', 
          borderRadius: '8px',
          marginBottom: '20px'
        }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#856404' }}>🔧 Admin Panel</h4>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))', gap: '10px' }}>
            <div style={{ padding: '8px', backgroundColor: '#f8d7da', borderRadius: '4px', fontSize: '12px' }}>
              Feature Flags Control
            </div>
            <div style={{ padding: '8px', backgroundColor: '#d1ecf1', borderRadius: '4px', fontSize: '12px' }}>
              A/B Test Management  
            </div>
            <div style={{ padding: '8px', backgroundColor: '#d4edda', borderRadius: '4px', fontSize: '12px' }}>
              User Analytics
            </div>
          </div>
        </div>
      )}
      {/* @common:endif */}

      {/* @common:if [condition="featureFlags.notifications"] */}
      {hasNotifications && (
        <div style={{ 
          position: 'fixed', 
          top: '10px', 
          right: '10px', 
          padding: '10px', 
          backgroundColor: '#28a745', 
          color: 'white',
          borderRadius: '6px',
          fontSize: '12px',
          maxWidth: '200px'
        }}>
          🔔 Notifications enabled
          {/* @common:if [condition="platform.isMobile && platform.hasVibration"] */}
          {isMobile && deviceCapabilities.vibration && (
            <div style={{ marginTop: '4px', fontSize: '10px' }}>
              📳 With haptic feedback
            </div>
          )}
          {/* @common:endif */}
        </div>
      )}
      {/* @common:endif */}

      <div style={{ 
        marginTop: '30px', 
        padding: '15px', 
        backgroundColor: '#e9ecef',
        borderRadius: '8px',
        fontSize: '11px',
        color: '#495057'
      }}>
        <strong>Build Configuration:</strong>
        <pre style={{ margin: '8px 0', fontSize: '10px', whiteSpace: 'pre-wrap' }}>
{JSON.stringify({
  timestamp: /* @common:define-inline [value="build.timestamp" default="new Date().toISOString()"] */new Date().toISOString(),
  target: /* @common:define-inline [value="build.target" default="development"] */"development",
  platform,
  userType,
  abTestVariant,
  enabledFeatures: {
    hasAdvancedAnalytics,
    hasCollaboration,
    hasMobileCamera,
    hasDesktopShortcuts,
    has3DVisualization,
    hasNotifications,
    isAdmin
  }
}, null, 2)}
        </pre>
      </div>
    </div>
  );
}

module.exports = ComplexAppRouter;
`;

function transformJSX(code) {
  try {
    console.log('🔧 Transforming JSX with SWC...');
    
    const result = transformSync(code, {
      jsc: {
        parser: {
          syntax: 'ecmascript',
          jsx: true,
        },
        transform: {
          react: {
            runtime: 'classic', // Use React.createElement
          },
        },
        target: 'es2020',
        preserveAllComments: true,
      },
      module: {
        type: 'commonjs',
      },
    });
    
    console.log('✅ JSX transformation successful');
    return result.code;
    
  } catch (error) {
    console.error('❌ JSX transformation failed:', error);
    throw error;
  }
}

async function optimizeWithWasm(code, config) {
  console.log('🚀 Optimizing with WASM macro processor...');
  
  const wasmModulePath = '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js';
  const wasmModuleUrl = `file://${process.cwd()}/${wasmModulePath}`;
  const wasmModule = await import(wasmModuleUrl);
  
  const optimizedCode = wasmModule.optimize(code, JSON.stringify(config));
  
  const originalSize = code.length;
  const optimizedSize = optimizedCode.length;
  const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
  const reductionText = reduction > 0 ? `-${reduction.toFixed(1)}%` : 
                       reduction < 0 ? `+${Math.abs(reduction).toFixed(1)}%` : '0%';
  
  console.log(`✅ WASM optimization complete (${originalSize} → ${optimizedSize} chars, ${reductionText})`);
  return optimizedCode;
}

function evaluateComponent(transformedCode) {
  try {
    console.log('📦 Evaluating transformed component...');
    
    // Create a new context with React available
    const context = vm.createContext({
      require,
      module: { exports: {} },
      exports: {},
      React,
      console,
    });
    
    // Execute the transformed code in the context
    vm.runInContext(transformedCode, context);
    
    // Get the exported component
    const Component = context.module.exports;
    
    if (typeof Component !== 'function') {
      throw new Error('Component is not a function');
    }
    
    console.log('✅ Component evaluation successful');
    return Component;
    
  } catch (error) {
    console.error('❌ Component evaluation failed:', error);
    throw error;
  }
}

function renderComponent(Component, props = {}) {
  try {
    console.log('🎨 Rendering component to HTML...');
    
    // Create React element and render to string
    const element = React.createElement(Component, props);
    const html = renderToString(element);
    
    console.log('✅ Component rendering successful');
    return html;
    
  } catch (error) {
    console.error('❌ Component rendering failed:', error);
    throw error;
  }
}

// Test JSX transformation, WASM optimization, and rendering
async function runTests() {
  console.log('🚀 Starting SWC JSX Transform + WASM Optimization Test\n');
  
  try {
    console.log('=== JSX Syntax Transformation ===');
    const transformedCode = transformJSX(jsxComponent);
    console.log('📄 Transformed code:');
    console.log(transformedCode);
    console.log('\n');
    
    console.log('=== WASM Macro Optimization ===');
    
    // Test multiple configurations to see conditional compilation in action
    const configs = [
      {
        name: "Enterprise Mobile User",
        config: {
          build: { target: 'production', timestamp: new Date().toISOString() },
          platform: { 
            isMobile: true, 
            isDesktop: false,
            hasVibration: true,
            hasWakeLock: true,
            hasDeviceOrientation: true,
            hasCamera: true
          },
          device: { orientation: 'portrait' },
          featureFlags: {
            'advanced-analytics': true,
            'real-time-collaboration': true,
            'mobile-camera': true,
            'desktop-shortcuts': false,
            '3d-visualization': true,
            'notifications': true,
            'video-calling': true,
            'ai-suggestions': true
          },
          user: { type: 'enterprise', isAdmin: false },
          abTests: { dashboardLayout: 'list' }
        },
        props: { 
          userType: 'enterprise', 
          platform: 'mobile',
          deviceCapabilities: { vibration: true, wakeLock: true, orientation: true, camera: true, webgl: false },
          abTestVariant: 'list'
        }
      },
      {
        name: "Premium Desktop User",
        config: {
          build: { target: 'production', timestamp: new Date().toISOString() },
          platform: { 
            isMobile: false, 
            isDesktop: true,
            hasVibration: false,
            hasWakeLock: false,
            hasDeviceOrientation: false,
            hasWebGL: true
          },
          featureFlags: {
            'advanced-analytics': true,
            'real-time-collaboration': false,
            'mobile-camera': false,
            'desktop-shortcuts': true,
            '3d-visualization': true,
            'notifications': true,
            'video-calling': false,
            'ai-suggestions': true
          },
          user: { type: 'premium', isAdmin: false },
          abTests: { dashboardLayout: 'grid' }
        },
        props: { 
          userType: 'premium', 
          platform: 'desktop',
          deviceCapabilities: { vibration: false, wakeLock: false, orientation: false, camera: false, webgl: true },
          abTestVariant: 'grid'
        }
      },
      {
        name: "Admin Free User",
        config: {
          build: { target: 'development', timestamp: new Date().toISOString() },
          platform: { 
            isMobile: false, 
            isDesktop: true,
            hasVibration: false,
            hasWakeLock: false,
            hasDeviceOrientation: false,
            hasWebGL: true
          },
          featureFlags: {
            'advanced-analytics': false,
            'real-time-collaboration': false,
            'mobile-camera': false,
            'desktop-shortcuts': true,
            '3d-visualization': false,
            'notifications': true,
            'video-calling': false,
            'ai-suggestions': false
          },
          user: { type: 'free', isAdmin: true },
          abTests: { dashboardLayout: 'grid' }
        },
        props: { 
          userType: 'admin', 
          platform: 'desktop',
          deviceCapabilities: { vibration: false, wakeLock: false, orientation: false, camera: false, webgl: true },
          abTestVariant: 'grid'
        }
      }
    ];

    for (const { name, config, props } of configs) {
      console.log(`\n🧪 Testing: ${name}`);
      console.log('📋 Configuration:', JSON.stringify(config, null, 2));
      
      const optimizedCode = await optimizeWithWasm(transformedCode, config);
      console.log(`📄 Optimized code length: ${optimizedCode.length} chars`);
      
      try {
        const Component = evaluateComponent(optimizedCode);
        const html = renderComponent(Component, props);
        console.log(`🎨 Rendered HTML preview (first 300 chars):`);
        console.log(html.substring(0, 300) + '...');
      } catch (error) {
        console.error(`❌ Failed to render ${name}:`, error.message);
      }
    }
    
    // Use the first config for detailed output
    const detailedConfig = configs[0];
    console.log(`\n=== Detailed Output for: ${detailedConfig.name} ===`);
    const optimizedCode = await optimizeWithWasm(transformedCode, detailedConfig.config);
    console.log('📄 Full optimized code:');
    console.log(optimizedCode);
    console.log('\n');
    
    console.log('=== Component Evaluation & Rendering ===');
    const Component = evaluateComponent(optimizedCode);
    const html = renderComponent(Component, detailedConfig.props);
    console.log('🎨 Rendered HTML:');
    console.log(html);
    
    console.log('\n✅ Test completed successfully!');
    
  } catch (error) {
    console.error('❌ Test failed:', error);
    process.exit(1);
  }
}

// Run the tests
runTests(); 


