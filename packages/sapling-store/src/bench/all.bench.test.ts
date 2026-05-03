import { configureStore } from "@reduxjs/toolkit";
import { act, createElement, Fragment, useSyncExternalStore } from "react";
import { createRoot } from "react-dom/client";
import { proxy, useSnapshot } from "valtio";
import { describe, expect, it } from "vitest";
import { create } from "zustand";
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

type StatResult = { mean: number; median: number; min: number; max: number };
type LibName = "redux" | "valtio" | "zustand" | "our";

type BenchResults = Record<LibName, { mount: StatResult; update: StatResult }>;

type Action = {
  type: "set";
  payload: { idx: number; value: number };
};

type Item = { id: number; value: number };
type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

const LIBRARIES: LibName[] = ["redux", "valtio", "zustand", "our"];
const UPDATED_SLOT = 0;
const UNCHANGED_SLOT = 1;

function getSlotText(container: HTMLElement, slot: number) {
  const node = container.querySelector(`[data-slot="${slot}"]`);

  if (!(node instanceof HTMLElement)) {
    throw new Error(`Missing rendered slot ${slot}`);
  }

  return node.textContent ?? "";
}

const emptyStats = (): StatResult => ({
  mean: 0,
  median: 0,
  min: 0,
  max: 0,
});

const stats = (values: number[]) => computeStats(removeOutliers(values));

describe("bench - all stores comparison", () => {
  it("measures all stores mount and update performance", {
    timeout: 30000,
  }, async () => {
    const results: BenchResults = {
      redux: {
        mount: emptyStats(),
        update: emptyStats(),
      },
      valtio: {
        mount: emptyStats(),
        update: emptyStats(),
      },
      zustand: {
        mount: emptyStats(),
        update: emptyStats(),
      },
      our: {
        mount: emptyStats(),
        update: emptyStats(),
      },
    };

    (
      globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }
    ).IS_REACT_ACT_ENVIRONMENT = true;

    // Redux benchmark
    {
      const mountTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const initial = makeArrayState();
        const container = document.createElement("div");
        const root = createRoot(container);

        function reducer(state = initial, action: Action) {
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

        function Reader({ index }: { index: number }) {
          const value = useSyncExternalStore(store.subscribe, () =>
            selectorForIndex(index)(),
          );
          return createElement("span", { "data-slot": index }, value);
        }

        function App() {
          return createElement(
            Fragment,
            null,
            Array.from({ length: N_SUBSCRIBERS }, (_, i) =>
              createElement(Reader, {
                key: i,
                index: i % ARRAY_SIZE,
              }),
            ),
          );
        }

        const mountTime = time(() => {
          act(() => {
            root.render(createElement(App));
          });
        });

        expect(getSlotText(container, UPDATED_SLOT)).toBe("0");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("1");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            store.dispatch({
              type: "set",
              payload: { idx, value: idx + 1 },
            });
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(getSlotText(container, UPDATED_SLOT)).toBe("1");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("2");

        act(() => {
          root.unmount();
        });

        mountTimings.push(mountTime);
        updateTimings.push(updateTime);
      }

      results.redux.mount = stats(mountTimings);
      results.redux.update = stats(updateTimings);
    }

    // Valtio benchmark
    {
      const mountTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const container = document.createElement("div");
        const root = createRoot(container);
        const state = proxy(makeArrayState());

        function Reader({ index }: { index: number }) {
          const snapshot = useSnapshot(state.items[index]);
          const value = snapshot.value;
          return createElement("span", { "data-slot": index }, value);
        }

        function App() {
          return createElement(
            Fragment,
            null,
            Array.from({ length: N_SUBSCRIBERS }, (_, i) =>
              createElement(Reader, {
                key: i,
                index: i % ARRAY_SIZE,
              }),
            ),
          );
        }

        const mountTime = time(() => {
          act(() => {
            root.render(createElement(App));
          });
        });

        expect(getSlotText(container, UPDATED_SLOT)).toBe("0");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("1");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            state.items[idx].value = idx + 1;
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(getSlotText(container, UPDATED_SLOT)).toBe("1");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("2");

        act(() => {
          root.unmount();
        });

        mountTimings.push(mountTime);
        updateTimings.push(updateTime);
      }

      results.valtio.mount = stats(mountTimings);
      results.valtio.update = stats(updateTimings);
    }

    // Zustand benchmark
    {
      const mountTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
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
          return createElement("span", { "data-slot": index }, value);
        }

        function App() {
          return createElement(
            Fragment,
            null,
            Array.from({ length: N_SUBSCRIBERS }, (_, i) =>
              createElement(Reader, {
                key: i,
                index: i % ARRAY_SIZE,
              }),
            ),
          );
        }

        const mountTime = time(() => {
          act(() => {
            root.render(createElement(App));
          });
        });

        expect(getSlotText(container, UPDATED_SLOT)).toBe("0");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("1");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            useStore.getState().setValue(idx, idx + 1);
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(getSlotText(container, UPDATED_SLOT)).toBe("1");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("2");

        act(() => {
          root.unmount();
        });

        mountTimings.push(mountTime);
        updateTimings.push(updateTime);
      }

      results.zustand.mount = stats(mountTimings);
      results.zustand.update = stats(updateTimings);
    }

    // Our store benchmark
    {
      const mountTimings: number[] = [];
      const updateTimings: number[] = [];

      for (let run = 0; run < RUNS; run++) {
        const container = document.createElement("div");
        const root = createRoot(container);
        const { store, useSelector } = ourCreateStore(makeArrayState());

        function Reader({ index }: { index: number }) {
          const value = useSelector(() => store.items[index].value);
          return createElement("span", { "data-slot": index }, value);
        }

        function App() {
          return createElement(
            Fragment,
            null,
            Array.from({ length: N_SUBSCRIBERS }, (_, i) =>
              createElement(Reader, {
                key: i,
                index: i % ARRAY_SIZE,
              }),
            ),
          );
        }

        const mountTime = time(() => {
          act(() => {
            root.render(createElement(App));
          });
        });

        expect(getSlotText(container, UPDATED_SLOT)).toBe("0");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("1");

        const updateStart = performance.now();
        await act(async () => {
          for (let idx = 0; idx < N_UPDATES; idx++) {
            store.items[idx].value = idx + 1;
          }
        });
        const updateTime = performance.now() - updateStart;

        expect(getSlotText(container, UPDATED_SLOT)).toBe("1");
        expect(getSlotText(container, UNCHANGED_SLOT)).toBe("2");

        act(() => {
          root.unmount();
        });

        mountTimings.push(mountTime);
        updateTimings.push(updateTime);
      }

      results.our.mount = stats(mountTimings);
      results.our.update = stats(updateTimings);
    }

    const comparisonTable: Record<string, { mount: string; update: string }> =
      {};
    for (const lib of LIBRARIES) {
      comparisonTable[lib] = {
        mount: `${results[lib].mount.mean.toFixed(2)}ms (±${(results[lib].mount.max - results[lib].mount.min).toFixed(2)})`,
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
      const s = results[lib].mount;
      const u = results[lib].update;
      // eslint-disable-next-line no-console
      console.log(
        `${lib.padEnd(10)} mount:     ${s.mean.toFixed(2).padEnd(7)} | ${s.median.toFixed(2).padEnd(7)} | ${s.min.toFixed(2).padEnd(7)} | ${s.max.toFixed(2)}`,
      );
      // eslint-disable-next-line no-console
      console.log(
        `${"".padEnd(10)} update:    ${u.mean.toFixed(2).padEnd(7)} | ${u.median.toFixed(2).padEnd(7)} | ${u.min.toFixed(2).padEnd(7)} | ${u.max.toFixed(2)}`,
      );
    }
  });
});
