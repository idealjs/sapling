import { describe, it } from "vitest";
import { createStore as ourCreateStore } from "../../useSelector";
import {
  ARRAY_SIZE,
  computeStats,
  formatMB,
  getMemoryUsage,
  type MemoryMetrics,
  makeArrayState,
  N_SUBSCRIBERS,
  N_UPDATES,
  RUNS,
  removeOutliers,
  time,
} from "../bench.utils";

describe("bench - our store", () => {
  it("measures subscribe and update for array workload (no view)", () => {
    const subscribeTimings: number[] = [];
    const updateTimings: number[] = [];
    const memoryMetrics: MemoryMetrics[] = [];

    for (let run = 0; run < RUNS; run++) {
      const heapUsedBefore = getMemoryUsage();
      const { store, subscribeSelector } = ourCreateStore(makeArrayState());

      const selectorForIndex = (i: number) => () => store.items[i].value;
      const unsubFns: Array<() => void> = [];

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;

      const subscribeTime = time(() => {
        for (let i = 0; i < N_SUBSCRIBERS; i++) {
          const idx = i % ARRAY_SIZE;
          const unsub = subscribeSelector(
            () => selectorForIndex(idx)(),
            () => {},
          );
          unsubFns.push(unsub);
        }
      });

      const updateStart = performance.now();
      for (let u = 0; u < N_UPDATES; u++) {
        const idx = Math.floor(Math.random() * ARRAY_SIZE);
        store.items[idx].value = Math.random();
      }
      const updateTime = performance.now() - updateStart;

      heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

      for (const u of unsubFns) u();

      const retained = getMemoryUsage();

      subscribeTimings.push(subscribeTime);
      updateTimings.push(updateTime);
      memoryMetrics.push({
        heapUsedBefore,
        heapUsedAfter,
        heapUsedPeak,
        retained,
      });
    }

    const subClean = removeOutliers(subscribeTimings);
    const updClean = removeOutliers(updateTimings);

    const subStats = computeStats(subClean);
    const updStats = computeStats(updClean);
    const memoryStats = {
      heapUsedBefore: memoryMetrics[0]?.heapUsedBefore || 0,
      heapUsedAfter: memoryMetrics[0]?.heapUsedAfter || 0,
      heapUsedPeak: Math.max(...memoryMetrics.map((m) => m.heapUsedPeak)),
      retained: memoryMetrics[memoryMetrics.length - 1]?.retained || 0,
    };

    // eslint-disable-next-line no-console
    console.table({
      "our-subscribe": {
        mean: subStats.mean.toFixed(2),
        median: subStats.median.toFixed(2),
        min: subStats.min.toFixed(2),
        max: subStats.max.toFixed(2),
      },
      "our-update": {
        mean: updStats.mean.toFixed(2),
        median: updStats.median.toFixed(2),
        min: updStats.min.toFixed(2),
        max: updStats.max.toFixed(2),
      },
      "our-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
      },
    });
  });
});
