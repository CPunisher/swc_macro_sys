import fs from 'fs';
import path from 'path';
import { transformSync } from '@swc/core';

/**
 * Optimizer utility for SWC Macro system
 * Handles WASM loading with JSX transformation support
 */
export class SWCOptimizer {
  constructor() {
    this.optimize = null;
    this.isInitialized = false;
  }

  /**
   * Initialize the WASM optimizer
   */
  async initialize() {
    if (this.isInitialized) return;

    // Load the real WASM module - no fallbacks
    const wasmModule = await import('../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    this.optimize = wasmModule.optimize;
    console.log('✅ SWC WASM module loaded successfully');

    this.isInitialized = true;
  }

  /**
   * Transform JSX to regular JavaScript
   */
  transformJSX(code) {
    try {
      const result = transformSync(code, {
        jsc: {
          parser: {
            syntax: 'ecmascript',
            jsx: true,
          },
          transform: {
            react: {
              runtime: 'classic', // Use React.createElement
            },
          },
          target: 'es2020',
          preserveAllComments: true, // Keep macro comments
        },
        module: {
          type: 'commonjs',
        },
      });
      
      return result.code;
    } catch (error) {
      console.error('❌ JSX transformation failed:', error);
      throw new Error(`JSX transformation failed: ${error.message}`);
    }
  }

  /**
   * Optimize source code with given configuration
   * Handles both regular JS and JSX (with auto-transformation)
   */
  async optimizeCode(source, config, options = {}) {
    await this.initialize();
    
    let transformedSource = source;
    
    // Auto-detect JSX and transform if needed
    if (options.isJSX || source.includes('<') && source.includes('/>')) {
      console.log('🔧 Transforming JSX to JavaScript...');
      transformedSource = this.transformJSX(source);
      console.log('✅ JSX transformation complete');
    }
    
    return this.optimize(transformedSource, JSON.stringify(config));
  }

  /**
   * Analyze the optimization results
   */
  analyzeOptimization(original, optimized, config) {
    const originalSize = original.length;
    const optimizedSize = optimized.length;
    const sizeReduction = originalSize - optimizedSize;
    const sizeReductionPercent = ((sizeReduction / originalSize) * 100).toFixed(2);

    // Count webpack modules in original and optimized
    const originalModules = this.countWebpackModules(original);
    const optimizedModules = this.countWebpackModules(optimized);
    const modulesRemoved = originalModules - optimizedModules;

    // Analyze conditional blocks
    const originalBlocks = this.countConditionalBlocks(original);
    const optimizedBlocks = this.countConditionalBlocks(optimized);
    const blocksRemoved = originalBlocks - optimizedBlocks;

    return {
      sizes: {
        original: originalSize,
        optimized: optimizedSize,
        reduction: sizeReduction,
        reductionPercent: parseFloat(sizeReductionPercent)
      },
      modules: {
        original: originalModules,
        optimized: optimizedModules,
        removed: modulesRemoved
      },
      conditionalBlocks: {
        original: originalBlocks,
        optimized: optimizedBlocks,
        removed: blocksRemoved
      },
      config: config,
      isUsingMock: this.isUsingMock
    };
  }

  /**
   * Count webpack modules in source
   */
  countWebpackModules(source) {
    // Updated regex to handle both (function and function formats
    const moduleRegex = /\d+:\s*(?:\()?function/g;
    const matches = source.match(moduleRegex);
    return matches ? matches.length : 0;
  }

  /**
   * Count conditional compilation blocks
   */
  countConditionalBlocks(source) {
    const blockRegex = /\/\*\s*@common:if\s*\[/g;
    const matches = source.match(blockRegex);
    return matches ? matches.length : 0;
  }
}

// Create a singleton instance
export const optimizer = new SWCOptimizer(); 