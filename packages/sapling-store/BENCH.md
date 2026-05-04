# Benchmark Tests for sapling-store

Performance comparison of Redux, Valtio, Zustand, and our `createStore` implementation.

## Setup

Install dependencies (including Redux, Valtio, Zustand):

```bash
cd /home/cqh/workspace/sapling
yarn
```

## Running Benchmarks

Run all benchmarks and display comparison table:

```bash
cd packages/sapling-store
yarn vitest run src/bench/all.bench.test.ts
```

Or run individual library benchmarks:

```bash
# Our store
yarn vitest run src/bench/ours.bench.test.ts

# Redux
yarn vitest run src/bench/redux.bench.test.ts

# Valtio
yarn vitest run src/bench/valtio.bench.test.ts

# Zustand
yarn vitest run src/bench/zustand.bench.test.ts
```

## Benchmark Details

- **Runs**: 10 iterations per library
- **Outlier handling**: Min and max timings removed before computing stats
- **Workload**: 1000 array items, 1000 subscribers, 1000 random mutations
- **Metrics**: Subscribe time (ms) and update loop time (ms)

### Statistics Reported

- **mean**: Average time across cleaned runs
- **median**: Middle value of sorted timings
- **min**: Minimum time after outlier removal
- **max**: Maximum time after outlier removal

## Results Interpretation

Lower times indicate better performance. The comparison helps identify:

- **Subscribe overhead**: Time to register 1000 listeners
- **Update throughput**: Time to perform 1000 random mutations

See output table for detailed results across all four implementations.

yarn vitest run ours.subscribe-update --maxWorkers 1 --pool forks --execArgv=--cpu-prof --execArgv=--cpu-prof-name=ours-subscribe-update.cpuprofile
