import { configureStore } from "@reduxjs/toolkit";
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

type BenchAction = {
  type: "set";
  payload: { idx: number; value: number };
};

describe("bench - redux", () => {
  it("measures subscribe and update for array workload (no view)", () => {
    const memoryMetrics: MemoryMetrics[] = [];

    for (let run = 0; run < RUNS; run++) {
      const heapUsedBefore = getMemoryUsage();
      const initial = makeArrayState();

      function reducer(state = initial, action: BenchAction) {
        if (action.type === "set") {
          const { idx, value } = action.payload;
          const items = state.items.slice();
          items[idx] = {
            ...items[idx],
            meta: {
              ...items[idx].meta,
              levelOne: {
                ...items[idx].meta.levelOne,
                levelTwo: {
                  ...items[idx].meta.levelOne.levelTwo,
                  value,
                },
              },
            },
          };
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
        store.getState().items[i].meta.levelOne.levelTwo.value;
      const unsubscribeFns: Array<() => void> = [];

      for (let i = 0; i < N_SUBSCRIBERS; i++) {
        const idx = i % ARRAY_SIZE;
        let last = selectorForIndex(idx)();
        const unsubscribe = store.subscribe(() => {
          const v = selectorForIndex(idx)();
          if (v !== last) last = v;
        });
        unsubscribeFns.push(unsubscribe);
      }

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;

      for (let u = 0; u < N_UPDATES; u++) {
        const idx = Math.floor(Math.random() * ARRAY_SIZE);
        store.dispatch({
          type: "set",
          payload: { idx, value: Math.random() },
        });
      }

      heapUsedPeak = Math.max(heapUsedPeak, getMemoryUsage());

      for (const unsubscribe of unsubscribeFns) {
        unsubscribe();
      }

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
      "redux-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
      },
    });
  });
});
