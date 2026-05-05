import { describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

describe("createProxy - set with notify", () => {
  it("should notify on simple property set with set operation", () => {
    const target = { name: "John", age: 30 };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy.name = "Jane";

    expect(target.name).toBe("Jane");
    expect(notifyPaths).toEqual([["name"]]);
  });

  it("should notify on multiple property sets", () => {
    const target = { name: "John", age: 30 };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy.name = "Jane";
    proxy.age = 31;

    expect(notifyPaths).toEqual([["name"], ["age"]]);
  });

  it("should notify on nested object property set", () => {
    const target = {
      user: {
        profile: {
          name: "John",
        },
      },
    };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    const userProxy = proxy.user;
    const profileProxy = userProxy.profile;
    profileProxy.name = "Jane";

    expect(target.user.profile.name).toBe("Jane");
    expect(notifyPaths).toEqual([["user", "profile", "name"]]);
  });

  it("should notify with correct path for deeply nested property set", () => {
    const target = {
      level1: {
        level2: {
          level3: {
            value: "initial",
          },
        },
      },
    };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy.level1.level2.level3.value = "updated";

    expect(target.level1.level2.level3.value).toBe("updated");
    expect(notifyPaths).toEqual([["level1", "level2", "level3", "value"]]);
  });

  it("should notify multiple nested property sets with correct paths", () => {
    const target = {
      user: {
        profile: {
          name: "John",
          email: "john@example.com",
        },
      },
    };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy.user.profile.name = "Jane";
    proxy.user.profile.email = "jane@example.com";

    expect(notifyPaths).toEqual([
      ["user", "profile", "name"],
      ["user", "profile", "email"],
    ]);
  });

  it("should notify on symbol property set", () => {
    const sym = Symbol("test");
    const target = { name: "John", [sym]: "old" };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy[sym] = "new";

    expect(target[sym]).toBe("new");
    expect(notifyPaths).toEqual([["symbol:test"]]);
  });

  it("should notify on symbol with numeric parameter set", () => {
    const sym = Symbol(0);
    const target = { name: "John", [sym]: "old" };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy[sym] = "new";

    expect(target[sym]).toBe("new");
    expect(notifyPaths).toEqual([["symbol:0"]]);
  });

  it("should notify on numeric index set (array)", () => {
    const target = { items: [1, 2, 3] };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy.items[1] = 20;

    expect(target.items[1]).toBe(20);
    expect(notifyPaths).toEqual([["items", "1"]]);
  });

  it("should notify on nested array element set", () => {
    const target = { data: [{ name: "item1" }, { name: "item2" }] };
    const notifyPaths: Array<PathKey[]> = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    proxy.data[0].name = "newItem";

    expect(target.data[0].name).toBe("newItem");
    expect(notifyPaths).toEqual([["data", "0", "name"]]);
  });

  it("should notify on delete property", () => {
    const target: { name?: string } = { name: "John" };
    const notifyPaths: PathKey[][] = [];
    const notify = (paths: PathKey[], operation: string) => {
      if (operation === "delete") {
        notifyPaths.push(paths);
      }
    };

    const proxy = createProxy(target, { notifyChange: notify });

    delete proxy.name;

    expect("name" in proxy).toBe(false);
    expect(notifyPaths).toEqual([["name"]]);
  });

  it("should maintain separate notify contexts for multiple proxies", () => {
    const target1 = { value: 1 };
    const target2 = { value: 2 };
    const notifyPaths1: Array<PathKey[]> = [];
    const notifyPaths2: Array<PathKey[]> = [];

    const notify1 = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths1.push(paths);
      }
    };
    const notify2 = (paths: PathKey[], operation: string) => {
      if (operation === "set") {
        notifyPaths2.push(paths);
      }
    };

    const proxy1 = createProxy(target1, { notifyChange: notify1 });
    const proxy2 = createProxy(target2, { notifyChange: notify2 });

    proxy1.value = 10;
    proxy2.value = 20;

    expect(notifyPaths1).toEqual([["value"]]);
    expect(notifyPaths2).toEqual([["value"]]);
  });
});
