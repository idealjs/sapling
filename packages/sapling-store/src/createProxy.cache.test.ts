import { describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

describe("createProxy - deleteProperty behavior", () => {
  it("should track property deletion with delete operation", () => {
    const target = { value1: { value2: { value3: { value: 1 } } } };
    const notifyPaths: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      notifyPaths.push({ paths, operation });
    };
    const proxy = createProxy(target, { notifySet: notify });

    expect(proxy.value1.value2.value3.value).toBe(1);
    proxy.value1.value2.value3.value = 2;
    expect(notifyPaths).toMatchInlineSnapshot(`
      [
        {
          "operation": "get",
          "paths": [
            "value1",
          ],
        },
        {
          "operation": "get",
          "paths": [
            "value1",
            "value2",
          ],
        },
        {
          "operation": "get",
          "paths": [
            "value1",
            "value2",
            "value3",
          ],
        },
        {
          "operation": "get",
          "paths": [
            "value1",
            "value2",
            "value3",
            "value",
          ],
        },
        {
          "operation": "get",
          "paths": [
            "value1",
          ],
        },
        {
          "operation": "get",
          "paths": [
            "value1",
            "value2",
          ],
        },
        {
          "operation": "get",
          "paths": [
            "value1",
            "value2",
            "value3",
          ],
        },
        {
          "operation": "set",
          "paths": [
            "value1",
            "value2",
            "value3",
            "value",
          ],
        },
      ]
    `);
  });
});
