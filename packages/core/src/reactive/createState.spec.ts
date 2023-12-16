import { describe, expect, it, vi } from "vitest";

import { createState, derive, effect, subscribe } from ".";

describe("derive with createState", () => {
  it("derive val", () => {
    const stateA = createState(1);
    const derivedA = derive(() => {
      const value = stateA.val + 1;
      return value;
    });

    expect(derivedA.val).toBe(2);

    stateA.val++;

    expect(derivedA.val).toBe(3);
  });

  it("derive object", () => {
    const stateA = createState<{ count: number }>();
    const derivedA = derive(() => {
      const value = (stateA.val?.count ?? 0) + 1;
      return value;
    });

    expect(derivedA.val).toBe(1);

    stateA.val = { count: 1 };
    stateA.val.count++;

    expect(stateA.val?.count).toBe(2);
    expect(derivedA.val).toBe(3);
  });

  it("derive object with initial value", () => {
    const stateA = createState<{ count: number }>({ count: 0 });
    const derivedA = derive(() => {
      const value = stateA.val.count + 1;
      return value;
    });

    expect(derivedA.val).toBe(1);

    stateA.val.count++;

    expect(stateA.val.count).toBe(1);
    expect(derivedA.val).toBe(2);
  });

  it("derive derived", () => {
    const stateA = createState(1);
    const derivedA = derive(() => {
      return stateA.val + 1;
    });

    const derivedB = derive(() => {
      return derivedA.val * derivedA.val;
    });

    expect(derivedB.val).toBe(4);

    stateA.val++;

    expect(derivedB.val).toBe(9);
  });
});

describe("effect with createState", () => {
  it("effect with timer", () => {
    vi.useFakeTimers();
    const stubFn = vi.fn();
    const interval = createState(1000);
    const subscriber = effect(() => {
      const handler = setInterval(stubFn, interval.val);
      return () => {
        clearInterval(handler);
      };
    });
    vi.advanceTimersToNextTimer();
    expect(stubFn).toBeCalledTimes(1);

    // Note that a effect clear is done here
    subscriber.val?.();

    interval.val = 2000;
    vi.advanceTimersByTime(2000);

    // effect won't be reexecute due to effect clear
    expect(stubFn).toBeCalledTimes(1);
  });

  it("effect with two reactive object", () => {
    const count = createState(0);
    const hidden = createState(false);
    const mockFnA = vi.fn((v, v2) => {});
    effect(() => {
      mockFnA(count.val, hidden.val);
    });
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFnA).toBeCalledTimes(3);
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFnA).toBeCalledTimes(5);
  });

  it("effect with condition", () => {
    const count = createState(0);
    const hidden = createState(false);
    const mockFnA = vi.fn();
    const mockFnB = vi.fn();
    effect(() => {
      mockFnB();
      if (!hidden.val) {
        mockFnA(count.val);
      }
    });
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFnA).toBeCalledTimes(2);
    expect(mockFnB).toBeCalledTimes(3);
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFnA).toBeCalledTimes(3);
    expect(mockFnB).toBeCalledTimes(4);
  });

  it("effect with complex object", () => {
    const stateA = createState({ value: { data: { count: 1 }, count: 1 } });
    const mockFnA = vi.fn((v) => {});
    const mockFnB = vi.fn((v) => {});

    effect(() => {
      mockFnA(stateA.val.value.count);
    });
    effect(() => {
      mockFnB(stateA.val.value.data.count);
    });
    expect(mockFnA).toBeCalledTimes(1);
    expect(mockFnB).toBeCalledTimes(1);

    stateA.val.value.count++;
    expect(mockFnA).toBeCalledTimes(2);
    expect(mockFnB).toBeCalledTimes(2);
    expect(stateA.val).toMatchInlineSnapshot(`
      bound Object {
        "value": bound Object {
          "count": 2,
          "data": bound Object {
            "count": 1,
          },
        },
      }
    `);
    stateA.val.value.data.count++;
    expect(mockFnA).toBeCalledTimes(2);
    expect(mockFnB).toBeCalledTimes(3);
    expect(stateA.val).toMatchInlineSnapshot(`
      bound Object {
        "value": bound Object {
          "count": 2,
          "data": bound Object {
            "count": 2,
          },
        },
      }
    `);
  });
});
