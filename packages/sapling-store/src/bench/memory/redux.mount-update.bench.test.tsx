import { configureStore } from "@reduxjs/toolkit";
import { act } from "react";
import { createRoot } from "react-dom/client";
import { Provider, useSelector } from "react-redux";
import { describe, it } from "vitest";
import {
  formatMB,
  getMemoryUsage,
  type MemoryMetrics,
  makeArrayState,
  RUNS,
} from "../bench.utils";

type BenchAction = {
  type: "set";
  payload: { idx: number; value: number };
};

type ArrayState = ReturnType<typeof makeArrayState>;

const selectItems = (state: ArrayState) => state.items;

const selectItemValue = (state: ArrayState, index: number) =>
  state.items[index].meta.levelOne.levelTwo.value;

describe("bench - redux", () => {
  it("measures mount and update for array workload", { timeout: 30000 }, () => {
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

      function Reader({ index }: { index: number }) {
        const value = useSelector((state: ArrayState) =>
          selectItemValue(state, index),
        );
        return <span data-slot={index}>{String(value)}</span>;
      }

      function App() {
        const items = useSelector((state: ArrayState) => selectItems(state));

        return (
          <>
            {items.map((item) => (
              <Reader key={item.id} index={item.id} />
            ))}
          </>
        );
      }

      act(() => {
        root.render(
          <Provider store={store}>
            <App />
          </Provider>,
        );
      });

      const heapUsedAfter = getMemoryUsage();
      let heapUsedPeak = heapUsedAfter;

      act(() => {
        for (const item of selectItems(store.getState())) {
          store.dispatch({
            type: "set",
            payload: { idx: item.id, value: Math.random() },
          });
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
      "redux-memory": {
        before: `${formatMB(memoryStats.heapUsedBefore * 1024 * 1024)} MB`,
        after: `${formatMB(memoryStats.heapUsedAfter * 1024 * 1024)} MB`,
        peak: `${formatMB(memoryStats.heapUsedPeak * 1024 * 1024)} MB`,
        retained: `${formatMB(memoryStats.retained * 1024 * 1024)} MB`,
      },
    });
  });
});
