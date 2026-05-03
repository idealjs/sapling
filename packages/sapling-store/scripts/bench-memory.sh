#!/bin/bash
# Memory benchmark test runner with GC support

cd "$(dirname "$0")/../../.." || exit 1

files=(
	packages/sapling-store/src/bench/memory/redux.mount-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/redux.subscribe-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/valtio.mount-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/valtio.subscribe-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/zustand.mount-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/zustand.subscribe-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/ours.mount-update.bench.test.tsx
	packages/sapling-store/src/bench/memory/ours.subscribe-update.bench.test.tsx
)

for file in "${files[@]}"; do
	echo "📦 $file"
	node --expose-gc node_modules/vitest/vitest.mjs run "$file" --config packages/sapling-store/vitest.memory.config.ts --reporter=verbose
	echo ""
done
