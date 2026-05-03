import { create } from "zustand";
import { describe, it } from "vitest";
import {
  ARRAY_SIZE,
  computeStats,
  makeArrayState,
  N_SUBSCRIBERS,
  N_UPDATES,
  RUNS,
  removeOutliers,
  time,
} from "./bench.utils";

type Item = { id: number; value: number };
type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

describe("bench - zustand", () => {
  it("measures subscribe and updates for array workload", () => {
    const subscribeTimings: number[] = [];
    const updateTimings: number[] = [];

    for (let run = 0; run < RUNS; run++) {
      const initial = makeArrayState();

      const useStore = create<ArrayState>((set) => ({
        items: initial.items,
        setValue: (idx: number, value: number) =>
          set((state) => ({
            items: state.items.map((item, i) =>
              i === idx ? { ...item, value } : item,
            ),
          })),
      }));

      const selectorForIndex = (i: number) => () =>
        useStore.getState().items[i].value;

      const unsubscribeFns: Array<() => void> = [];

      const subscribeTime = time(() => {
        for (let i = 0; i < N_SUBSCRIBERS; i++) {
          const idx = i % ARRAY_SIZE;
          const unsub = useStore.subscribe(() => {
            selectorForIndex(idx)();
          });
          unsubscribeFns.push(unsub);
        }
      });

      const updateTime = time(() => {
        for (let u = 0; u < N_UPDATES; u++) {
          const idx = Math.floor(Math.random() * ARRAY_SIZE);
          useStore.getState().setValue(idx, Math.random());
        }
      });

      for (const u of unsubscribeFns) u();

      subscribeTimings.push(subscribeTime);
      updateTimings.push(updateTime);
    }

    const subClean = removeOutliers(subscribeTimings);
    const updClean = removeOutliers(updateTimings);

    const subStats = computeStats(subClean);
    const updStats = computeStats(updClean);

    // eslint-disable-next-line no-console
    console.table({
      "zustand-subscribe": {
        mean: subStats.mean.toFixed(2),
        median: subStats.median.toFixed(2),
        min: subStats.min.toFixed(2),
        max: subStats.max.toFixed(2),
      },
      "zustand-update": {
        mean: updStats.mean.toFixed(2),
        median: updStats.median.toFixed(2),
        min: updStats.min.toFixed(2),
        max: updStats.max.toFixed(2),
      },
    });
  });
});
