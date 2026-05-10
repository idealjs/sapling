import { act } from "react";
import { createRoot } from "react-dom/client";
import { describe, expect, it } from "vitest";
import { create } from "zustand";
import { makeArrayState } from "../bench/bench.utils";

type Item = { id: number; meta: { levelOne: { levelTwo: { value: number } } } };
type ArrayState = {
  items: Item[];
  setValue: (idx: number, value: number) => void;
};

describe("zustand - object mount and update", () => {
  it("should update view when object properties change", () => {
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

    const container = document.createElement("div");
    const root = createRoot(container);

    function Reader({ index }: { index: number }) {
      // Reading the entire meta object instead of just the primitive value
      const meta = useStore((state) => state.items[index].meta);
      return (
        <span data-slot={index}>{String(meta.levelOne.levelTwo.value)}</span>
      );
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

    // Verify initial render
    let spans = container.querySelectorAll("span");
    expect(spans.length).toBe(initial.items.length);

    // Update object properties
    const newValues: number[] = [];
    act(() => {
      for (const item of useStore.getState().items) {
        const newValue = Math.random();
        newValues.push(newValue);
        useStore.getState().setValue(item.id, newValue);
      }
    });

    // Verify view is updated with new values
    spans = container.querySelectorAll("span");
    const updatedValues = Array.from(spans).map((span) => span.textContent);
    updatedValues.forEach((value, index) => {
      expect(value).toBe(String(newValues[index]));
    });

    act(() => {
      root.unmount();
    });
  });
});
