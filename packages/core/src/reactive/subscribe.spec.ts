import { describe, expect, it, vi } from "vitest";

import { createProxy, subscribe } from ".";

describe("subscribe with createProxy", () => {
  it("get null value", () => {
    const mockFnA = vi.fn();
    const stateA = createProxy<{ val: { count: number | null } }>({
      val: { count: null },
    });
    subscribe(stateA.val, mockFnA);
    expect(stateA.val.count).toBe(null);
    stateA.val.count = 0;
    expect(stateA.val.count).toBe(0);
    stateA.val.count++;
    expect(stateA.val.count).toBe(1);
    expect(mockFnA).toBeCalledTimes(2);
  });

  it("set null value", () => {
    const mockFnA = vi.fn();
    const stateA = createProxy<{ val: { count: number | null } }>({
      val: { count: 0 },
    });
    subscribe(stateA.val, mockFnA);
    expect(stateA.val.count).toBe(0);
    stateA.val.count = null;
    expect(stateA.val.count).toBe(null);
    expect(mockFnA).toBeCalledTimes(1);
  });

  it("call update when change value", () => {
    const mockFnA = vi.fn();
    const stateA = createProxy({ val: 0 });
    subscribe(stateA, mockFnA);
    stateA.val++;
    expect(stateA.val).toBe(1);
    expect(mockFnA).toBeCalledTimes(1);
  });

  it("call update when change value with nested object", () => {
    const mockFnA = vi.fn();
    const stateA = createProxy({ val: { count: 0 } });
    subscribe(stateA.val, mockFnA);
    stateA.val.count++;
    expect(stateA.val.count).toBe(1);
    expect(mockFnA).toBeCalledTimes(1);
  });

  it("call update when set value", () => {
    const mockFnA = vi.fn();
    const stateA = createProxy<{ val: { count: number } }>();
    subscribe(stateA, mockFnA);
    stateA.val = { count: 0 };
    subscribe(stateA.val, mockFnA);
    stateA.val.count++;
    expect(stateA.val.count).toBe(1);
    expect(mockFnA).toBeCalledTimes(2);
  });

  it("assign object to multiple reactive object", () => {
    const mockFnA = vi.fn();
    const mockFnB = vi.fn();
    const val = { count: 0 };
    const stateA = createProxy<{ val: { count: number } }>();
    const stateB = createProxy<{ val: { count: number } }>();
    subscribe(stateA, mockFnA);
    subscribe(stateB, mockFnB);
    stateA.val = val;
    stateB.val = val;
    subscribe(stateA.val, mockFnA);
    subscribe(stateB.val, mockFnB);
    stateA.val.count++;
    stateB.val.count++;
    expect(stateA.val === stateB.val).toBe(true);
    expect(stateA.val.count).toBe(2);
    expect(stateB.val.count).toBe(2);
    expect(mockFnA).toBeCalledTimes(3);
    expect(mockFnB).toBeCalledTimes(3);
  });

  it("assign value should equal", () => {
    const mockFnA = vi.fn();
    const mockFnB = vi.fn();
    const val = { count: 0 };
    const stateA = createProxy<{ val: { count: number } }>();
    const stateB = createProxy<{ val: { count: number } }>();
    subscribe(stateA, mockFnA);
    subscribe(stateB, mockFnB);
    stateA.val = val;
    stateB.val = stateA.val;
    subscribe(stateA.val, mockFnA);
    subscribe(stateB.val, mockFnB);
    expect(mockFnA).toBeCalledTimes(1);
    expect(mockFnB).toBeCalledTimes(1);
    expect(stateB.val === stateA.val).toBe(true);
    stateA.val.count++;
    stateB.val.count++;
    expect(mockFnA).toBeCalledTimes(3);
    expect(mockFnB).toBeCalledTimes(3);
  });
});
