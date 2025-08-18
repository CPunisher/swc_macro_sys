# Testing Guide

This project uses a comprehensive testing strategy that covers both Rust and JavaScript code across all workspace packages.

## Quick Start

```bash
# Install dependencies
pnpm install

# Run all tests (Rust + JavaScript)
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run tests with coverage
pnpm test:coverage

# Open test UI
pnpm test:ui
```

## Test Commands

### Main Commands

| Command | Description |
|---------|-------------|
| `pnpm test` | Run all tests (Rust + JS) sequentially |
| `pnpm test:all` | Run all tests with parallel JS tests |
| `pnpm test:rust` | Run all Rust tests |
| `pnpm test:js` | Run all JavaScript tests |
| `pnpm test:ci` | Build everything then run all tests |

### Specific Test Types

| Command | Description |
|---------|-------------|
| `pnpm test:unit` | Run unit tests in all workspaces |
| `pnpm test:integration` | Run integration tests in all workspaces |
| `pnpm test:e2e` | Run E2E tests in all workspaces |

### Development Commands

| Command | Description |
|---------|-------------|
| `pnpm test:watch` | Run tests in watch mode (root + workspaces) |
| `pnpm test:ui` | Open Vitest UI for visual testing |
| `pnpm test:coverage` | Generate coverage reports for all tests |

### Build & Maintenance

| Command | Description |
|---------|-------------|
| `pnpm build` | Build all packages (Rust + WASM + JS) |
| `pnpm lint` | Lint all code (Rust + JS) |
| `pnpm clean` | Clean all build artifacts |

## Test Structure

### Rust Tests

Located in `crates/*/src` and `crates/*/tests`:
- Unit tests: In `src/*.rs` files with `#[cfg(test)]`
- Integration tests: In `tests/*.rs` files
- Run with: `cargo test --workspace`

### JavaScript Tests

#### Root Tests
- Location: `test/` directory
- Framework: Vitest
- Run with: `pnpm test:root`

#### Module Federation Tests
- Location: `examples/module-federation-example/test/`
- Categories:
  - `unit/` - Optimization logic tests
  - `integration/` - Build process tests
  - `e2e/` - Browser-based tests
  - `performance/` - Benchmarks
- Run with: `cd examples/module-federation-example && pnpm test`

#### JSX Server Demo Tests
- Location: `examples/jsx-server-demo/test/`
- Run with: `cd examples/jsx-server-demo && pnpm test`

## Coverage Reports

Coverage reports are generated in:
- `coverage/` - JavaScript coverage (root)
- `coverage/rust/` - Rust coverage
- `*/coverage/` - Package-specific coverage

View coverage:
```bash
# Generate all coverage reports
pnpm test:coverage

# Open coverage in browser
open coverage/index.html
open coverage/rust/index.html
```

## CI/CD Integration

Tests run automatically on:
- Pull requests
- Main branch commits
- Release tags

GitHub Actions workflow runs:
1. Build all packages
2. Run Rust tests
3. Run JavaScript tests
4. Generate coverage reports
5. Upload to Codecov

## Writing Tests

### Rust Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        assert_eq!(2 + 2, 4);
    }
}
```

### JavaScript Tests (Vitest)

```javascript
import { describe, it, expect } from 'vitest';

describe('Feature', () => {
  it('should work', () => {
    expect(true).toBe(true);
  });
});
```

## Debugging Tests

### Rust
```bash
# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

### JavaScript
```bash
# Run specific test file
pnpm vitest run path/to/test.js

# Run tests matching pattern
pnpm vitest -t "pattern"

# Debug in VS Code
# Add breakpoint and run "Debug Test" from Vitest UI
```

## Best Practices

1. **Test Organization**
   - Keep tests close to the code they test
   - Use descriptive test names
   - Group related tests with describe blocks

2. **Test Coverage**
   - Aim for >80% coverage
   - Focus on critical paths
   - Don't test implementation details

3. **Performance**
   - Keep unit tests fast (<100ms)
   - Use fixtures for large data
   - Mock external dependencies

4. **Maintenance**
   - Update tests when changing features
   - Remove obsolete tests
   - Keep tests simple and focused

## Troubleshooting

### Tests failing locally but not in CI
- Check Node.js version matches CI
- Clean and rebuild: `pnpm clean && pnpm build`
- Check for uncommitted files

### Coverage not generating
- Install coverage dependencies: `pnpm install`
- For Rust: Install tarpaulin: `cargo install cargo-tarpaulin`
- Check for syntax errors in tests

### Watch mode not working
- Check file permissions
- Restart watch mode
- Try running specific workspace: `cd package && pnpm test:watch`

## Package-Specific Testing

### swc_macro_wasm
```bash
cd crates/swc_macro_wasm
cargo test
```

### module-federation-example
```bash
cd examples/module-federation-example
pnpm test        # All tests
pnpm test:unit   # Unit only
pnpm test:e2e    # E2E only
```

### jsx-server-demo
```bash
cd examples/jsx-server-demo
pnpm test
```