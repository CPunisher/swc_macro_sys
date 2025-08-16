#!/usr/bin/env node

import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import path from 'path';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

console.log(`
╔══════════════════════════════════════════════╗
║   Module Federation Test Suite               ║
║   Powered by Vitest                          ║
╚══════════════════════════════════════════════╝
`);

const args = process.argv.slice(2);
const command = args[0];

const commands = {
  all: {
    desc: 'Run all tests',
    cmd: 'vitest run'
  },
  unit: {
    desc: 'Run unit tests',
    cmd: 'vitest run test/unit'
  },
  integration: {
    desc: 'Run integration tests',
    cmd: 'vitest run test/integration'
  },
  e2e: {
    desc: 'Run E2E tests',
    cmd: 'vitest run test/e2e'
  },
  performance: {
    desc: 'Run performance benchmarks',
    cmd: 'vitest run test/performance'
  },
  watch: {
    desc: 'Run tests in watch mode',
    cmd: 'vitest'
  },
  ui: {
    desc: 'Open Vitest UI',
    cmd: 'vitest --ui'
  },
  coverage: {
    desc: 'Run tests with coverage',
    cmd: 'vitest --coverage'
  },
  build: {
    desc: 'Build and optimize before testing',
    cmd: 'pnpm run build:optimized && vitest run'
  }
};

function showHelp() {
  console.log('Available commands:\n');
  Object.entries(commands).forEach(([name, { desc }]) => {
    console.log(`  ${name.padEnd(12)} ${desc}`);
  });
  console.log(`
Examples:
  node test.js               # Run all tests
  node test.js unit          # Run unit tests only
  node test.js watch         # Run in watch mode
  node test.js coverage      # Run with coverage report
`);
}

if (!command || command === 'help') {
  showHelp();
} else if (command in commands) {
  console.log(`🧪 Running: ${commands[command].desc}\n`);
  try {
    execSync(commands[command].cmd, { 
      stdio: 'inherit',
      cwd: __dirname
    });
  } catch (error) {
    process.exit(1);
  }
} else {
  console.error(`❌ Unknown command: ${command}\n`);
  showHelp();
  process.exit(1);
}