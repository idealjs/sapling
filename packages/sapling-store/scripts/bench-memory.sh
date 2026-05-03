#!/bin/bash
# Memory benchmark test runner with GC support

cd "$(dirname "$0")/../../.." || exit 1

echo "🔍 Running memory benchmark with GC support..."
echo ""

files=(
	packages/sapling-store/src/bench/redux.bench.test.ts
	packages/sapling-store/src/bench/valtio.bench.test.ts
	packages/sapling-store/src/bench/zustand.bench.test.ts
	packages/sapling-store/src/bench/ours.bench.test.ts
)

for file in "${files[@]}"; do
	echo "📦 $file"
	node --expose-gc node_modules/vitest/vitest.mjs run "$file" --config packages/sapling-store/vitest.memory.config.ts --reporter=verbose
	echo ""
done
