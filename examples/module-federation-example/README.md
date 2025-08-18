# Module Federation Example with Rspack and Lodash-ES

This example demonstrates Module Federation using Rspack with lodash-es as a shared module.

## Project Structure

```
examples/module-federation-example/
├── host/              # Host application (port 3001)
├── remote/            # Remote application (port 3002)
└── package.json       # Root package.json with workspace configuration
```

## Features

- **Module Federation**: Host app consumes components from remote app
- **Shared Dependencies**: lodash-es is shared between both apps to avoid duplication
- **Rspack**: Fast bundler with Module Federation support
- **Live Demo**: Both apps demonstrate lodash-es usage

## Setup and Running

1. **Install dependencies**:
   ```bash
   cd examples/module-federation-example
   pnpm install
   ```

2. **Start both applications**:
   ```bash
   # Start both apps in parallel
   pnpm dev
   
   # Or start individually:
   # Terminal 1 (Remote app)
   cd remote && pnpm dev
   
   # Terminal 2 (Host app) 
   cd host && pnpm dev
   ```

3. **View the applications**:
   - **Remote app**: http://localhost:3002
   - **Host app**: http://localhost:3001

## What's Demonstrated

### Shared lodash-es Module
Both applications use lodash-es functions, but the library is loaded only once and shared between them:

**Remote app uses**:
- `capitalize()` - in Button component
- `debounce()`, `throttle()` - in utils
- `pick()`, `omit()` - for data filtering
- `groupBy()` - for data organization

**Host app uses**:
- `sortBy()` - for sorting items
- `uniq()` - for getting unique categories
- Remote utils for data formatting

### Module Federation Setup

**Remote app exposes**:
- `./Button` - A styled button component
- `./utils` - Utility functions using lodash-es

**Host app consumes**:
- Remote Button component
- Remote utility functions
- Demonstrates dynamic imports

## Key Configuration

### Shared Dependencies
```javascript
shared: {
  'lodash-es': {
    singleton: true,
    strictVersion: true,
    requiredVersion: '^4.17.21',
  },
}
```

This ensures:
- Only one instance of lodash-es is loaded
- Version compatibility between apps
- Reduced bundle size through sharing

## Build for Production

```bash
pnpm build
```

This will create production builds for both applications in their respective `dist/` folders.