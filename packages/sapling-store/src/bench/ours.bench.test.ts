import { describe, it } from "vitest";
import { createStore as ourCreateStore } from "../useSelector";
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

describe("bench - our store", () => {
  it("measures subscribe and updates for array workload", () => {
    const subscribeTimings: number[] = [];
    const updateTimings: number[] = [];

    for (let run = 0; run < RUNS; run++) {
      const { store, subscribeSelector } = ourCreateStore(makeArrayState());

      const selectorForIndex = (i: number) => () => store.items[i].value;

      const unsubFns: Array<() => void> = [];

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

      const updateTime = time(() => {
        for (let u = 0; u < N_UPDATES; u++) {
          const idx = Math.floor(Math.random() * ARRAY_SIZE);
          store.items[idx].value = Math.random();
        }
      });

      for (const u of unsubFns) u();

      subscribeTimings.push(subscribeTime);
      updateTimings.push(updateTime);
    }

    const subClean = removeOutliers(subscribeTimings);
    const updClean = removeOutliers(updateTimings);

    const subStats = computeStats(subClean);
    const updStats = computeStats(updClean);

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
    });
  });
});
