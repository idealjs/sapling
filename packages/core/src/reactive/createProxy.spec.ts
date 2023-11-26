import { describe, expect, it, vi } from "vitest";

import { createProxyUtil } from "./createProxy";

describe("createProxy", () => {
  it("call update when change value", () => {
    const mockFn = vi.fn();
    const proxy = createProxyUtil(mockFn)({ val: 0 });
    proxy.val++;
    expect(proxy.val).toBe(1);
    expect(mockFn).toBeCalledTimes(1);
  });

  it("call update when set value", () => {
    const mockFn = vi.fn();
    const proxy = createProxyUtil(mockFn)<{ val: { count: number } }>();
    proxy.val = { count: 0 };
    proxy.val.count++;
    expect(proxy.val.count).toBe(1);
    expect(mockFn).toBeCalledTimes(2);
  });
});
