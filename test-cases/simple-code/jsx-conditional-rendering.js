const React = require('react');

function ComplexAppRouter({ userType = "free", platform = "desktop", deviceCapabilities = {}, abTestVariant = "grid" }) {
  
  return (
    <div style={{ maxWidth: '800px', margin: '0 auto', padding: '20px', fontFamily: 'Arial, sans-serif' }}>
      
      {/* Navigation component */}
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
        <div style={{ fontSize: '14px', color: '#6c757d' }}>
          📱 Mobile Interface Active
          {/* @common:if [condition="platform.hasVibration"] */}
          <span style={{ marginLeft: '10px', color: '#28a745' }}>
            ✨ Haptic feedback available
          </span>
          {/* @common:endif */}
        </div>
        {/* @common:endif */}
        
        {/* @common:if [condition="platform.isDesktop"] */}
        <div style={{ fontSize: '14px', color: '#6c757d' }}>
          🖥️ Desktop Interface Active
          {/* @common:if [condition="featureFlags.hasDesktopShortcuts"] */}
          <span style={{ marginLeft: '10px', color: '#007bff' }}>
            ⌨️ Keyboard shortcuts enabled
          </span>
          {/* @common:endif */}
        </div>
        {/* @common:endif */}
      </nav>

      {/* Dashboard Layout - Grid Version */}
      {/* @common:if [condition="abTests.isGridLayout"] */}
      <div style={{ padding: '15px', border: '1px solid #e9ecef', borderRadius: '8px', marginBottom: '20px' }}>
        <h3 style={{ color: '#343a40', marginBottom: '15px' }}>
          Dashboard (grid layout)
        </h3>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '15px' }}>
          <div style={{ padding: '10px', backgroundColor: '#e3f2fd', borderRadius: '6px' }}>
            <strong>Grid Item 1</strong>
            <p style={{ fontSize: '12px', margin: '5px 0' }}>Grid layout active</p>
          </div>
          <div style={{ padding: '10px', backgroundColor: '#f3e5f5', borderRadius: '6px' }}>
            <strong>Grid Item 2</strong>
            <p style={{ fontSize: '12px', margin: '5px 0' }}>Optimized for desktop</p>
          </div>
          {/* @common:if [condition="featureFlags.hasAdvancedAnalytics"] */}
          <div style={{ padding: '10px', backgroundColor: '#e8f5e8', borderRadius: '6px' }}>
            <strong>Analytics Panel</strong>
            <p style={{ fontSize: '12px', margin: '5px 0' }}>Premium analytics enabled</p>
            {/* @common:if [condition="featureFlags.has3dVisualization"] */}
            <div style={{ marginTop: '8px', fontSize: '11px', color: '#28a745' }}>
              🎯 3D visualization ready
            </div>
            {/* @common:endif */}
          </div>
          {/* @common:endif */}
        </div>
      </div>
      {/* @common:endif */}

      {/* Dashboard Layout - List Version */}
      {/* @common:if [condition="abTests.isListLayout"] */}
      <div style={{ padding: '15px', border: '1px solid #e9ecef', borderRadius: '8px', marginBottom: '20px' }}>
        <h3 style={{ color: '#343a40', marginBottom: '15px' }}>
          Dashboard (list layout)
        </h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ padding: '12px', backgroundColor: '#fff3cd', border: '1px solid #ffeaa7', borderRadius: '6px' }}>
            <strong>List Item 1</strong> - Linear layout active
          </div>
          <div style={{ padding: '12px', backgroundColor: '#d1ecf1', border: '1px solid #bee5eb', borderRadius: '6px' }}>
            <strong>List Item 2</strong> - Mobile-optimized
          </div>
          {/* @common:if [condition="featureFlags.hasAdvancedAnalytics"] */}
          <div style={{ padding: '12px', backgroundColor: '#d4edda', border: '1px solid #c3e6cb', borderRadius: '6px' }}>
            <strong>Analytics List View</strong> - Premium features
          </div>
          {/* @common:endif */}
        </div>
      </div>
      {/* @common:endif */}

      {/* Mobile Platform Features */}
      {/* @common:if [condition="platform.isMobile"] */}
      <div style={{ marginBottom: '20px' }}>
        <div style={{ padding: '15px', backgroundColor: '#f8d7da', border: '1px solid #f5c6cb', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#721c24' }}>📱 Mobile Features</h4>
          
          {/* @common:if [condition="featureFlags.hasMobileCamera"] */}
          <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#d1ecf1', borderRadius: '4px' }}>
            📷 Camera access enabled
            {/* @common:if [condition="platform.hasWakeLock"] */}
            <div style={{ fontSize: '12px', color: '#0c5460', marginTop: '4px' }}>
              🔒 Screen wake lock available
            </div>
            {/* @common:endif */}
          </div>
          {/* @common:endif */}
          
          {/* @common:if [condition="platform.hasDeviceOrientation"] */}
          <div style={{ padding: '8px', backgroundColor: '#fff3cd', borderRadius: '4px' }}>
            🧭 Device orientation: portrait
          </div>
          {/* @common:endif */}
        </div>
      </div>
      {/* @common:endif */}

      {/* Desktop Platform Features */}
      {/* @common:if [condition="platform.isDesktop"] */}
      <div style={{ marginBottom: '20px' }}>
        <div style={{ padding: '15px', backgroundColor: '#d4edda', border: '1px solid #c3e6cb', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#155724' }}>🖥️ Desktop Features</h4>
          
          {/* @common:if [condition="featureFlags.hasDesktopShortcuts"] */}
          <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#e2e3e5', borderRadius: '4px' }}>
            ⌨️ Keyboard shortcuts active
            {/* @common:if [condition="platform.hasWebGL"] */}
            <div style={{ fontSize: '12px', color: '#383d41', marginTop: '4px' }}>
              🎮 WebGL acceleration enabled
            </div>
            {/* @common:endif */}
          </div>
          {/* @common:endif */}
          
          {/* @common:if [condition="featureFlags.hasAdvancedAnalytics"] */}
          <div style={{ padding: '8px', backgroundColor: '#cce5ff', borderRadius: '4px' }}>
            📊 Desktop analytics dashboard
          </div>
          {/* @common:endif */}
        </div>
      </div>
      {/* @common:endif */}

      {/* Enterprise User Features */}
      {/* @common:if [condition="user.isEnterprise"] */}
      <div style={{ marginBottom: '20px' }}>
        <div style={{ padding: '15px', backgroundColor: '#e7f3ff', border: '1px solid #b3d9ff', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#004085' }}>🏢 Enterprise Features</h4>
          
          {/* @common:if [condition="featureFlags.hasCollaboration"] */}
          <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#d1f2eb', borderRadius: '4px' }}>
            👥 Real-time collaboration enabled
            {/* @common:if [condition="featureFlags.hasVideoCalling"] */}
            <div style={{ fontSize: '12px', color: '#0e6655', marginTop: '4px' }}>
              📹 Video calling available
            </div>
            {/* @common:endif */}
          </div>
          {/* @common:endif */}
          
          {/* @common:if [condition="featureFlags.hasAdvancedAnalytics"] */}
          <div style={{ padding: '8px', backgroundColor: '#fff2cc', borderRadius: '4px' }}>
            📈 Enterprise analytics & reporting
          </div>
          {/* @common:endif */}
        </div>
      </div>
      {/* @common:endif */}

      {/* Premium User Features */}
      {/* @common:if [condition="user.isPremium"] */}
      <div style={{ marginBottom: '20px' }}>
        <div style={{ padding: '15px', backgroundColor: '#fff0f5', border: '1px solid #ffb3d1', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#6d1650' }}>⭐ Premium Features</h4>
          
          {/* @common:if [condition="featureFlags.hasAdvancedAnalytics"] */}
          <div style={{ marginBottom: '10px', padding: '8px', backgroundColor: '#e1f5fe', borderRadius: '4px' }}>
            📊 Advanced analytics
          </div>
          {/* @common:endif */}
          
          {/* @common:if [condition="featureFlags.hasAiSuggestions"] */}
          <div style={{ padding: '8px', backgroundColor: '#f3e5f5', borderRadius: '4px' }}>
            🤖 AI-powered suggestions
          </div>
          {/* @common:endif */}
        </div>
      </div>
      {/* @common:endif */}

      {/* Free User Features */}
      {/* @common:if [condition="user.isFree"] */}
      <div style={{ marginBottom: '20px' }}>
        <div style={{ padding: '15px', backgroundColor: '#f8f9fa', border: '1px solid #dee2e6', borderRadius: '8px' }}>
          <h4 style={{ margin: '0 0 10px 0', color: '#495057' }}>🆓 Free Tier</h4>
          <div style={{ padding: '8px', backgroundColor: '#e9ecef', borderRadius: '4px' }}>
            Basic features available. Upgrade for premium features!
          </div>
        </div>
      </div>
      {/* @common:endif */}
      
      {/* Admin Panel */}
      {/* @common:if [condition="user.isAdmin"] */}
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
      {/* @common:endif */}

      {/* Notifications */}
      {/* @common:if [condition="featureFlags.hasNotifications"] */}
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
        {/* @common:if [condition="platform.hasVibration"] */}
        <div style={{ marginTop: '4px', fontSize: '10px' }}>
          📳 With haptic feedback
        </div>
        {/* @common:endif */}
      </div>
      {/* @common:endif */}

      {/* Build Configuration */}
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
  timestamp: new Date().toISOString(),
  target: 'production',
  platform,
  userType,
  abTestVariant
}, null, 2)}
        </pre>
      </div>
    </div>
  );
}

module.exports = ComplexAppRouter;
