import { describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

describe("createProxy - nested object replacement", () => {
  it("should create a new proxy after replacing a nested object and keep notifying reads and writes", () => {
    const target = {
      user: {
        profile: {
          details: {
            name: "John",
          },
        },
      },
    };

    const operations: Array<{ paths: PathKey[]; operation: string }> = [];
    const notify = (paths: PathKey[], operation: string) => {
      operations.push({ paths, operation });
    };

    const proxy = createProxy(target, { notify });

    const originalProfileProxy = proxy.user.profile;

    proxy.user.profile = {
      details: {
        name: "Jane",
      },
    };

    const replacementProfileProxy = proxy.user.profile;

    expect(replacementProfileProxy).not.toBe(originalProfileProxy);
    expect(replacementProfileProxy.details.name).toBe("Jane");

    replacementProfileProxy.details.name = "Janet";

    expect(target.user.profile.details.name).toBe("Janet");
    expect(operations).toEqual(
      expect.arrayContaining([
        { paths: ["user"], operation: "get" },
        { paths: ["user", "profile"], operation: "get" },
        { paths: ["user", "profile"], operation: "set" },
        { paths: ["user"], operation: "get" },
        { paths: ["user", "profile"], operation: "get" },
        { paths: ["user", "profile", "details"], operation: "get" },
        {
          paths: ["user", "profile", "details", "name"],
          operation: "get",
        },
        {
          paths: ["user", "profile", "details", "name"],
          operation: "set",
        },
      ]),
    );
  });
});
