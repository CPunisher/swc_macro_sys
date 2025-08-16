#!/usr/bin/env node

// Runtime execution test for optimized vendor chunks.
// Verifies that kept exports still execute (not undefined) after optimization.

const fs = require('fs');
const path = require('path');
const { createRequire } = require('module');

function readJson(p) {
  return JSON.parse(fs.readFileSync(p, 'utf8'));
}

function resolveDistPaths() {
  const hostDist = path.resolve(__dirname, '../host/dist');
  const remoteDist = path.resolve(__dirname, '../remote/dist');
  const hostUsage = path.join(hostDist, 'share-usage.json');
  const remoteUsage = path.join(remoteDist, 'share-usage.json');
  const candidate = [];
  if (fs.existsSync(hostUsage)) candidate.push({ app: 'host', dist: hostDist, usagePath: hostUsage });
  if (fs.existsSync(remoteUsage)) candidate.push({ app: 'remote', dist: remoteDist, usagePath: remoteUsage });
  if (candidate.length === 0) throw new Error('No share-usage.json found. Build first.');
  return candidate;
}

function pickVendorChunk(dist, usage) {
  // Find first library with chunk_characteristics and .js chunk file
  const libs = Object.entries(usage.treeShake || {});
  for (const [lib, cfg] of libs) {
    const chars = cfg?.chunk_characteristics;
    if (!chars) continue;
    const files = Array.isArray(chars.chunk_files) ? chars.chunk_files : [];
    const jsFile = files.find(f => typeof f === 'string' && f.endsWith('.js'));
    if (!jsFile) continue;
    const full = path.join(dist, jsFile);
    if (fs.existsSync(full)) return { library: lib, chunkPath: full, characteristics: chars };
  }
  throw new Error('No vendor chunk with chunk_characteristics.chunk_files found.');
}

function createRuntime(modulesObj) {
  const cache = Object.create(null);
  function __webpack_require__(id) {
    if (cache[id]) return cache[id].exports;
    const fn = modulesObj[id];
    if (!fn) throw new Error('Module not found: ' + id);
    const module = { exports: {} };
    cache[id] = module;
    fn(module, module.exports, __webpack_require__);
    return module.exports;
  }
  __webpack_require__.d = (exports, definition) => {
    for (const key in definition) {
      Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
    }
  };
  __webpack_require__.r = (exports) => {
    Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
    Object.defineProperty(exports, '__esModule', { value: true });
  };
  return __webpack_require__;
}

function evaluateChunkAndGetModulesFromSource(source) {
  // Try evaluation path first
  try {
    const wrapper = `(function(){ var exports = {}; ${source}\n;return exports.modules || exports.__modules || null; })()`;
    // eslint-disable-next-line no-new-func
    const fn = new Function(wrapper + ';');
    const modules = fn();
    if (modules) return modules;
  } catch (_) {}

  // Fallback: extract object literal for exports.modules = { ... }
  const assignIdx = source.indexOf('exports.modules');
  if (assignIdx !== -1) {
    const eqIdx = source.indexOf('=', assignIdx);
    if (eqIdx !== -1) {
      const braceStart = source.indexOf('{', eqIdx);
      if (braceStart !== -1) {
        let i = braceStart;
        let depth = 0;
        let inStr = false;
        let strCh = '';
        for (; i < source.length; i++) {
          const ch = source[i];
          if (inStr) {
            if (ch === '\\') { i++; continue; }
            if (ch === strCh) inStr = false;
            continue;
          }
          if (ch === '"' || ch === '\'') { inStr = true; strCh = ch; continue; }
          if (ch === '{') depth++;
          else if (ch === '}') { depth--; if (depth === 0) { i++; break; } }
        }
        const objCode = source.slice(braceStart, i);
        // eslint-disable-next-line no-new-func
        const modules = new Function('return (' + objCode + ');')();
        if (modules && typeof modules === 'object') return modules;
      }
    }
  }
  throw new Error('modules object missing after evaluation');
}

function requireChunkModules(chunkPath) {
  try {
    const requireCjs = createRequire(require.main === module ? __filename : require.main.filename);
    const mod = requireCjs(chunkPath);
    if (mod && (mod.modules || mod.__modules)) return mod.modules || mod.__modules;
  } catch (e) {
    // fall through to source eval
  }
  const source = fs.readFileSync(chunkPath, 'utf8');
  return evaluateChunkAndGetModulesFromSource(source);
}

function findModuleIdByHeuristic(library, exportName, modulesObj) {
  // Try to locate module by matching known path pattern inside function body comments
  const pathPattern = new RegExp(`/(?:${library.replace(/[.*+?^${}()|[\\]\\]/g, '\\$&')})/${exportName}\\.js`);
  for (const [id, fn] of Object.entries(modulesObj)) {
    try {
      const src = String(fn);
      if (pathPattern.test(src)) return id;
    } catch (_) {}
  }
  return null;
}

function main() {
  console.log('🧪 Running vendor runtime execution test');
  const apps = resolveDistPaths();
  let failures = 0;
  apps.forEach(({ app, dist, usagePath }) => {
    console.log(`\nApp: ${app}`);
    const usage = readJson(usagePath);
    const { library, chunkPath, characteristics } = pickVendorChunk(dist, usage);
    const kept = Object.entries(usage.treeShake?.[library] || {})
      .filter(([k, v]) => k !== 'chunk_characteristics' && v === true)
      .map(([k]) => k);
    console.log(`Library: ${library}`);
    console.log(`Chunk: ${path.basename(chunkPath)}`);
    console.log(`Kept exports: ${kept.slice(0, 12).join(', ')}${kept.length > 12 ? ' ...' : ''}`);

    const modulesObj = requireChunkModules(chunkPath);
  const __webpack_require__orig = createRuntime(modulesObj);
  let __last_required_id = null;
  function __webpack_require__(id) {
    __last_required_id = id;
    return __webpack_require__orig(id);
  }
  __webpack_require__.d = __webpack_require__orig.d;
  __webpack_require__.r = __webpack_require__orig.r;

    // Ensure entry module exists
    const entryId = characteristics.entry_module_id;
    if (!entryId) throw new Error('entry_module_id missing in chunk_characteristics');
    let entryExports;
    try {
      entryExports = __webpack_require__(entryId);
    } catch (e) {
      console.error('❌ Failed requiring entry module:', e.message);
      failures++;
      return;
    }

    // For each kept export, locate its module id and require it
    let localFailures = 0;
    kept.forEach(name => {
      // Try original ID pattern match (path-like IDs)
      let moduleId = Object.keys(modulesObj).find(id => new RegExp(`/` + library + `/` + name + `\\.js$`).test(id));
      // If not found (numeric ids), fall back to heuristic by scanning function source
      if (!moduleId) moduleId = findModuleIdByHeuristic(library, name, modulesObj);
      // If still not found, try triggering the getter on entry exports to capture the required id
      if (!moduleId) {
        try {
          __last_required_id = null;
          // Accessing the property triggers __webpack_require__ via getter defined by __webpack_require__.d
          // eslint-disable-next-line no-unused-vars
          const _val = entryExports && entryExports[name];
          if (__last_required_id != null) moduleId = __last_required_id;
        } catch (_) {}
      }
      if (moduleId) {
        try {
          const mod = __webpack_require__(moduleId);
          const value = mod && (mod.default ?? mod[name] ?? mod);
          const ok = typeof value === 'function' || typeof value === 'object' || typeof value === 'string';
          if (!ok) {
            console.error(`❌ Export '${name}' loaded as undefined/null from ${moduleId}`);
            localFailures++;
          }
          return;
        } catch (e) {
          console.error(`❌ Requiring '${name}' from ${moduleId} threw: ${e.message}`);
          localFailures++;
          return;
        }
      }
      // Final fallback: check directly on entry module exports aggregator
      try {
        const value = entryExports && (entryExports[name] ?? (name === 'default' ? entryExports.default : undefined));
        const ok = typeof value === 'function' || typeof value === 'object' || typeof value === 'string';
        if (!ok) {
          console.error(`❌ Missing export '${name}' on entry module exports`);
          localFailures++;
        }
      } catch (e) {
        console.error(`❌ Accessing export '${name}' on entry module threw: ${e.message}`);
        localFailures++;
      }
    });

    if (localFailures === 0) {
      console.log('✅ Runtime verification passed: all kept exports are defined');
    } else {
      failures += localFailures;
    }
  });

  if (failures > 0) {
    console.error(`\n❌ Vendor runtime test found ${failures} issue(s). A 'call of undefined' is likely due to removing a required module.`);
    process.exit(1);
  }
  console.log('\n✅ Vendor runtime test completed successfully');
}

main();


