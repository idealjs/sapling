import { beforeEach, describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";
import type { PathKey } from "./types";

describe("createProxy - tracing path with get operations", () => {
  let trackingPaths: Array<{ paths: PathKey[]; operation: string }>;

  beforeEach(() => {
    trackingPaths = [];
  });

  const notify = (paths: PathKey[], operation: string) => {
    trackingPaths.push({ paths, operation });
  };

  it("should track simple property access with get operation", () => {
    const target = { name: "John", age: 30 };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.name).toBe("John");

    expect(trackingPaths).toEqual([{ paths: ["name"], operation: "get" }]);
  });

  it("should track multiple property accesses with get operation", () => {
    const target = { name: "John", age: 30, email: "john@example.com" };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.name).toBe("John");
    expect(proxy.age).toBe(30);
    expect(proxy.email).toBe("john@example.com");

    expect(trackingPaths).toEqual([
      { paths: ["name"], operation: "get" },
      { paths: ["age"], operation: "get" },
      { paths: ["email"], operation: "get" },
    ]);
  });

  it("should track nested object property access", () => {
    const target = {
      user: {
        profile: {
          name: "John",
        },
      },
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.user.profile.name).toBe("John");

    expect(trackingPaths).toEqual([
      { paths: ["user"], operation: "get" },
      { paths: ["user", "profile"], operation: "get" },
      { paths: ["user", "profile", "name"], operation: "get" },
    ]);
  });

  it("should track deeply nested property access", () => {
    const target = {
      level1: {
        level2: {
          level3: {
            level4: {
              value: "deep",
            },
          },
        },
      },
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.level1.level2.level3.level4.value).toBe("deep");

    expect(trackingPaths).toEqual([
      { paths: ["level1"], operation: "get" },
      { paths: ["level1", "level2"], operation: "get" },
      { paths: ["level1", "level2", "level3"], operation: "get" },
      { paths: ["level1", "level2", "level3", "level4"], operation: "get" },
      {
        paths: ["level1", "level2", "level3", "level4", "value"],
        operation: "get",
      },
    ]);
  });

  it("should track non-trackable properties like symbols", () => {
    const sym = Symbol("test");
    const target = {
      name: "John",
      [sym]: "symbol value",
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.name).toBe("John");
    expect(proxy[sym]).toBe("symbol value");

    expect(trackingPaths).toEqual([
      { paths: ["name"], operation: "get" },
      { paths: ["symbol:test"], operation: "get" },
    ]);
  });

  it("should track numeric properties", () => {
    const target = {
      0: "first",
      1: "second",
      2: "third",
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy[0]).toBe("first");
    expect(proxy[1]).toBe("second");
    expect(proxy[2]).toBe("third");

    expect(trackingPaths).toEqual([
      { paths: ["0"], operation: "get" },
      { paths: ["1"], operation: "get" },
      { paths: ["2"], operation: "get" },
    ]);
  });

  it("should track symbol with numeric parameter", () => {
    const sym = Symbol(0);
    const target = {
      name: "John",
      [sym]: "symbol with number",
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.name).toBe("John");
    expect(proxy[sym]).toBe("symbol with number");

    expect(trackingPaths).toEqual([
      { paths: ["name"], operation: "get" },
      { paths: ["symbol:0"], operation: "get" },
    ]);
  });

  it("should track array access by index", () => {
    const target = { items: [1, 2, 3] };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.items[0]).toBe(1);
    expect(proxy.items[1]).toBe(2);
    expect(proxy.items[2]).toBe(3);

    expect(trackingPaths).toEqual([
      { paths: ["items"], operation: "get" },
      { paths: ["items", "0"], operation: "get" },
      { paths: ["items"], operation: "get" },
      { paths: ["items", "1"], operation: "get" },
      { paths: ["items"], operation: "get" },
      { paths: ["items", "2"], operation: "get" },
    ]);
  });

  it("should track property access on nested arrays", () => {
    const target = {
      data: [{ name: "item1" }, { name: "item2" }],
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.data[0].name).toBe("item1");
    expect(proxy.data[1].name).toBe("item2");

    expect(trackingPaths).toEqual([
      { paths: ["data"], operation: "get" },
      { paths: ["data", "0"], operation: "get" },
      { paths: ["data", "0", "name"], operation: "get" },
      { paths: ["data"], operation: "get" },
      { paths: ["data", "1"], operation: "get" },
      { paths: ["data", "1", "name"], operation: "get" },
    ]);
  });

  it("should maintain separate tracking contexts", () => {
    const target1 = { value: 1 };
    const target2 = { value: 2 };
    const paths1: Array<{ paths: PathKey[]; operation: string }> = [];
    const paths2: Array<{ paths: PathKey[]; operation: string }> = [];

    const notify1 = (paths: PathKey[], operation: string) => {
      paths1.push({ paths, operation });
    };
    const notify2 = (paths: PathKey[], operation: string) => {
      paths2.push({ paths, operation });
    };

    const proxy1 = createProxy(target1, { notifyGet: notify1 });
    const proxy2 = createProxy(target2, { notifyGet: notify2 });

    expect(proxy1.value).toBe(1);
    expect(proxy2.value).toBe(2);

    expect(paths1).toEqual([{ paths: ["value"], operation: "get" }]);
    expect(paths2).toEqual([{ paths: ["value"], operation: "get" }]);
  });

  it("should accumulate tracking paths across multiple accesses", () => {
    const target = {
      user: { name: "John", age: 30 },
      settings: { theme: "dark" },
    };
    const proxy = createProxy(target, { notifyGet: notify });

    expect(proxy.user.name).toBe("John");
    expect(proxy.user.age).toBe(30);
    expect(proxy.settings.theme).toBe("dark");

    expect(trackingPaths).toEqual([
      { paths: ["user"], operation: "get" },
      { paths: ["user", "name"], operation: "get" },
      { paths: ["user"], operation: "get" },
      { paths: ["user", "age"], operation: "get" },
      { paths: ["settings"], operation: "get" },
      { paths: ["settings", "theme"], operation: "get" },
    ]);
  });
});
