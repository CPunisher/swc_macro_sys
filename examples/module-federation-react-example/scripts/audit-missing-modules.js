#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import vm from 'vm';

function listJsFiles(distDir) {
  const files = fs.readdirSync(distDir).filter(f => f.endsWith('.js') && !f.endsWith('.map') && !f.endsWith('.original'));
  return files.map(f => path.join(distDir, f));
}

function extractRequireIds(source) {
  const ids = new Set();
  // __webpack_require__(123), __webpack_require__("path"), __webpack_require__(`path`)
  const re = /__webpack_require__\s*\(\s*(\d+|"[^"]+"|'[^']+'|`[^`]+`)\s*\)/g;
  let m;
  while ((m = re.exec(source)) !== null) {
    let raw = m[1];
    if (raw.startsWith('"') || raw.startsWith("'") || raw.startsWith('`')) {
      raw = raw.slice(1, -1);
    }
    ids.add(raw);
  }
  return ids;
}

function extractDefinedModuleKeys(source) {
  const keys = new Set();
  // Robust patterns allowing optional comments/newlines between ':' and function/arrow
  const comment = String.raw`(?:/\*[\s\S]*?\*/\s*)?`;
  const ws = String.raw`[\s\n\r]*`;
  const funcStr = new RegExp(String.raw`"([^"]+)"\s*:\s*` + comment + String.raw`function\s*\(`, 'g');
  const funcNum = new RegExp(String.raw`(\d+)\s*:\s*` + comment + String.raw`function\s*\(`, 'g');
  const arrowStr = new RegExp(String.raw`"([^"]+)"\s*:\s*` + comment + String.raw`\([^)]*\)` + ws + String.raw`=>`, 'g');
  const arrowNum = new RegExp(String.raw`(\d+)\s*:\s*` + comment + String.raw`\([^)]*\)` + ws + String.raw`=>`, 'g');
  let m;
  while ((m = funcStr.exec(source)) !== null) keys.add(m[1]);
  while ((m = funcNum.exec(source)) !== null) keys.add(m[1]);
  while ((m = arrowStr.exec(source)) !== null) keys.add(m[1]);
  while ((m = arrowNum.exec(source)) !== null) keys.add(m[1]);
  return keys;
}

function extractFromModulesVar(source) {
  const keys = new Set();
  const marker = 'var __webpack_modules__';
  const start = source.indexOf(marker);
  if (start === -1) return keys;
  const openObj = source.indexOf('({', start);
  if (openObj === -1) return keys;
  const closeObj = source.indexOf('})', openObj + 2);
  if (closeObj === -1) return keys;
  const slice = source.slice(openObj + 1, closeObj + 1); // include braces
  // Reuse key regex on this slice only
  for (const k of extractDefinedModuleKeys(slice)) keys.add(k);
  return keys;
}

function evalAndCaptureDefinedModules(source) {
  const definedKeys = new Set();
  const sandbox = {
    __captured: definedKeys,
    self: {},
    window: {},
    globalThis: {},
  };
  // Fake minimal webpack runtime used in these bundles
  sandbox.self.webpackChunk_mf_react_host = sandbox.self.webpackChunk_mf_react_host || [];
  sandbox.self.webpackChunk_mf_react_remote = sandbox.self.webpackChunk_mf_react_remote || [];
  const pushInterceptor = function (arrName) {
    return function () {
      // Expect shape: [chunkIdOrName, modulesObject]
      for (const arg of arguments) {
        if (Array.isArray(arg)) {
          for (const entry of arg) {
            if (Array.isArray(entry) && entry.length >= 2 && typeof entry[1] === 'object') {
              const modules = entry[1];
              for (const k of Object.keys(modules)) sandbox.__captured.add(k);
            }
          }
        }
      }
      return Array.prototype.push.apply(this, arguments);
    };
  };
  sandbox.self.webpackChunk_mf_react_host.push = pushInterceptor('host');
  sandbox.self.webpackChunk_mf_react_remote.push = pushInterceptor('remote');
  const context = vm.createContext(sandbox);
  try {
    // Execute script; ignore runtime errors from missing __webpack_require__ etc.
    const script = new vm.Script(source, { timeout: 50 });
    script.runInContext(context, { timeout: 50 });
  } catch (_) {
    // best-effort capture
  }
  return Array.from(definedKeys);
}

function auditDist(distDir) {
  const files = listJsFiles(distDir);
  const referenced = new Map(); // id -> Set(files)
  const defined = new Map(); // id -> Set(files)

  for (const file of files) {
    const content = fs.readFileSync(file, 'utf8');
    // Collect via harness evaluation
    try {
      const keysFromVm = evalAndCaptureDefinedModules(content);
      for (const k of keysFromVm) {
        if (!defined.has(k)) defined.set(k, new Set());
        defined.get(k).add(path.basename(file));
      }
    } catch (e) {
      // fall back to regex-only
    }
    for (const id of extractRequireIds(content)) {
      if (!referenced.has(id)) referenced.set(id, new Set());
      referenced.get(id).add(path.basename(file));
    }
    // Also add regex-derived keys (fallback/additional)
    for (const key of extractDefinedModuleKeys(content)) {
      if (!defined.has(key)) defined.set(key, new Set());
      defined.get(key).add(path.basename(file));
    }
    // Targeted capture from var __webpack_modules__ = ({ ... })
    for (const key of extractFromModulesVar(content)) {
      if (!defined.has(key)) defined.set(key, new Set());
      defined.get(key).add(path.basename(file));
    }
  }

  const missing = [];
  for (const [id, users] of referenced.entries()) {
    // Ignore known externals
    const isExternal = id.startsWith('webpack/sharing/consume/') || id.startsWith('webpack/container/entry/') || id.includes('@rspack/dev-server/client');
    if (!defined.has(id) && !isExternal) {
      missing.push({ id, referencedBy: Array.from(users).sort() });
    }
  }

  missing.sort((a, b) => a.id.localeCompare(b.id));
  return { files: files.length, referencedCount: referenced.size, definedCount: defined.size, missing };
}

function main() {
  const root = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..');
  const hostDist = path.join(root, 'host', 'dist');
  const remoteDist = path.join(root, 'remote', 'dist');

  const results = {};
  if (fs.existsSync(hostDist)) results.host = auditDist(hostDist);
  if (fs.existsSync(remoteDist)) results.remote = auditDist(remoteDist);

  for (const [app, res] of Object.entries(results)) {
    console.log(`\n=== ${app.toUpperCase()} DIST AUDIT ===`);
    console.log(`Files scanned: ${res.files}`);
    console.log(`Referenced ids: ${res.referencedCount}`);
    console.log(`Defined ids: ${res.definedCount}`);
    if (res.missing.length === 0) {
      console.log('No missing module ids referenced.');
    } else {
      console.log(`Missing referenced module ids: ${res.missing.length}`);
      for (const m of res.missing.slice(0, 50)) {
        console.log(`- ${m.id} (referenced in: ${m.referencedBy.join(', ')})`);
      }
      if (res.missing.length > 50) console.log(`... and ${res.missing.length - 50} more`);
    }
  }
}

main();
