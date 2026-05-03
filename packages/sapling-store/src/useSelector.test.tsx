import { act, createElement } from "react";
import { createRoot } from "react-dom/client";
import { describe, expect, it } from "vitest";
import { createStore } from "./useSelector";

describe("useSelector", () => {
  it("only rerenders when the selected snapshot changes", async () => {
    (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

    const { store, useSelector } = createStore({
      user: {
        name: "John",
        age: 30,
      },
      settings: {
        theme: "dark",
      },
    });

    let renderCount = 0;
    let latestName = "";

    const container = document.createElement("div");
    const root = createRoot(container);

    function Reader() {
      const name = useSelector(() => store.user.name);
      renderCount += 1;
      latestName = name;
      return null;
    }

    try {
      await act(async () => {
        root.render(createElement(Reader));
      });

      expect(renderCount).toBe(1);
      expect(latestName).toBe("John");

      await act(async () => {
        store.settings.theme = "light";
      });

      expect(renderCount).toBe(1);
      expect(latestName).toBe("John");

      await act(async () => {
        store.user.name = "Jane";
      });

      expect(renderCount).toBe(2);
      expect(latestName).toBe("Jane");
    } finally {
      await act(async () => {
        root.unmount();
      });
    }
  });
});
