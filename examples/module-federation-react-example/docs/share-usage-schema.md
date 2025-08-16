# Minimal Share Usage Schema for Direct Macro Consumption

This schema is designed to output the exact format that the SWC macro parser expects, eliminating all transformation overhead.

## Current Problem

The optimization script currently transforms from arrays to boolean flags:
```javascript
// From current format:
{
  "lodash-es": {
    "used_exports": ["add", "sortBy", "uniq"],
    "unused_exports": ["map", "filter", "reduce"]
  }
}

// To desired direct format from rspack:
{
  "treeShake": {
    "lodash-es": {
      "add": true,
      "sortBy": true, 
      "uniq": true,
      "map": false,
      "filter": false,
      "reduce": false
    }
  },
  // entry module is carried within each library's chunk_characteristics (see below)
}
```

## Proposed Minimal Schema

Output the **exact** macro format directly from rspack:

```json
{
  "treeShake": {
    "lodash-es": {
      "add": true,
      "sortBy": true,
      "uniq": true,
      "default": true,
      "map": false,
      "filter": false,
      "reduce": false,
      "groupBy": false,
      "omit": false,
      "capitalize": false,
      "pick": false,
      "throttle": false,
      "debounce": false,
      "chunk_characteristics": {
        "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
      }
    }
  },
  "chunk_characteristics": {
    "is_runtime_chunk": false,
    "has_runtime": false,
    "is_entrypoint": false,
    "can_be_initial": false,
    "is_only_initial": false,
    "chunk_format": "async-node",
    "chunk_loading_type": null,
    "runtime_names": ["remote", "main"],
    "entry_name": null,
    "has_async_chunks": false,
    "chunk_files": ["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js"],
    "is_shared_chunk": false,
    "shared_modules": []
  }
}
```

## Schema Benefits

1. **Zero Transformation**: Direct consumption by optimizer
2. **Minimal Size**: Only required fields
3. **Direct Pass-Through**: Can be JSON.stringify'd directly to macro

## Implementation

### Rspack Output
```typescript
// Direct output of macro format
const shareUsage = {
  treeShake: compilation.getTreeShakeFlagsWithEntryIds(), // { "lodash-es": { "sortBy": true, "map": false, chunk_characteristics: { entry_module_id: '...' } } }
  chunk_characteristics: compilation.getChunkCharacteristics()
};
```

### Optimization Script (Simplified)
```javascript
// No transformation needed!
async function optimizeChunk(chunkPath, shareUsageJson, optimizer) {
  const result = optimizer.optimize(sourceCode, JSON.stringify(shareUsageJson));
  return result;
}

// Usage becomes trivial:
const shareUsage = JSON.parse(fs.readFileSync('share-usage.json'));
optimizeChunk(chunkPath, shareUsage, optimizer);
```

### Merging Multiple Apps
```javascript
function mergeShareUsage(files) {
  const merged = { treeShake: {}, chunk_characteristics: {} };
  
  files.forEach(({ data }) => {
    // Merge treeShake flags (OR used exports, AND unused exports)
    Object.entries(data.treeShake).forEach(([module, flags]) => {
      if (!merged.treeShake[module]) {
        merged.treeShake[module] = { ...flags };
      } else {
        Object.entries(flags).forEach(([exportName, isUsed]) => {
          // If any app uses it, mark as used
          merged.treeShake[module][exportName] = 
            merged.treeShake[module][exportName] || isUsed;
        });
      }
    });
    
    // Use first chunk_characteristics found
    if (!Object.keys(merged.chunk_characteristics).length) {
      merged.chunk_characteristics = data.chunk_characteristics;
    }
  });
  
  return merged;
}
```

## JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Minimal Share Usage for Direct Macro Consumption",
  "type": "object",
  "required": ["treeShake", "chunk_characteristics"],
  "properties": {
    "treeShake": {
      "type": "object",
      "patternProperties": {
        "^[a-zA-Z0-9@/_-]+$": {
          "type": "object",
          "patternProperties": {
            "^[a-zA-Z0-9_$]+$": {
              "type": "boolean"
            }
          }
        }
      }
    },
    "chunk_characteristics": {
      "type": "object",
      "required": ["chunk_format"],
      "properties": {
        "is_runtime_chunk": { "type": "boolean" },
        "has_runtime": { "type": "boolean" },
        "is_entrypoint": { "type": "boolean" },
        "can_be_initial": { "type": "boolean" },
        "is_only_initial": { "type": "boolean" },
        "chunk_format": { "type": "string" },
        "chunk_loading_type": { "type": ["string", "null"] },
        "runtime_names": { 
          "type": "array", 
          "items": { "type": "string" } 
        },
        "entry_name": { "type": ["string", "null"] },
        "has_async_chunks": { "type": "boolean" },
        "chunk_files": { 
          "type": "array", 
          "items": { "type": "string" } 
        },
        "is_shared_chunk": { "type": "boolean" },
        "shared_modules": { 
          "type": "array", 
          "items": { "type": "object" } 
        }
      }
    }
  }
}
```

This minimal schema eliminates all transformation overhead by outputting exactly what the macro expects.