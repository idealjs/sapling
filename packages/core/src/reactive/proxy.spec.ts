import { describe, expect, it, vi } from "vitest";

import { proxy, subscribe } from ".";

describe("createProxy", () => {
  it("get null value", () => {
    const mockFn1 = vi.fn();
    const proxy1 = proxy<{ val: { count: number | null } }>({
      val: { count: null },
    });
    subscribe(proxy1.val, mockFn1);
    expect(proxy1.val.count).toBe(null);
    proxy1.val.count = 0;
    expect(proxy1.val.count).toBe(0);
    proxy1.val.count++;
    expect(proxy1.val.count).toBe(1);
    expect(mockFn1).toBeCalledTimes(2);
  });
  it("call update when change value", () => {
    const mockFn1 = vi.fn();
    const proxy1 = proxy({ val: 0 });
    subscribe(proxy1, mockFn1);
    proxy1.val++;
    expect(proxy1.val).toBe(1);
    expect(mockFn1).toBeCalledTimes(1);
  });

  it("call update when change value with nested object", () => {
    const mockFn1 = vi.fn();
    const proxy1 = proxy({ val: { count: 0 } });
    subscribe(proxy1.val, mockFn1);
    proxy1.val.count++;
    expect(proxy1.val.count).toBe(1);
    expect(mockFn1).toBeCalledTimes(1);
  });

  it("call update when set value", () => {
    const mockFn1 = vi.fn();
    const proxy1 = proxy<{ val: { count: number } }>();
    subscribe(proxy1, mockFn1);
    proxy1.val = { count: 0 };
    subscribe(proxy1.val, mockFn1);
    proxy1.val.count++;
    expect(proxy1.val.count).toBe(1);
    expect(mockFn1).toBeCalledTimes(2);
  });

  it("assign object to multiple proxy", () => {
    const mockFn1 = vi.fn();
    const mockFn2 = vi.fn();
    const val = { count: 0 };
    const proxy1 = proxy<{ val: { count: number } }>();
    const proxy2 = proxy<{ val: { count: number } }>();
    subscribe(proxy1, mockFn1);
    subscribe(proxy2, mockFn2);
    proxy1.val = val;
    proxy2.val = val;
    subscribe(proxy1.val, mockFn1);
    subscribe(proxy2.val, mockFn2);
    proxy1.val.count++;
    proxy2.val.count++;
    expect(proxy1.val === proxy2.val).toBe(true);
    expect(proxy1.val.count).toBe(2);
    expect(proxy2.val.count).toBe(2);
    expect(mockFn1).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(3);
  });

  it("assign value should equal", () => {
    const mockFn1 = vi.fn();
    const mockFn2 = vi.fn();
    const val = { count: 0 };
    const proxy1 = proxy<{ val: { count: number } }>();
    const proxy2 = proxy<{ val: { count: number } }>();
    subscribe(proxy1, mockFn1);
    subscribe(proxy2, mockFn2);
    proxy1.val = val;
    proxy2.val = proxy1.val;
    subscribe(proxy1.val, mockFn1);
    subscribe(proxy2.val, mockFn2);
    expect(mockFn1).toBeCalledTimes(1);
    expect(mockFn2).toBeCalledTimes(1);
    expect(proxy2.val === proxy1.val).toBe(true);
    proxy1.val.count++;
    proxy2.val.count++;
    expect(mockFn1).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(3);
  });
});
