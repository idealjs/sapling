import { act } from "react";
import { createRoot } from "react-dom/client";
import { describe, expect, it } from "vitest";
import { makeArrayState } from "../bench/bench.utils";
import createUseStore from "../createUseStore";

describe("our store - object mount and update", () => {
  it("should update view when object properties change", () => {
    (
      globalThis as typeof globalThis & {
        IS_REACT_ACT_ENVIRONMENT?: boolean;
      }
    ).IS_REACT_ACT_ENVIRONMENT = true;

    const initial = makeArrayState();
    const useStore = createUseStore(initial);
    const container = document.createElement("div");
    const root = createRoot(container);

    function Reader({ index }: { index: number }) {
      // Reading the entire meta object instead of just the primitive value
      const value = useStore(
        (value) => value.items[index].meta.levelOne.levelTwo.value,
      );
      return <span data-slot={index}>{String(value)}</span>;
    }

    function App() {
      const items = useStore((value) => value.items.length);
      return (
        <>
          {new Array(items).fill(null).map((_, index) => (
            <Reader
              key={useStore.originalValue.items[index].id}
              index={useStore.originalValue.items[index].id}
            />
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

    // Update object properties without changing the object reference
    const newValues: number[] = [];
    act(() => {
      for (const item of useStore.proxy.items) {
        const newValue = Math.random();
        newValues.push(newValue);
        item.meta.levelOne.levelTwo.value = newValue;
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
