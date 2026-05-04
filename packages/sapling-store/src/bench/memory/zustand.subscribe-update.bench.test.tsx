import { describe, it } from "vitest";
import { create } from "zustand";
import {
  formatMB,
  getMemoryUsage,
  type MemoryMetrics,
  makeArrayState,
  N_SUBSCRIBERS,
  N_UPDATES,
  RUNS,
} from "../bench.utils";

type Item = { id: number; meta: { levelOne: { levelTwo: { value: number } } } };
type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

describe("bench - zustand", () => {
  it("measures subscribe and update for array workload (no view)", () => {
    const memoryMetrics: MemoryMetrics[] = [];
    let count = 0;
    const calc = (v: number) => {
      count += v;
    };
    for (let run = 0; run < RUNS; run++) {
      const heapUsedBefore = getMemoryUsage();
      const initial = makeArrayState();

      const useStore = create<ArrayState>((set) => ({
        items: initial.items,
        setValue: (idx: number, value: number) =>
          set((state) => ({
            items: state.items.map((item, i) =>
              i === idx
                ? {
                    ...item,
                    meta: {
                      ...item.meta,
                      levelOne: {
                        ...item.meta.levelOne,
                        levelTwo: {
                          ...item.meta.levelOne.levelTwo,
                          value,
                        },
                      },
                    },
                  }
                : item,
            ),
          })),
      }));

      const selectorForIndex = (i: number) => () =>
        useStore.getState().items[i].meta.levelOne.levelTwo.value;
      const unsubscribeFns: Array<() => void> = [];

      for (let i = 0; i < N_SUBSCRIBERS; i++) {
        const selector = selectorForIndex(i);
        const unsub = useStore.subscribe(() => {
          calc(selector());
        });
        unsubscribeFns.push(unsub);
      }

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;

      for (let u = 0; u < N_UPDATES; u++) {
        useStore.getState().setValue(u, u + 1);
      }

      heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

      for (const u of unsubscribeFns) u();

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
      "zustand-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
        count,
      },
    });
  });
});
