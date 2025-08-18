#!/bin/bash

# Create a directory for the packed tarballs
mkdir -p rspack-tarballs

# Get current date stamp
DATE=$(date +%Y%m%d%H%M%S)

# Clean up old tarballs first
rm -f rspack-tarballs/*.tgz

echo "Packing rspack packages with timestamp: ${DATE}"

# Pack @rspack/cli
echo "Packing @rspack/cli..."
cd rspack/packages/rspack-cli && npm pack --pack-destination ../../../rspack-tarballs
cd ../../..
mv rspack-tarballs/rspack-cli-*.tgz rspack-tarballs/rspack-cli-${DATE}.tgz

# Pack @rspack/core  
echo "Packing @rspack/core..."
cd rspack/packages/rspack && npm pack --pack-destination ../../../rspack-tarballs
cd ../../..
mv rspack-tarballs/rspack-core-*.tgz rspack-tarballs/rspack-core-${DATE}.tgz

# Pack @rspack/binding with native binding
echo "Packing @rspack/binding..."
# First, ensure the native binding exists
if [ -f "rspack/crates/node_binding/rspack.darwin-arm64.node" ]; then
  echo "Found native binding, including in package..."
  cp rspack/crates/node_binding/rspack.darwin-arm64.node rspack/crates/node_binding/
fi
cd rspack/crates/node_binding && npm pack --pack-destination ../../../rspack-tarballs
cd ../../..
mv rspack-tarballs/rspack-binding-*.tgz rspack-tarballs/rspack-binding-${DATE}.tgz

# Pack @rspack/binding-testing (for rspack-test-tools)
echo "Packing @rspack/binding-testing..."
cd rspack/crates/rspack_binding_builder_testing && npm pack --pack-destination ../../../rspack-tarballs
cd ../../..
mv rspack-tarballs/rspack-binding-testing-*.tgz rspack-tarballs/rspack-binding-testing-${DATE}.tgz 2>/dev/null || true

# Pack create-rspack
echo "Packing create-rspack..."
cd rspack/packages/create-rspack && npm pack --pack-destination ../../../rspack-tarballs
cd ../../..
mv rspack-tarballs/create-rspack-*.tgz rspack-tarballs/create-rspack-${DATE}.tgz 2>/dev/null || true

# Pack @rspack/browser (if it exists)
if [ -d "rspack/packages/rspack-browser" ]; then
  echo "Packing @rspack/browser..."
  cd rspack/packages/rspack-browser && npm pack --pack-destination ../../../rspack-tarballs
  cd ../../..
  mv rspack-tarballs/rspack-browser-*.tgz rspack-tarballs/rspack-browser-${DATE}.tgz 2>/dev/null || true
fi

# Pack rspack-test-tools (if needed)
if [ -d "rspack/packages/rspack-test-tools" ]; then
  echo "Packing @rspack/test-tools..."  
  cd rspack/packages/rspack-test-tools && npm pack --pack-destination ../../../rspack-tarballs
  cd ../../..
  mv rspack-tarballs/rspack-test-tools-*.tgz rspack-tarballs/rspack-test-tools-${DATE}.tgz 2>/dev/null || true
fi

# Extract, add native binding, and repack @rspack/binding
echo "Adding native binding to @rspack/binding tarball..."
cd rspack-tarballs
mkdir -p temp-binding
tar -xzf rspack-binding-${DATE}.tgz -C temp-binding
if [ -f "../rspack/crates/node_binding/rspack.darwin-arm64.node" ]; then
  cp ../rspack/crates/node_binding/rspack.darwin-arm64.node temp-binding/package/
  cd temp-binding
  tar -czf ../rspack-binding-${DATE}.tgz package
  cd ..
  rm -rf temp-binding
  echo "Native binding added to tarball"
else
  echo "Warning: Native binding not found at ../rspack/crates/node_binding/rspack.darwin-arm64.node"
  rm -rf temp-binding
fi
cd ..

# Update package.json with the new tarballs
echo "Updating package.json overrides..."
node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));

// Ensure pnpm.overrides exists
if (!pkg.pnpm) pkg.pnpm = {};
if (!pkg.pnpm.overrides) pkg.pnpm.overrides = {};

// Update all the overrides
pkg.pnpm.overrides['@rspack/cli'] = 'file:./rspack-tarballs/rspack-cli-${DATE}.tgz';
pkg.pnpm.overrides['@rspack/core'] = 'file:./rspack-tarballs/rspack-core-${DATE}.tgz';
pkg.pnpm.overrides['@rspack/binding'] = 'file:./rspack-tarballs/rspack-binding-${DATE}.tgz';

// Add optional packages if their tarballs exist
if (fs.existsSync('rspack-tarballs/rspack-binding-testing-${DATE}.tgz')) {
  pkg.pnpm.overrides['@rspack/binding-testing'] = 'file:./rspack-tarballs/rspack-binding-testing-${DATE}.tgz';
}
if (fs.existsSync('rspack-tarballs/create-rspack-${DATE}.tgz')) {
  pkg.pnpm.overrides['create-rspack'] = 'file:./rspack-tarballs/create-rspack-${DATE}.tgz';
}
if (fs.existsSync('rspack-tarballs/rspack-browser-${DATE}.tgz')) {
  pkg.pnpm.overrides['@rspack/browser'] = 'file:./rspack-tarballs/rspack-browser-${DATE}.tgz';
}
if (fs.existsSync('rspack-tarballs/rspack-test-tools-${DATE}.tgz')) {
  pkg.pnpm.overrides['@rspack/test-tools'] = 'file:./rspack-tarballs/rspack-test-tools-${DATE}.tgz';
}

fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2) + '\\n');
console.log('Updated package.json with new tarball paths');
console.log('Overrides:', Object.keys(pkg.pnpm.overrides).join(', '));
"

echo "Done! Tarballs created with timestamp: ${DATE}"
ls -la rspack-tarballs/*.tgz