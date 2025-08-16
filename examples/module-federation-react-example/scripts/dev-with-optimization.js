#!/usr/bin/env node

import { spawn, execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(__dirname, '..');

console.log('🚀 Starting Module Federation Dev with Optimization\n');

// Check if servers are running
async function checkServers() {
  try {
    const hostResponse = await fetch('http://localhost:3001/');
    const remoteResponse = await fetch('http://localhost:3002/remoteEntry.js');
    return hostResponse.ok && remoteResponse.ok;
  } catch {
    return false;
  }
}

// Watch for file changes and re-optimize
function setupWatcher() {
  console.log('👀 Setting up file watcher for auto-optimization...');
  
  const chokidar = await import('chokidar').catch(() => null);
  if (!chokidar) {
    console.log('⚠️  chokidar not available - install with: pnpm add -D chokidar');
    return;
  }
  
  const watcher = chokidar.watch([
    'host/dist/share-usage.json',
    'remote/dist/share-usage.json'
  ], {
    cwd: projectRoot,
    ignoreInitial: false
  });
  
  let optimizing = false;
  
  watcher.on('change', async (filePath) => {
    if (optimizing) return;
    
    console.log(`📊 ${filePath} changed - running optimization...`);
    optimizing = true;
    
    try {
      execSync('node --experimental-wasm-modules scripts/optimize-shared-chunks.js', {
        cwd: projectRoot,
        stdio: 'inherit'
      });
      console.log('✅ Optimization completed');
    } catch (error) {
      console.error('❌ Optimization failed:', error.message);
    } finally {
      optimizing = false;
    }
  });
  
  return watcher;
}

// Start dev servers
function startDevServers() {
  console.log('🚀 Starting host and remote dev servers...');
  
  const hostProcess = spawn('pnpm', ['-C', 'host', 'dev'], {
    stdio: ['ignore', 'inherit', 'inherit'],
    cwd: projectRoot
  });
  
  const remoteProcess = spawn('pnpm', ['-C', 'remote', 'dev'], {
    stdio: ['ignore', 'inherit', 'inherit'],
    cwd: projectRoot
  });
  
  return { hostProcess, remoteProcess };
}

// Wait for servers to be ready
async function waitForServers(timeout = 60000) {
  console.log('⏳ Waiting for dev servers to be ready...');
  const start = Date.now();
  
  while (Date.now() - start < timeout) {
    if (await checkServers()) {
      console.log('✅ Dev servers are ready');
      return true;
    }
    await new Promise(resolve => setTimeout(resolve, 2000));
    process.stdout.write('.');
  }
  
  throw new Error('Dev servers failed to start within timeout');
}

// Main function
async function main() {
  let serverProcesses = null;
  let watcher = null;
  
  try {
    // Check if servers are already running
    const serversRunning = await checkServers();
    
    if (!serversRunning) {
      serverProcesses = startDevServers();
      await waitForServers();
    } else {
      console.log('✅ Dev servers already running');
    }
    
    // Run initial optimization
    console.log('\n⚡ Running initial optimization...');
    try {
      execSync('node --experimental-wasm-modules scripts/optimize-shared-chunks.js', {
        cwd: projectRoot,
        stdio: 'inherit'
      });
      console.log('✅ Initial optimization completed');
    } catch (error) {
      console.warn('⚠️  Initial optimization failed:', error.message);
    }
    
    // Setup file watcher for auto-optimization
    watcher = await setupWatcher();
    
    console.log('\n🎉 Development environment ready!');
    console.log('📱 Host: http://localhost:3001');
    console.log('🔗 Remote: http://localhost:3002');
    console.log('⚡ Auto-optimization enabled');
    console.log('\nPress Ctrl+C to stop...\n');
    
    // Keep the process alive
    process.on('SIGINT', () => {
      console.log('\n🧹 Shutting down...');
      
      if (watcher) watcher.close();
      
      if (serverProcesses) {
        serverProcesses.hostProcess?.kill();
        serverProcesses.remoteProcess?.kill();
      }
      
      try {
        execSync('npx -y kill-port 3001 3002', { stdio: 'ignore' });
      } catch {}
      
      process.exit(0);
    });
    
    // Keep alive
    while (true) {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
    
  } catch (error) {
    console.error('❌ Failed to start development environment:', error.message);
    process.exit(1);
  }
}

main();
