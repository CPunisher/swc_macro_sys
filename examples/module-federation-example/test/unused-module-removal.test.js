#!/usr/bin/env node

/**
 * Test suite specifically for verifying complete removal of unused modules.
 * These tests should FAIL until the optimizer is fixed to remove entire modules.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const {
  loadWebpackChunk,
  analyzeModules,
  compareChunks,
  verifyModulesAbsent
} = require('./utils/module-inspector');

// Test configuration
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
 * Test 1: Verify unused lodash modules are completely removed
 * We only use sortBy and uniq, so hundreds of other lodash modules should be removed
 */
async function testUnusedLodashModulesRemoved() {
  logSection('Test 1: Unused Lodash Modules Should Be Removed');
  
  try {
    const hostDist = path.resolve(__dirname, '../host/dist');
    const lodashChunk = fs.readdirSync(hostDist)
      .find(f => f.includes('lodash-es') && f.endsWith('.js') && !f.includes('.original'));
    
    if (!lodashChunk) {
      throw new Error('No lodash chunk found');
    }
    
    const chunkPath = path.join(hostDist, lodashChunk);
    const originalPath = chunkPath + '.original';
    
    // Load chunks
    const optimizedChunk = loadWebpackChunk(chunkPath);
    const optimizedAnalysis = analyzeModules(optimizedChunk);
    
    // List of lodash functions we DON'T use (should be removed)
    const unusedLodashFunctions = [
      'camelCase', 'capitalize', 'deburr', 'endsWith', 'escape',
      'escapeRegExp', 'kebabCase', 'lowerCase', 'lowerFirst', 'pad',
      'padEnd', 'padStart', 'parseInt', 'repeat', 'replace',
      'snakeCase', 'split', 'startCase', 'startsWith', 'template',
      'toLower', 'toUpper', 'trim', 'trimEnd', 'trimStart',
      'truncate', 'unescape', 'upperCase', 'upperFirst', 'words',
      // Array functions we don't use
      'chunk', 'compact', 'concat', 'difference', 'differenceBy',
      'differenceWith', 'drop', 'dropRight', 'dropRightWhile', 'dropWhile',
      'fill', 'findIndex', 'findLastIndex', 'first', 'flatten',
      'flattenDeep', 'flattenDepth', 'fromPairs', 'head', 'indexOf',
      'initial', 'intersection', 'intersectionBy', 'intersectionWith',
      'join', 'last', 'lastIndexOf', 'nth', 'pull', 'pullAll',
      'pullAllBy', 'pullAllWith', 'pullAt', 'remove', 'reverse',
      'slice', 'tail', 'take', 'takeRight', 'takeRightWhile',
      'takeWhile', 'union', 'unionBy', 'unionWith', 'uniqBy',
      'uniqWith', 'unzip', 'unzipWith', 'without', 'xor',
      'xorBy', 'xorWith', 'zip', 'zipObject', 'zipObjectDeep', 'zipWith'
    ];
    
    log(`Checking ${unusedLodashFunctions.length} unused lodash functions...`, colors.cyan);
    
    let foundUnusedModules = 0;
    let missingUnusedModules = 0;
    
    unusedLodashFunctions.forEach(funcName => {
      const modulePatterns = [
        new RegExp(`lodash-es/${funcName}\\.js$`),
        new RegExp(`lodash-es/_${funcName}\\.js$`)
      ];
      
      let found = false;
      modulePatterns.forEach(pattern => {
        const moduleIds = Object.keys(optimizedChunk.modules || {});
        if (moduleIds.some(id => pattern.test(id))) {
          found = true;
        }
      });
      
      if (found) {
        foundUnusedModules++;
        log(`  ❌ Found unused module: ${funcName} (should be removed)`, colors.red);
      } else {
        missingUnusedModules++;
      }
    });
    
    log(`\nResults:`, colors.bright);
    log(`  Total modules in chunk: ${optimizedAnalysis.totalModules}`, colors.yellow);
    log(`  Unused modules still present: ${foundUnusedModules}`, colors.red);
    log(`  Unused modules correctly absent: ${missingUnusedModules}`, colors.green);
    
    // Calculate what the module count SHOULD be
    // We only use sortBy and uniq, which have dependencies
    const expectedMaxModules = 50; // sortBy, uniq, and their dependencies
    
    if (optimizedAnalysis.totalModules > expectedMaxModules) {
      log(`\n❌ TEST FAILED: Chunk has ${optimizedAnalysis.totalModules} modules but should have ≤ ${expectedMaxModules}`, colors.red);
      log(`  This means ${optimizedAnalysis.totalModules - expectedMaxModules} unused modules are not being removed!`, colors.red);
      return { success: false, test: 'unusedLodashModulesRemoved', foundUnused: foundUnusedModules };
    }
    
    if (foundUnusedModules > 0) {
      log(`\n❌ TEST FAILED: Found ${foundUnusedModules} unused modules that should be removed`, colors.red);
      return { success: false, test: 'unusedLodashModulesRemoved', foundUnused: foundUnusedModules };
    }
    
    log(`\n✅ TEST PASSED: All unused modules removed!`, colors.green);
    return { success: true, test: 'unusedLodashModulesRemoved' };
    
  } catch (error) {
    log(`❌ Test error: ${error.message}`, colors.red);
    return { success: false, test: 'unusedLodashModulesRemoved', error: error.message };
  }
}

/**
 * Test 2: Verify date-fns locale modules are removed when not used
 * We don't use any locales, so all locale modules should be removed
 */
async function testUnusedDateFnsLocalesRemoved() {
  logSection('Test 2: Unused date-fns Locale Modules Should Be Removed');
  
  try {
    const remoteDist = path.resolve(__dirname, '../remote/dist');
    const dateFnsChunk = fs.readdirSync(remoteDist)
      .find(f => f.includes('date-fns') && f.endsWith('.js') && !f.includes('.original'));
    
    if (!dateFnsChunk) {
      log('⚠️  No date-fns chunk found (might not be used)', colors.yellow);
      return { success: true, test: 'unusedDateFnsLocalesRemoved', skipped: true };
    }
    
    const chunkPath = path.join(remoteDist, dateFnsChunk);
    const optimizedChunk = loadWebpackChunk(chunkPath);
    const moduleIds = Object.keys(optimizedChunk.modules || {});
    
    // Check for locale modules
    const localePattern = /date-fns\/locale\//;
    const localeModules = moduleIds.filter(id => localePattern.test(id));
    
    log(`Found ${localeModules.length} locale modules in optimized chunk`, colors.cyan);
    
    if (localeModules.length > 0) {
      log(`\nLocale modules found (should be removed):`, colors.red);
      localeModules.slice(0, 10).forEach(id => {
        const locale = id.match(/locale\/([^/]+)/)?.[1] || 'unknown';
        log(`  ❌ ${locale}`, colors.red);
      });
      if (localeModules.length > 10) {
        log(`  ... and ${localeModules.length - 10} more`, colors.red);
      }
      
      log(`\n❌ TEST FAILED: Found ${localeModules.length} unused locale modules`, colors.red);
      return { success: false, test: 'unusedDateFnsLocalesRemoved', foundLocales: localeModules.length };
    }
    
    log(`✅ TEST PASSED: No unused locale modules found!`, colors.green);
    return { success: true, test: 'unusedDateFnsLocalesRemoved' };
    
  } catch (error) {
    log(`❌ Test error: ${error.message}`, colors.red);
    return { success: false, test: 'unusedDateFnsLocalesRemoved', error: error.message };
  }
}

/**
 * Test 3: Verify ramda internal modules are removed when not used
 * Many ramda internals should be removed if the exported functions aren't used
 */
async function testUnusedRamdaInternalsRemoved() {
  logSection('Test 3: Unused Ramda Internal Modules Should Be Removed');
  
  try {
    const remoteDist = path.resolve(__dirname, '../remote/dist');
    const ramdaChunk = fs.readdirSync(remoteDist)
      .find(f => f.includes('ramda') && f.endsWith('.js') && !f.includes('.original'));
    
    if (!ramdaChunk) {
      log('⚠️  No ramda chunk found (might not be used)', colors.yellow);
      return { success: true, test: 'unusedRamdaInternalsRemoved', skipped: true };
    }
    
    const chunkPath = path.join(remoteDist, ramdaChunk);
    const optimizedChunk = loadWebpackChunk(chunkPath);
    const moduleIds = Object.keys(optimizedChunk.modules || {});
    
    // Check for internal helper modules
    const internalPattern = /ramda\/es\/internal\//;
    const internalModules = moduleIds.filter(id => internalPattern.test(id));
    
    log(`Found ${internalModules.length} internal modules in optimized chunk`, colors.cyan);
    log(`Total modules in chunk: ${moduleIds.length}`, colors.yellow);
    
    // Ramda has ~250 internal modules, if none are used, most should be removed
    const expectedMaxInternals = 50; // Conservative estimate for actually used internals
    
    if (internalModules.length > expectedMaxInternals) {
      log(`\n❌ TEST FAILED: Found ${internalModules.length} internal modules, expected ≤ ${expectedMaxInternals}`, colors.red);
      log(`  Sample unused internals that should be removed:`, colors.red);
      internalModules.slice(0, 5).forEach(id => {
        const internal = id.split('/').pop().replace('.js', '');
        log(`    - ${internal}`, colors.red);
      });
      return { success: false, test: 'unusedRamdaInternalsRemoved', foundInternals: internalModules.length };
    }
    
    log(`✅ TEST PASSED: Internal modules properly pruned!`, colors.green);
    return { success: true, test: 'unusedRamdaInternalsRemoved' };
    
  } catch (error) {
    log(`❌ Test error: ${error.message}`, colors.red);
    return { success: false, test: 'unusedRamdaInternalsRemoved', error: error.message };
  }
}

/**
 * Test 4: Create a test case with completely isolated unused modules
 * This tests the most basic case - modules with zero imports/exports usage
 */
async function testCompletelyUnusedModules() {
  logSection('Test 4: Completely Unused Modules Should Be Removed');
  
  try {
    // First, let's check if we have any vendor chunks with known unused modules
    const remoteDist = path.resolve(__dirname, '../remote/dist');
    const hostDist = path.resolve(__dirname, '../host/dist');
    
    // Check total module count across all chunks
    let totalModulesBefore = 0;
    let totalModulesAfter = 0;
    
    const allDists = [hostDist, remoteDist];
    
    for (const distDir of allDists) {
      const chunks = fs.readdirSync(distDir)
        .filter(f => f.endsWith('.js') && !f.includes('.map'));
      
      for (const chunkFile of chunks) {
        const chunkPath = path.join(distDir, chunkFile);
        const originalPath = chunkPath + '.original';
        
        if (fs.existsSync(originalPath)) {
          const original = loadWebpackChunk(originalPath);
          const optimized = loadWebpackChunk(chunkPath);
          
          totalModulesBefore += Object.keys(original.modules || {}).length;
          totalModulesAfter += Object.keys(optimized.modules || {}).length;
        }
      }
    }
    
    const modulesRemoved = totalModulesBefore - totalModulesAfter;
    const removalPercentage = totalModulesBefore > 0 
      ? ((modulesRemoved / totalModulesBefore) * 100).toFixed(1)
      : 0;
    
    log(`Module removal statistics:`, colors.cyan);
    log(`  Before optimization: ${totalModulesBefore} modules`, colors.yellow);
    log(`  After optimization: ${totalModulesAfter} modules`, colors.yellow);
    log(`  Modules removed: ${modulesRemoved} (${removalPercentage}%)`, colors.yellow);
    
    // We should be removing at least 30% of modules in a typical vendor bundle
    const expectedMinRemovalPercentage = 30;
    
    if (parseFloat(removalPercentage) < expectedMinRemovalPercentage) {
      log(`\n❌ TEST FAILED: Only ${removalPercentage}% of modules removed, expected ≥ ${expectedMinRemovalPercentage}%`, colors.red);
      log(`  The optimizer is not removing unused modules!`, colors.red);
      return { success: false, test: 'completelyUnusedModules', removalPercentage };
    }
    
    log(`\n✅ TEST PASSED: Removed ${removalPercentage}% of modules!`, colors.green);
    return { success: true, test: 'completelyUnusedModules', removalPercentage };
    
  } catch (error) {
    log(`❌ Test error: ${error.message}`, colors.red);
    return { success: false, test: 'completelyUnusedModules', error: error.message };
  }
}

/**
 * Test 5: Verify transitive dependency removal
 * If module A is unused and only module B imports A, both should be removed
 */
async function testTransitiveDependencyRemoval() {
  logSection('Test 5: Transitive Dependencies Should Be Removed');
  
  try {
    // Look for modules that are only used by other unused modules
    const hostDist = path.resolve(__dirname, '../host/dist');
    const lodashChunk = fs.readdirSync(hostDist)
      .find(f => f.includes('lodash-es') && f.endsWith('.js') && !f.includes('.original'));
    
    if (!lodashChunk) {
      throw new Error('No lodash chunk found');
    }
    
    const chunkPath = path.join(hostDist, lodashChunk);
    const optimizedChunk = loadWebpackChunk(chunkPath);
    const moduleIds = Object.keys(optimizedChunk.modules || {});
    
    // Check for helper modules that are only used by unused functions
    // For example, _baseIteratee is used by many functions we don't use
    const transitiveHelpers = [
      '_baseIteratee', '_baseAssignValue', '_baseFor', '_baseForOwn',
      '_baseGet', '_baseSet', '_baseUniq', '_baseFlatten',
      '_baseIntersection', '_baseDifference', '_basePullAll',
      '_baseWhile', '_baseZipObject'
    ];
    
    let foundTransitiveModules = 0;
    
    transitiveHelpers.forEach(helper => {
      const pattern = new RegExp(`lodash-es/${helper}\\.js$`);
      if (moduleIds.some(id => pattern.test(id))) {
        foundTransitiveModules++;
        log(`  ❌ Found transitive dependency: ${helper}`, colors.red);
      }
    });
    
    if (foundTransitiveModules > 0) {
      log(`\n❌ TEST FAILED: Found ${foundTransitiveModules} transitive dependencies that should be removed`, colors.red);
      log(`  These modules are only used by other unused modules`, colors.red);
      return { success: false, test: 'transitiveDependencyRemoval', foundTransitive: foundTransitiveModules };
    }
    
    log(`✅ TEST PASSED: Transitive dependencies properly removed!`, colors.green);
    return { success: true, test: 'transitiveDependencyRemoval' };
    
  } catch (error) {
    log(`❌ Test error: ${error.message}`, colors.red);
    return { success: false, test: 'transitiveDependencyRemoval', error: error.message };
  }
}

/**
 * Main test runner
 */
async function runAllTests() {
  console.log(`${colors.bright}${colors.cyan}`);
  console.log('╔══════════════════════════════════════════════════════════╗');
  console.log('║    Unused Module Removal Test Suite (Should Fail!)      ║');
  console.log('║  These tests verify that unused modules are removed     ║');
  console.log('╚══════════════════════════════════════════════════════════╝');
  console.log(colors.reset);
  
  log(`\n⚠️  These tests are expected to FAIL until the optimizer is fixed!`, colors.yellow);
  log(`They demonstrate that unused modules are NOT being removed.\n`, colors.yellow);
  
  const testResults = [];
  
  // Ensure build is up to date
  log('Building and optimizing project...', colors.cyan);
  try {
    execSync('pnpm build && pnpm optimize', { stdio: 'inherit' });
  } catch (error) {
    log('Build/optimize failed, continuing with existing files...', colors.yellow);
  }
  
  // Run all tests
  testResults.push(await testUnusedLodashModulesRemoved());
  testResults.push(await testUnusedDateFnsLocalesRemoved());
  testResults.push(await testUnusedRamdaInternalsRemoved());
  testResults.push(await testCompletelyUnusedModules());
  testResults.push(await testTransitiveDependencyRemoval());
  
  // Summary
  logSection('Test Suite Summary');
  
  const passed = testResults.filter(r => r.success).length;
  const failed = testResults.filter(r => !r.success).length;
  const skipped = testResults.filter(r => r.skipped).length;
  
  testResults.forEach(result => {
    if (result.skipped) {
      log(`⏭️  SKIPPED: ${result.test}`, colors.yellow);
    } else {
      const status = result.success ? '✅ PASSED' : '❌ FAILED';
      const color = result.success ? colors.green : colors.red;
      log(`${status}: ${result.test}`, color);
      if (result.error) {
        log(`  Error: ${result.error}`, colors.red);
      }
    }
  });
  
  console.log(`\n${colors.bright}`);
  if (failed > 0) {
    log(`❌ ${failed} tests failed (as expected - optimizer needs fixing!)`, colors.red);
    log(`   The optimizer is NOT removing unused modules, only stripping exports.`, colors.yellow);
    log(`   Module count stays the same while file size reduces ~35-40%.`, colors.yellow);
  } else if (passed === testResults.length - skipped) {
    log(`🎉 All ${passed} tests passed! The optimizer is properly removing unused modules!`, colors.green);
  }
  console.log(colors.reset);
  
  // Exit with error code to indicate tests failed (expected)
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
  testUnusedLodashModulesRemoved,
  testUnusedDateFnsLocalesRemoved,
  testUnusedRamdaInternalsRemoved,
  testCompletelyUnusedModules,
  testTransitiveDependencyRemoval,
  runAllTests
};