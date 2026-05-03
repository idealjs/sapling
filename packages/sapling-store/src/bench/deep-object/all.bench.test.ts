import { configureStore } from "@reduxjs/toolkit";
import { act, createElement, Fragment, useSyncExternalStore } from "react";
import { createRoot } from "react-dom/client";
import { proxy, useSnapshot } from "valtio";
import { describe, expect, it } from "vitest";
import { create } from "zustand";
import { createStore as ourCreateStore } from "../../useSelector";
import {
  ARRAY_SIZE,
  computeStats,
  N_SUBSCRIBERS,
  N_UPDATES,
  RUNS,
  removeOutliers,
  time,
} from "../bench.utils";

type StatResult = { mean: number; median: number; min: number; max: number };
type LibName = "redux" | "valtio" | "zustand" | "our";

type BenchResults = Record<
  LibName,
  { subscribe: StatResult; update: StatResult }
>;

type Action = {
  type: "set";
  payload: { idx: number; value: number };
};

type DeepItem = {
  id: number;
  meta: {
    levelOne: {
      levelTwo: {
        value: number;
      };
    };
  };
};

type DeepArrayState = {
  items: DeepItem[];
  setValue: (idx: number, value: number) => void;
};

const LIBRARIES: LibName[] = ["redux", "valtio", "zustand", "our"];

const makeDeepArrayState = (): { items: DeepItem[] } => ({
  items: Array.from({ length: ARRAY_SIZE }, (_, i) => ({
    id: i,
    meta: {
      levelOne: {
        levelTwo: {
          value: i,
        },
      },
    },
  })),
});

describe("bench - all stores comparison for deep objects", () => {
  it("measures all stores subscribe and update performance", async () => {
    const results: BenchResults = {
      redux: {
        subscribe: { mean: 0, median: 0, min: 0, max: 0 },
        update: { mean: 0, median: 0, min: 0, max: 0 },
      },
      valtio: {
        subscribe: { mean: 0, median: 0, min: 0, max: 0 },
        update: { mean: 0, median: 0, min: 0, max: 0 },
      },
      zustand: {
        subscribe: { mean: 0, median: 0, min: 0, max: 0 },
        update: { mean: 0, median: 0, min: 0, max: 0 },
      },
      our: {
        subscribe: { mean: 0, median: 0, min: 0, max: 0 },
        update: { mean: 0, median: 0, min: 0, max: 0 },
      },
    };

    (
      globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }
    ).IS_REACT_ACT_ENVIRONMENT = true;

    // Redux benchmark (rendered readers + deterministic updates)
    {
      const subscribeTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const initial = makeDeepArrayState();
        const container = document.createElement("div");
        const root = createRoot(container);

        function reducer(state = initial, action: Action) {
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

        function Reader({ index }: { index: number }) {
          const value = useSyncExternalStore(store.subscribe, () =>
            selectorForIndex(index)(),
          );
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

        const subscribeTime = time(() => {
          act(() => root.render(createElement(App)));
        });

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("0");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            store.dispatch({ type: "set", payload: { idx, value: idx + 1 } });
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("1");
        expect(container.querySelector('[data-slot="1"]')?.textContent).toBe("2");

        act(() => root.unmount());

        subscribeTimings.push(subscribeTime);
        updateTimings.push(updateTime);
      }

      results.redux.subscribe = computeStats(removeOutliers(subscribeTimings));
      results.redux.update = computeStats(removeOutliers(updateTimings));
    }

    // Valtio benchmark (rendered readers + deterministic updates)
    {
      const subscribeTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const container = document.createElement("div");
        const root = createRoot(container);
        const state = proxy(makeDeepArrayState());

        function Reader({ index }: { index: number }) {
          const snapshot = useSnapshot(state.items[index].meta.levelOne.levelTwo);
          return createElement("span", { "data-slot": index }, String(snapshot.value));
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

        const subscribeTime = time(() => {
          act(() => root.render(createElement(App)));
        });

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("0");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            state.items[idx].meta.levelOne.levelTwo.value = idx + 1;
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("1");
        expect(container.querySelector('[data-slot="1"]')?.textContent).toBe("2");

        act(() => root.unmount());

        subscribeTimings.push(subscribeTime);
        updateTimings.push(updateTime);
      }

      results.valtio.subscribe = computeStats(removeOutliers(subscribeTimings));
      results.valtio.update = computeStats(removeOutliers(updateTimings));
    }

    // Zustand benchmark (rendered readers + deterministic updates)
    {
      const subscribeTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const initial = makeDeepArrayState();
        const container = document.createElement("div");
        const root = createRoot(container);

        const useStore = create<DeepArrayState>((set) => ({
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

        function Reader({ index }: { index: number }) {
          const value = useStore((state) => state.items[index].meta.levelOne.levelTwo.value);
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

        const subscribeTime = time(() => {
          act(() => root.render(createElement(App)));
        });

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("0");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            useStore.getState().setValue(idx, idx + 1);
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("1");
        expect(container.querySelector('[data-slot="1"]')?.textContent).toBe("2");

        act(() => root.unmount());

        subscribeTimings.push(subscribeTime);
        updateTimings.push(updateTime);
      }

      results.zustand.subscribe = computeStats(removeOutliers(subscribeTimings));
      results.zustand.update = computeStats(removeOutliers(updateTimings));
    }

    // Our store benchmark (rendered readers + deterministic updates)
    {
      const subscribeTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const container = document.createElement("div");
        const root = createRoot(container);
        const { store, useSelector } = ourCreateStore(makeDeepArrayState());

        function Reader({ index }: { index: number }) {
          const value = useSelector(() => store.items[index].meta.levelOne.levelTwo.value);
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

        const subscribeTime = time(() => {
          act(() => root.render(createElement(App)));
        });

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("0");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            store.items[idx].meta.levelOne.levelTwo.value = idx + 1;
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(container.querySelector('[data-slot="0"]')?.textContent).toBe("1");
        expect(container.querySelector('[data-slot="1"]')?.textContent).toBe("2");

        act(() => root.unmount());

        subscribeTimings.push(subscribeTime);
        updateTimings.push(updateTime);
      }

      results.our.subscribe = computeStats(removeOutliers(subscribeTimings));
      results.our.update = computeStats(removeOutliers(updateTimings));
    }

    const comparisonTable: Record<
      string,
      { subscribe: string; update: string }
    > = {};
    for (const lib of LIBRARIES) {
      comparisonTable[lib] = {
        subscribe: `${results[lib].subscribe.mean.toFixed(2)}ms (±${(results[lib].subscribe.max - results[lib].subscribe.min).toFixed(2)})`,
        update: `${results[lib].update.mean.toFixed(2)}ms (±${(results[lib].update.max - results[lib].update.min).toFixed(2)})`,
      };
    }

    // eslint-disable-next-line no-console
    console.log("\n📊 Benchmark Results (10 runs, outliers removed):\n");
    // eslint-disable-next-line no-console
    console.table(comparisonTable);

    // eslint-disable-next-line no-console
    console.log("\nDetailed stats (mean | median | min | max):");
    for (const lib of LIBRARIES) {
      const s = results[lib].subscribe;
      const u = results[lib].update;
      // eslint-disable-next-line no-console
      console.log(
        `${lib.padEnd(10)} subscribe: ${s.mean.toFixed(2).padEnd(7)} | ${s.median.toFixed(2).padEnd(7)} | ${s.min.toFixed(2).padEnd(7)} | ${s.max.toFixed(2)}`,
      );
      // eslint-disable-next-line no-console
      console.log(
        `${"".padEnd(10)} update:    ${u.mean.toFixed(2).padEnd(7)} | ${u.median.toFixed(2).padEnd(7)} | ${u.min.toFixed(2).padEnd(7)} | ${u.max.toFixed(2)}`,
      );
    }
  });
});
