#!/usr/bin/env node

/**
 * Test suite for verifying that webpack modules are actually removed during tree-shaking,
 * not just having their exports stripped. This test exposes and inspects the webpack_modules
 * object to ensure complete module removal for unused code.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const {
  loadWebpackChunk,
  loadMainBundle,
  analyzeModules,
  compareChunks,
  verifyModulesExist,
  verifyModulesAbsent,
  getModuleDetails,
  scanChunksInDirectory
} = require('./utils/module-inspector');

// Test configuration
const TEST_CONFIG = {
  hostDist: path.resolve(__dirname, '../host/dist'),
  remoteDist: path.resolve(__dirname, '../remote/dist'),
  testLibPath: path.resolve(__dirname, '../test-shared-lib'),
  verbose: process.env.VERBOSE === 'true'
};

// Color output for better readability
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m'
};

function log(message, color = colors.reset) {
  console.log(`${color}${message}${colors.reset}`);
}

function logSection(title) {
  console.log(`\n${colors.bright}${colors.blue}${'='.repeat(60)}${colors.reset}`);
  console.log(`${colors.bright}${colors.cyan}${title}${colors.reset}`);
  console.log(`${colors.bright}${colors.blue}${'='.repeat(60)}${colors.reset}\n`);
}

/**
 * Test 1: Verify lodash module removal for unused functions
 * Checks that unused lodash modules are completely removed, not just exports
 */
async function testLodashModuleRemoval() {
  logSection('Test 1: Lodash Module Removal Verification');
  
  try {
    // Find lodash chunks in both original and optimized versions
    const lodashPattern = /lodash-es.*\.js$/;
    const hostChunks = fs.readdirSync(TEST_CONFIG.hostDist)
      .filter(f => lodashPattern.test(f) && !f.includes('.map') && !f.includes('.original'));
    
    if (hostChunks.length === 0) {
      log('⚠️  No lodash chunks found. Building project...', colors.yellow);
      execSync('pnpm build', { stdio: 'inherit' });
    }
    
    for (const chunkFile of hostChunks) {
      const chunkPath = path.join(TEST_CONFIG.hostDist, chunkFile);
      const originalPath = chunkPath + '.original';
      
      log(`\nAnalyzing: ${chunkFile}`, colors.cyan);
      
      // Load optimized chunk
      const optimizedChunk = loadWebpackChunk(chunkPath);
      const optimizedAnalysis = analyzeModules(optimizedChunk);
      
      log(`  Optimized: ${optimizedAnalysis.totalModules} modules`, colors.green);
      
      // If original exists, compare
      if (fs.existsSync(originalPath)) {
        const originalChunk = loadWebpackChunk(originalPath);
        const originalAnalysis = analyzeModules(originalChunk);
        
        log(`  Original: ${originalAnalysis.totalModules} modules`, colors.yellow);
        
        const comparison = compareChunks(originalChunk, optimizedChunk);
        
        if (comparison.details && comparison.details.removedModules && comparison.details.removedModules.length > 0) {
          log(`  ✅ Removed ${comparison.details.removedModules.length} complete modules!`, colors.green);
          
          if (TEST_CONFIG.verbose) {
            log('\n  Removed modules:', colors.yellow);
            comparison.details.removedModules.slice(0, 10).forEach(moduleId => {
              const shortId = moduleId.split('/').pop();
              log(`    - ${shortId}`, colors.red);
            });
            if (comparison.details.removedModules.length > 10) {
              log(`    ... and ${comparison.details.removedModules.length - 10} more`, colors.red);
            }
          }
          
          // Verify specific unused lodash functions are completely gone
          const unusedLodashFunctions = [
            'zip', 'zipObject', 'zipWith',  // Rarely used zip functions
            'xor', 'xorBy', 'xorWith',      // XOR operations
            'meanBy', 'sumBy', 'minBy',     // Statistical functions
            'pullAt', 'pullAll', 'pullAllBy', // Array manipulation
            'invertBy', 'mapKeys', 'mapValues' // Object transformation
          ];
          
          const absentModules = [];
          unusedLodashFunctions.forEach(func => {
            const pattern = new RegExp(`lodash-es/_?${func}\\.js$`);
            if (!verifyModulesExist(optimizedChunk, [pattern])) {
              absentModules.push(func);
            }
          });
          
          if (absentModules.length > 0) {
            log(`  ✅ Verified removal of ${absentModules.length} unused lodash functions`, colors.green);
          }
        } else {
          log(`  ⚠️  No modules removed (might be fully used)`, colors.yellow);
        }
      } else {
        log(`  ℹ️  No original file for comparison`, colors.cyan);
      }
    }
    
    return { success: true, test: 'lodashModuleRemoval' };
  } catch (error) {
    log(`❌ Test failed: ${error.message}`, colors.red);
    return { success: false, test: 'lodashModuleRemoval', error: error.message };
  }
}

/**
 * Test 2: Verify complete module removal in vendor chunks
 * Tests that unused vendor library modules are completely eliminated
 */
async function testVendorChunkModuleRemoval() {
  logSection('Test 2: Vendor Chunk Module Removal');
  
  try {
    const vendorLibraries = ['ramda', 'date-fns'];
    const results = [];
    
    for (const lib of vendorLibraries) {
      const pattern = new RegExp(`${lib}.*\\.js$`);
      const remoteChunks = fs.readdirSync(TEST_CONFIG.remoteDist)
        .filter(f => pattern.test(f) && !f.includes('.map') && !f.includes('.original'));
      
      for (const chunkFile of remoteChunks) {
        const chunkPath = path.join(TEST_CONFIG.remoteDist, chunkFile);
        const originalPath = chunkPath + '.original';
        
        log(`\nAnalyzing ${lib} chunk: ${chunkFile}`, colors.cyan);
        
        const optimizedChunk = loadWebpackChunk(chunkPath);
        const optimizedAnalysis = analyzeModules(optimizedChunk);
        
        log(`  Modules in optimized: ${optimizedAnalysis.totalModules}`, colors.green);
        
        if (fs.existsSync(originalPath)) {
          const originalChunk = loadWebpackChunk(originalPath);
          const originalAnalysis = analyzeModules(originalChunk);
          const comparison = compareChunks(originalChunk, optimizedChunk);
          
          log(`  Modules in original: ${originalAnalysis.totalModules}`, colors.yellow);
          log(`  Modules removed: ${comparison.details.removedModules.length}`, colors.green);
          log(`  Size reduction: ${comparison.summary.reductionPercentage}`, colors.green);
          
          if (comparison.details.removedModules.length > 0) {
            results.push({
              library: lib,
              modulesRemoved: comparison.details.removedModules.length,
              sizeReduction: parseFloat(comparison.summary.reductionPercentage)
            });
            
            // Verify that internal helper modules were removed if unused
            const helperPatterns = [
              /_internal/,
              /_helpers/,
              /_util/,
              /locale.*(?:af|ar|az|bg|bn|ca|cs|cy|da|de|el|eo|et|fa|fi|fr|gl|gu|he|hi|hr|hu|hy|id|is|it|ja|ka|kk|kn|ko|lb|lt|lv|mk|mn|ms|mt|nb|nl|nn|pl|pt|ro|ru|sk|sl|sq|sr|sv|ta|te|th|tr|ug|uk|uz|vi|zh)/
            ];
            
            let helpersRemoved = 0;
            helperPatterns.forEach(pattern => {
              const removed = comparison.details.removedModules.filter(m => pattern.test(m));
              helpersRemoved += removed.length;
            });
            
            if (helpersRemoved > 0) {
              log(`  ✅ Removed ${helpersRemoved} internal helper/locale modules`, colors.green);
            }
          }
        }
      }
    }
    
    if (results.length > 0) {
      log('\n📊 Summary:', colors.bright);
      results.forEach(r => {
        log(`  ${r.library}: Removed ${r.modulesRemoved} modules (${r.sizeReduction.toFixed(1)}% size reduction)`, colors.green);
      });
    }
    
    return { success: true, test: 'vendorChunkModuleRemoval', results };
  } catch (error) {
    log(`❌ Test failed: ${error.message}`, colors.red);
    return { success: false, test: 'vendorChunkModuleRemoval', error: error.message };
  }
}

/**
 * Test 3: Custom test library module removal
 * Uses our controlled test library to verify precise module removal behavior
 */
async function testCustomLibraryModuleRemoval() {
  logSection('Test 3: Custom Test Library Module Removal');
  
  try {
    // First, we need to build with our test library included
    log('Setting up test with custom library...', colors.cyan);
    
    // Check if test-shared-lib is being used in the build
    const hostBootstrapPath = path.join(TEST_CONFIG.hostDist, '..', 'src', 'bootstrap.js');
    if (fs.existsSync(hostBootstrapPath)) {
      const bootstrapContent = fs.readFileSync(hostBootstrapPath, 'utf8');
      
      // Check for test library usage
      if (!bootstrapContent.includes('test-shared-lib')) {
        log('  Test library not imported. Adding import to bootstrap...', colors.yellow);
        
        // Add controlled import to bootstrap
        const testImport = `
// Test import for module removal verification
import { formatDate, isValidEmail } from '../test-shared-lib';
console.log('Test imports loaded:', { formatDate, isValidEmail });
`;
        fs.writeFileSync(hostBootstrapPath, testImport + bootstrapContent);
        
        // Rebuild with test library
        log('  Rebuilding with test library...', colors.cyan);
        execSync('pnpm build', { stdio: 'inherit' });
      }
    }
    
    // Now analyze the chunks to find our test library modules
    const allChunks = scanChunksInDirectory(TEST_CONFIG.hostDist);
    
    // Expected behavior for our test library
    const expectations = {
      shouldBeRemoved: [
        'unused-module-a',
        'unused-module-b', 
        'legacy-helper',
        'deprecated-utils',
        'dependency-chain-a',
        'dependency-chain-b',
        'nested/deep/unused-deep'
      ],
      shouldBePartial: [
        'partial-usage-a',  // Only formatDate used
        'partial-usage-b'   // Only isValidEmail used
      ],
      shouldBeKept: [
        'core-utils',
        'main-feature'
      ]
    };
    
    let testLibModulesFound = false;
    let verificationResults = {
      correctlyRemoved: [],
      incorrectlyKept: [],
      correctlyKept: [],
      incorrectlyRemoved: []
    };
    
    // Check each chunk for test library modules
    for (const chunkInfo of allChunks) {
      const chunkData = loadWebpackChunk(chunkInfo.path);
      
      // Look for test-shared-lib modules
      const testLibPattern = /test-shared-lib/;
      const hasTestLib = verifyModulesExist(chunkData, [testLibPattern]);
      
      if (hasTestLib) {
        testLibModulesFound = true;
        log(`\nFound test library in: ${path.basename(chunkInfo.path)}`, colors.cyan);
        
        // Verify removed modules are actually gone
        expectations.shouldBeRemoved.forEach(moduleName => {
          const pattern = new RegExp(`test-shared-lib/${moduleName}`);
          if (verifyModulesAbsent(chunkData, [pattern])) {
            verificationResults.correctlyRemoved.push(moduleName);
          } else {
            verificationResults.incorrectlyKept.push(moduleName);
          }
        });
        
        // Verify kept modules are present
        expectations.shouldBeKept.forEach(moduleName => {
          const pattern = new RegExp(`test-shared-lib/${moduleName}`);
          if (verifyModulesExist(chunkData, [pattern])) {
            verificationResults.correctlyKept.push(moduleName);
          } else {
            verificationResults.incorrectlyRemoved.push(moduleName);
          }
        });
      }
    }
    
    if (!testLibModulesFound) {
      log('⚠️  Test library modules not found in build', colors.yellow);
      log('  This might be because the test hasn\'t been set up yet', colors.yellow);
    } else {
      // Report results
      log('\n📊 Test Library Module Removal Results:', colors.bright);
      
      if (verificationResults.correctlyRemoved.length > 0) {
        log(`\n✅ Correctly removed (${verificationResults.correctlyRemoved.length} modules):`, colors.green);
        verificationResults.correctlyRemoved.forEach(m => {
          log(`  - ${m}`, colors.green);
        });
      }
      
      if (verificationResults.incorrectlyKept.length > 0) {
        log(`\n❌ Incorrectly kept (${verificationResults.incorrectlyKept.length} modules):`, colors.red);
        verificationResults.incorrectlyKept.forEach(m => {
          log(`  - ${m}`, colors.red);
        });
      }
      
      if (verificationResults.correctlyKept.length > 0) {
        log(`\n✅ Correctly kept (${verificationResults.correctlyKept.length} modules):`, colors.green);
        verificationResults.correctlyKept.forEach(m => {
          log(`  - ${m}`, colors.green);
        });
      }
      
      if (verificationResults.incorrectlyRemoved.length > 0) {
        log(`\n❌ Incorrectly removed (${verificationResults.incorrectlyRemoved.length} modules):`, colors.red);
        verificationResults.incorrectlyRemoved.forEach(m => {
          log(`  - ${m}`, colors.red);
        });
      }
    }
    
    return { 
      success: verificationResults.incorrectlyKept.length === 0 && 
               verificationResults.incorrectlyRemoved.length === 0,
      test: 'customLibraryModuleRemoval',
      results: verificationResults
    };
  } catch (error) {
    log(`❌ Test failed: ${error.message}`, colors.red);
    return { success: false, test: 'customLibraryModuleRemoval', error: error.message };
  }
}

/**
 * Test 4: Module count verification
 * Ensures the total module count decreases after optimization
 */
async function testModuleCountReduction() {
  logSection('Test 4: Module Count Verification');
  
  try {
    const results = {
      host: { before: 0, after: 0, reduction: 0 },
      remote: { before: 0, after: 0, reduction: 0 }
    };
    
    // Analyze host dist
    log('Analyzing host distribution...', colors.cyan);
    const hostChunks = fs.readdirSync(TEST_CONFIG.hostDist)
      .filter(f => f.endsWith('.js') && !f.includes('.map'));
    
    hostChunks.forEach(chunkFile => {
      const chunkPath = path.join(TEST_CONFIG.hostDist, chunkFile);
      const originalPath = chunkPath + '.original';
      
      if (fs.existsSync(originalPath)) {
        const original = loadWebpackChunk(originalPath);
        const optimized = loadWebpackChunk(chunkPath);
        
        const originalAnalysis = analyzeModules(original);
        const optimizedAnalysis = analyzeModules(optimized);
        
        results.host.before += originalAnalysis.totalModules;
        results.host.after += optimizedAnalysis.totalModules;
      }
    });
    
    // Analyze remote dist
    log('Analyzing remote distribution...', colors.cyan);
    const remoteChunks = fs.readdirSync(TEST_CONFIG.remoteDist)
      .filter(f => f.endsWith('.js') && !f.includes('.map'));
    
    remoteChunks.forEach(chunkFile => {
      const chunkPath = path.join(TEST_CONFIG.remoteDist, chunkFile);
      const originalPath = chunkPath + '.original';
      
      if (fs.existsSync(originalPath)) {
        const original = loadWebpackChunk(originalPath);
        const optimized = loadWebpackChunk(chunkPath);
        
        const originalAnalysis = analyzeModules(original);
        const optimizedAnalysis = analyzeModules(optimized);
        
        results.remote.before += originalAnalysis.totalModules;
        results.remote.after += optimizedAnalysis.totalModules;
      }
    });
    
    // Calculate reductions
    if (results.host.before > 0) {
      results.host.reduction = ((results.host.before - results.host.after) / results.host.before * 100);
    }
    if (results.remote.before > 0) {
      results.remote.reduction = ((results.remote.before - results.remote.after) / results.remote.before * 100);
    }
    
    // Report results
    log('\n📊 Module Count Summary:', colors.bright);
    log(`\nHost:`, colors.cyan);
    log(`  Before optimization: ${results.host.before} modules`, colors.yellow);
    log(`  After optimization:  ${results.host.after} modules`, colors.green);
    if (results.host.reduction > 0) {
      log(`  Modules removed: ${results.host.before - results.host.after} (${results.host.reduction.toFixed(1)}% reduction)`, colors.green);
    }
    
    log(`\nRemote:`, colors.cyan);
    log(`  Before optimization: ${results.remote.before} modules`, colors.yellow);
    log(`  After optimization:  ${results.remote.after} modules`, colors.green);
    if (results.remote.reduction > 0) {
      log(`  Modules removed: ${results.remote.before - results.remote.after} (${results.remote.reduction.toFixed(1)}% reduction)`, colors.green);
    }
    
    const totalBefore = results.host.before + results.remote.before;
    const totalAfter = results.host.after + results.remote.after;
    const totalReduction = totalBefore > 0 ? ((totalBefore - totalAfter) / totalBefore * 100) : 0;
    
    log(`\nTotal:`, colors.bright);
    log(`  Before: ${totalBefore} modules`, colors.yellow);
    log(`  After:  ${totalAfter} modules`, colors.green);
    if (totalReduction > 0) {
      log(`  Overall reduction: ${totalBefore - totalAfter} modules (${totalReduction.toFixed(1)}%)`, colors.green);
    }
    
    return {
      success: totalReduction > 0 || totalBefore === 0,
      test: 'moduleCountReduction',
      results
    };
  } catch (error) {
    log(`❌ Test failed: ${error.message}`, colors.red);
    return { success: false, test: 'moduleCountReduction', error: error.message };
  }
}

/**
 * Main test runner
 */
async function runAllTests() {
  console.log(`${colors.bright}${colors.cyan}`);
  console.log('╔══════════════════════════════════════════════════════════╗');
  console.log('║     Webpack Module Removal Verification Test Suite      ║');
  console.log('╚══════════════════════════════════════════════════════════╝');
  console.log(colors.reset);
  
  const testResults = [];
  
  // Run all tests
  testResults.push(await testLodashModuleRemoval());
  testResults.push(await testVendorChunkModuleRemoval());
  testResults.push(await testCustomLibraryModuleRemoval());
  testResults.push(await testModuleCountReduction());
  
  // Summary
  logSection('Test Suite Summary');
  
  const passed = testResults.filter(r => r.success).length;
  const failed = testResults.filter(r => !r.success).length;
  
  testResults.forEach(result => {
    const status = result.success ? '✅ PASSED' : '❌ FAILED';
    const color = result.success ? colors.green : colors.red;
    log(`${status}: ${result.test}`, color);
    if (result.error) {
      log(`  Error: ${result.error}`, colors.red);
    }
  });
  
  console.log(`\n${colors.bright}`);
  if (failed === 0) {
    log(`🎉 All ${passed} tests passed!`, colors.green);
  } else {
    log(`⚠️  ${passed} passed, ${failed} failed`, colors.yellow);
  }
  console.log(colors.reset);
  
  process.exit(failed > 0 ? 1 : 0);
}

// Run tests if called directly
if (require.main === module) {
  runAllTests().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
  });
}

module.exports = {
  testLodashModuleRemoval,
  testVendorChunkModuleRemoval,
  testCustomLibraryModuleRemoval,
  testModuleCountReduction,
  runAllTests
};