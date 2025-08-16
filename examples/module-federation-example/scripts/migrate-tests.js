#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const rootDir = path.resolve(__dirname, '..');

console.log('🔄 Migrating legacy tests to Vitest...\n');

// Map of old test files to new test categories
const testMigrationMap = {
  'scripts/test-exports-functionality.js': 'test/integration/exports-functionality.test.js',
  'scripts/test-host.js': 'test/e2e/host-app.test.js',
  'scripts/test-double-optimization.js': 'test/unit/double-optimization.test.js',
  'scripts/test-identical-exports.js': 'test/unit/identical-exports.test.js',
  'scripts/test-rust-parity.js': 'test/integration/rust-parity.test.js',
  'scripts/verify-optimized-exports.js': 'test/integration/optimized-exports.test.js',
  'scripts/test-functional-optimization.js': 'test/integration/functional-optimization.test.js',
  'scripts/test-normal-lodash-chunk.js': 'test/unit/standard-lodash.test.js'
};

// Helper to convert old test to Vitest format
function convertToVitest(oldContent, testName) {
  const vitestTemplate = `import { describe, it, expect, beforeAll } from 'vitest';
const fs = require('fs');
const path = require('path');

describe('${testName}', () => {
  // Migrated from legacy test
  ${extractTestLogic(oldContent)}
});

${extractHelperFunctions(oldContent)}
`;

  return vitestTemplate;
}

// Extract test logic from old format
function extractTestLogic(content) {
  // Find the main test function
  const functionMatch = content.match(/async function \w+\(\)\s*{([\s\S]*?)^}/m);
  if (!functionMatch) {
    return '// TODO: Manually migrate test logic';
  }
  
  const logic = functionMatch[1]
    .split('\n')
    .map(line => '  ' + line)
    .join('\n');
  
  return `it('should pass legacy test', async () => {
${logic}
  });`;
}

// Extract helper functions
function extractHelperFunctions(content) {
  const helpers = [];
  const functionRegex = /^function (\w+)\s*\([^)]*\)\s*{[\s\S]*?^}/gm;
  
  let match;
  while ((match = functionRegex.exec(content)) !== null) {
    if (!match[0].includes('async function test')) {
      helpers.push(match[0]);
    }
  }
  
  return helpers.join('\n\n');
}

// Create legacy tests directory for reference
const legacyDir = path.join(rootDir, 'test/legacy');
if (!fs.existsSync(legacyDir)) {
  fs.mkdirSync(legacyDir, { recursive: true });
}

// Process each test file
Object.entries(testMigrationMap).forEach(([oldPath, newPath]) => {
  const fullOldPath = path.join(rootDir, oldPath);
  const fullNewPath = path.join(rootDir, newPath);
  
  if (fs.existsSync(fullOldPath)) {
    console.log(`📁 Migrating ${path.basename(oldPath)}...`);
    
    // Read old test
    const oldContent = fs.readFileSync(fullOldPath, 'utf-8');
    
    // Extract test name from filename
    const testName = path.basename(oldPath, '.js')
      .replace(/^test-/, '')
      .replace(/-/g, ' ')
      .replace(/\b\w/g, l => l.toUpperCase());
    
    // Convert to Vitest format
    const newContent = convertToVitest(oldContent, testName);
    
    // Ensure directory exists
    const newDir = path.dirname(fullNewPath);
    if (!fs.existsSync(newDir)) {
      fs.mkdirSync(newDir, { recursive: true });
    }
    
    // Write new test
    fs.writeFileSync(fullNewPath, newContent);
    console.log(`  ✅ Created ${path.relative(rootDir, fullNewPath)}`);
    
    // Copy original to legacy directory
    const legacyPath = path.join(legacyDir, path.basename(oldPath));
    fs.copyFileSync(fullOldPath, legacyPath);
    console.log(`  📋 Backed up to ${path.relative(rootDir, legacyPath)}`);
  } else {
    console.log(`  ⚠️  ${oldPath} not found`);
  }
});

// Create a migration summary
const summaryPath = path.join(legacyDir, 'MIGRATION.md');
const summary = `# Test Migration Summary

This directory contains the original test files before migration to Vitest.

## Migration Date
${new Date().toISOString()}

## Files Migrated
${Object.entries(testMigrationMap).map(([old, newPath]) => 
  `- \`${old}\` → \`${newPath}\``
).join('\n')}

## Next Steps
1. Review the migrated tests in the \`test/\` directory
2. Update test logic to use Vitest assertions
3. Add proper setup/teardown hooks
4. Remove console.log statements in favor of test assertions
5. Add more comprehensive test cases

## Running Tests
\`\`\`bash
# Run all tests
pnpm test

# Run specific category
pnpm test:unit
pnpm test:integration
pnpm test:e2e
\`\`\`
`;

fs.writeFileSync(summaryPath, summary);
console.log(`\n📝 Migration summary written to ${path.relative(rootDir, summaryPath)}`);

console.log('\n✅ Test migration complete!');
console.log('   Please review and update the migrated tests.');
console.log('   Original files are preserved in test/legacy/');