import { configureStore } from "@reduxjs/toolkit";
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

type BenchAction = {
  type: "set";
  payload: { idx: number; value: number };
};

describe("bench - redux", () => {
  it("measures subscribe and updates for array workload", () => {
    const subscribeTimings: number[] = [];
    const updateTimings: number[] = [];

    for (let run = 0; run < RUNS; run++) {
      const initial = makeArrayState();

      function reducer(state = initial, action: BenchAction) {
        if (action.type === "set") {
          const { idx, value } = action.payload;
          const items = state.items.slice();
          items[idx] = { ...items[idx], value };
          return { ...state, items };
        }
        return state;
      }

      const store = configureStore({
        reducer,
        devTools: false,
        middleware: (getDefaultMiddleware) =>
          getDefaultMiddleware({
            thunk: false,
            immutableCheck: false,
            serializableCheck: false,
            actionCreatorCheck: false,
          }),
      });

      const selectorForIndex = (i: number) => () =>
        store.getState().items[i].value;

      const subscribeTime = time(() => {
        for (let i = 0; i < N_SUBSCRIBERS; i++) {
          const idx = i % ARRAY_SIZE;
          let last = selectorForIndex(idx)();
          store.subscribe(() => {
            const v = selectorForIndex(idx)();
            if (v !== last) last = v;
          });
        }
      });

      const updateTime = time(() => {
        for (let u = 0; u < N_UPDATES; u++) {
          const idx = Math.floor(Math.random() * ARRAY_SIZE);
          store.dispatch({
            type: "set",
            payload: { idx, value: Math.random() },
          });
        }
      });

      subscribeTimings.push(subscribeTime);
      updateTimings.push(updateTime);
    }

    const subClean = removeOutliers(subscribeTimings);
    const updClean = removeOutliers(updateTimings);

    const subStats = computeStats(subClean);
    const updStats = computeStats(updClean);

    // eslint-disable-next-line no-console
    console.table({
      "redux-subscribe": {
        mean: subStats.mean.toFixed(2),
        median: subStats.median.toFixed(2),
        min: subStats.min.toFixed(2),
        max: subStats.max.toFixed(2),
      },
      "redux-update": {
        mean: updStats.mean.toFixed(2),
        median: updStats.median.toFixed(2),
        min: updStats.min.toFixed(2),
        max: updStats.max.toFixed(2),
      },
    });
  });
});
