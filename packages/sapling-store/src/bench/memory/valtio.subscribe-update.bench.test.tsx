import { proxy, subscribe as valtioSubscribe } from "valtio";
import { describe, it } from "vitest";
import {
  ARRAY_SIZE,
  formatMB,
  getMemoryUsage,
  type MemoryMetrics,
  makeArrayState,
  N_SUBSCRIBERS,
  N_UPDATES,
  RUNS,
} from "../bench.utils";

describe("bench - valtio", () => {
  it("measures subscribe and update for array workload (no view)", () => {
    const memoryMetrics: MemoryMetrics[] = [];

    for (let run = 0; run < RUNS; run++) {
      const heapUsedBefore = getMemoryUsage();
      const state = proxy(makeArrayState());

      const selectorForIndex = (i: number) => () =>
        state.items[i].meta.levelOne.levelTwo.value;
      const subs: Array<() => void> = [];

      for (let i = 0; i < N_SUBSCRIBERS; i++) {
        const idx = i % ARRAY_SIZE;
        const unsub = valtioSubscribe(
          state.items[i].meta.levelOne.levelTwo,
          () => {
            selectorForIndex(idx)();
          },
        );
        subs.push(unsub);
      }

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;

      for (let u = 0; u < N_UPDATES; u++) {
        const idx = Math.floor(Math.random() * ARRAY_SIZE);
        state.items[idx].meta.levelOne.levelTwo.value = Math.random();
      }

      heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

      for (const u of subs) u();

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

    // eslint-disable-next-line no-console
    console.table({
      "valtio-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
      },
    });
  });
});
