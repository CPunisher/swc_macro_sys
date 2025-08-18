// Test file to verify CommonJS module federation setup
// This test imports the host's main.js and logs it to console

try {
  // Import the host's main.js using CommonJS require
  const hostMain = require('../host/dist/main.js');
  
  console.log('Successfully imported host main.js:');
  console.log('Host main module:', hostMain);
  
  // Log the type and properties of the imported module
  console.log('Type:', typeof hostMain);
  console.log('Properties:', Object.keys(hostMain));
  
  // If it's a function, try to call it
  if (typeof hostMain === 'function') {
    console.log('Executing host main function...');
    const result = hostMain();
    console.log('Function result:', result);
  }
  
  console.log('✅ Host import test completed successfully');
} catch (error) {
  console.error('❌ Error importing host main.js:', error.message);
  console.error('Stack trace:', error.stack);
  process.exit(1);
}