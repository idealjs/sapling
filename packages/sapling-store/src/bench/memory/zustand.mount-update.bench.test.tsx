import { act, createElement, Fragment } from "react";
import { createRoot } from "react-dom/client";
import { describe, it } from "vitest";
import { create } from "zustand";
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

type Item = { id: number; value: number };
type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

describe("bench - zustand", () => {
  it(
    "measures mount and update for array workload",
    { timeout: 30000 },
    () => {
      const mountTimings: number[] = [];
      const updateTimings: number[] = [];
      const memoryMetrics: MemoryMetrics[] = [];

      (
        globalThis as typeof globalThis & {
          IS_REACT_ACT_ENVIRONMENT?: boolean;
        }
      ).IS_REACT_ACT_ENVIRONMENT = true;

      for (let run = 0; run < RUNS; run++) {
        const heapUsedBefore = getMemoryUsage();
        const initial = makeArrayState();
        const container = document.createElement("div");
        const root = createRoot(container);

        const useStore = create<ArrayState>((set) => ({
          items: initial.items,
          setValue: (idx: number, value: number) =>
            set((state) => ({
              items: state.items.map((item, i) =>
                i === idx ? { ...item, value } : item,
              ),
            })),
        }));

        function Reader({ index }: { index: number }) {
          const value = useStore((state) => state.items[index].value);
          return createElement("span", { "data-slot": index }, String(value));
        }

        function App() {
          return createElement(
            Fragment,
            null,
            Array.from({ length: N_SUBSCRIBERS }, (_, i) =>
              createElement(Reader, { key: i, index: i % ARRAY_SIZE }),
            ),
          );
        }

        const heapUsedAfter = getMemoryUsage();
        let heapUsedPeak = heapUsedAfter;

        const mountTime = time(() => {
          act(() => {
            root.render(createElement(App));
          });
        });

        const updateStart = performance.now();
        act(() => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            useStore.getState().setValue(idx, Math.random());
          }
        });
        const updateTime = performance.now() - updateStart;

        heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

        act(() => {
          root.unmount();
        });

        const retained = getMemoryUsage();

        mountTimings.push(mountTime);
        updateTimings.push(updateTime);
        memoryMetrics.push({
          heapUsedBefore,
          heapUsedAfter,
          heapUsedPeak,
          retained,
        });
      }

      const mountClean = removeOutliers(mountTimings);
      const updClean = removeOutliers(updateTimings);

      const mountStats = computeStats(mountClean);
      const updStats = computeStats(updClean);
      const memoryStats = {
        heapUsedBefore: memoryMetrics[0]?.heapUsedBefore || 0,
        heapUsedAfter: memoryMetrics[0]?.heapUsedAfter || 0,
        heapUsedPeak: Math.max(...memoryMetrics.map((m) => m.heapUsedPeak)),
        retained: memoryMetrics[memoryMetrics.length - 1]?.retained || 0,
      };

      // eslint-disable-next-line no-console
      console.table({
        "zustand-mount": {
          mean: mountStats.mean.toFixed(2),
          median: mountStats.median.toFixed(2),
          min: mountStats.min.toFixed(2),
          max: mountStats.max.toFixed(2),
        },
        "zustand-update": {
          mean: updStats.mean.toFixed(2),
          median: updStats.median.toFixed(2),
          min: updStats.min.toFixed(2),
          max: updStats.max.toFixed(2),
        },
        "zustand-memory": {
          before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
          after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
          peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
          retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
        },
      });
    },
  );
});
