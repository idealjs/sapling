import { describe, it } from "vitest";
import createUseStore from "../../createUseStore";
import {
  formatMB,
  getMemoryUsage,
  type MemoryMetrics,
  makeArrayState,
  N_SUBSCRIBERS,
  N_UPDATES,
  RUNS,
} from "../bench.utils";

describe("bench - our store", () => {
  it("measures subscribe and update for array workload (no view)", () => {
    const memoryMetrics: MemoryMetrics[] = [];
    let count = 0;
    const calc = (v: number) => {
      count += v;
    };
    for (let run = 0; run < RUNS; run++) {
      const heapUsedBefore = getMemoryUsage();
      const original = makeArrayState();
      const { proxy, subscribe } = createUseStore(original);
      const selectorForIndex = (i: number) => () =>
        original.items[i].meta.levelOne.levelTwo.value;
      const unsubFns: Array<() => void> = [];

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;
      for (let i = 0; i < N_SUBSCRIBERS; i++) {
        const selector = selectorForIndex(i);
        const unsub = subscribe(() => {
          calc(selector());
        });
        unsubFns.push(unsub);
      }

      heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

      for (let u = 0; u < N_UPDATES; u++) {
        proxy.items[u].meta.levelOne.levelTwo.value = u + 1;
      }

      for (const u of unsubFns) u();

      const retained = getMemoryUsage();

      memoryMetrics.push({
        heapUsedBefore,
        heapUsedAfter,
        heapUsedPeak,
        retained,
      });
    }

    const memoryStats = {
      heapUsedBefore: memoryMetrics[0]?.heapUsedBefore || 0,
      heapUsedAfter: memoryMetrics[0]?.heapUsedAfter || 0,
      heapUsedPeak: Math.max(...memoryMetrics.map((m) => m.heapUsedPeak)),
      retained: memoryMetrics[memoryMetrics.length - 1]?.retained || 0,
    };

    console.table({
      "our-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
        count,
      },
    });
  });
});
