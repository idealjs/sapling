import { describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

describe("createProxy - deleteProperty behavior", () => {
  it("should track property deletion with delete operation", () => {
    const target: { name?: string; age?: number } = { name: "John", age: 30 };
    const notifyPaths: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      notifyPaths.push({ paths, operation });
    };
    const proxy = createProxy(target, { notifySet: notify, notifyGet: notify });

    delete proxy.name;

    expect("name" in proxy).toBe(false);
    expect("name" in target).toBe(false);
    expect(notifyPaths).toEqual([{ paths: ["name"], operation: "delete" }]);
  });
});
