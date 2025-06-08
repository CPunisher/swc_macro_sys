import fs from 'fs';

// Configuration object as specified
const config = {
    "build": {
        "target": "production"
    },
    "device": {
        "isMobile": false
    },
    "user": {
        "language": "en",
        "isLoggedIn": true
    },
    "experiment": {
        "group": "B"
    },
    "featureFlags": {
        "newMobileUI": true,
        "enableNewFeature": false,
        "newUserProfile": false
    }
};

async function processTestFile() {
    try {
        // Read the test.js file
        const sourceCode = fs.readFileSync('./test.js', 'utf8');
        
        console.log('Original source code:');
        console.log('='.repeat(50));
        console.log(sourceCode);
        console.log('\n');
        
        // Convert config to JSON string
        const configString = JSON.stringify(config);
        
        console.log('Configuration:');
        console.log('='.repeat(50));
        console.log(JSON.stringify(config, null, 2));
        console.log('\n');
        
        // Import the background JS file
        const wasmBg = await import('./crates/swc_macro_wasm/pkg/swc_macro_wasm_bg.js');
        
        // Process the code using the wasm optimize function from background module
        const optimizedCode = wasmBg.optimize(sourceCode, configString);
        
        console.log('Optimized code:');
        console.log('='.repeat(50));
        console.log(optimizedCode);
        
    } catch (error) {
        console.error('Error processing file:', error);
        console.error('Stack trace:', error.stack);
    }
}

// Run the example
processTestFile();