import { describe, expect, it } from "vitest";
import { create } from "zustand";
import { makeArrayState } from "../bench.utils";

type Item = { id: number; meta: { levelOne: { levelTwo: { value: number } } } };

type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

describe("calc 500000500", () => {
  it("test zustand calc", () => {
    let count = 0;
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

    const calc = (v: number) => {
      count += v;
    };
    for (let u = 0; u < 1000; u++) {
      for (let i = 0; i < 1000; i++) {
        if (i <= u) {
          calc(useStore.getState().items[u].meta.levelOne.levelTwo.value + 1);
        } else {
          calc(useStore.getState().items[u].meta.levelOne.levelTwo.value);
        }
      }
    }
    expect(count).toBe(500000500);
    console.table({
      "zustand-500000500": {
        count,
      },
    });
  });
});
