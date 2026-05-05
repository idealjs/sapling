import { act } from "react";
import { createRoot } from "react-dom/client";
import { describe, it } from "vitest";
import { create } from "zustand";
import {
  formatMB,
  getMemoryUsage,
  type MemoryMetrics,
  makeArrayState,
  RUNS,
} from "../bench.utils";

type Item = { id: number; meta: { levelOne: { levelTwo: { value: number } } } };
type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

describe("bench - zustand", () => {
  it("measures mount and update for array workload", { timeout: 30000 }, () => {
    const memoryMetrics: MemoryMetrics[] = [];

    (
      globalThis as typeof globalThis & {
        IS_REACT_ACT_ENVIRONMENT?: boolean;
      }
    ).IS_REACT_ACT_ENVIRONMENT = true;

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

    for (let run = 0; run < RUNS; run++) {
      const heapUsedBefore = getMemoryUsage();
      const container = document.createElement("div");
      const root = createRoot(container);

      function Reader({ index }: { index: number }) {
        const value = useStore(
          (state) => state.items[index].meta.levelOne.levelTwo.value,
        );
        return <span data-slot={index}>{String(value)}</span>;
      }

      function App() {
        const items = useStore((state) => state.items);

        return (
          <>
            {items.map((item) => (
              <Reader key={item.id} index={item.id} />
            ))}
          </>
        );
      }

      act(() => {
        root.render(<App />);
      });

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;

      act(() => {
        for (const item of useStore.getState().items) {
          useStore.getState().setValue(item.id, Math.random());
        }
      });

      heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

      act(() => {
        root.unmount();
      });

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
      "zustand-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
      },
    });
  });
});
