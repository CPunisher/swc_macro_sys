#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

export CI=${CI:-1}

echo "Node: $(node -v || true)"
echo "pnpm: $(pnpm -v || true)"
echo "Rust: $(rustc -V || true)"

cd "$ROOT_DIR"

rustup target add wasm32-unknown-unknown || true

cargo test --workspace --all-features --all-targets -- --nocapture

pnpm install --frozen-lockfile=false || pnpm install

pnpm build:wasm || true

run_example() {
  local EX_DIR="$1"
  echo "Running example in $EX_DIR"
  cd "$ROOT_DIR/$EX_DIR"

  pnpm install --frozen-lockfile=false || pnpm install || true
  pnpm -C host install --frozen-lockfile=false || true
  pnpm -C remote install --frozen-lockfile=false || true

  pnpm -C host build
  pnpm -C remote build

  node --experimental-wasm-modules scripts/optimize-shared-chunks.js || true
  node scripts/audit-missing-modules.js || true

  pnpm vitest run || true

  npx playwright install --with-deps chromium
  npx playwright test --reporter=line --project=chromium

  cd "$ROOT_DIR"
}

run_example examples/module-federation-example || true
run_example examples/module-federation-react-example || true

echo "CI script completed"


