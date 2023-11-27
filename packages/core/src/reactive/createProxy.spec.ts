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

  it("call update when change value with nested object", () => {
    const mockFn = vi.fn();
    const proxy = createProxyUtil(mockFn)({ val: { count: 0 } });
    proxy.val.count++;
    expect(proxy.val.count).toBe(1);
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

  it("assign object to multiple proxy", () => {
    const mockFn = vi.fn();
    const mockFn2 = vi.fn();
    const val = { count: 0 };
    const proxy = createProxyUtil(mockFn)<{ val: { count: number } }>();
    const proxy2 = createProxyUtil(mockFn2)<{ val: { count: number } }>();
    proxy.val = val;
    proxy2.val = val;
    proxy.val.count++;
    proxy2.val.count++;
    expect(proxy.val === proxy2.val).toBe(true);
    expect(proxy.val.count).toBe(2);
    expect(proxy2.val.count).toBe(2);
    expect(mockFn).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(3);
  });

  it("assign value should equal", () => {
    const mockFn = vi.fn();
    const mockFn2 = vi.fn();
    const val = { count: 0 };
    const proxy = createProxyUtil(mockFn)<{ val: { count: number } }>();
    const proxy2 = createProxyUtil(mockFn2)<{ val: { count: number } }>();
    proxy.val = val;
    proxy2.val = proxy.val;
    expect(mockFn).toBeCalledTimes(1);
    expect(mockFn2).toBeCalledTimes(1);
    expect(proxy2.val === proxy.val).toBe(true);
    proxy.val.count++;
    proxy2.val.count++;
    expect(mockFn).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(3);
  });
});
