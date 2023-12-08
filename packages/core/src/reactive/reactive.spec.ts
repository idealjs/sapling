import { afterEach, describe, expect, it, vi } from "vitest";

import * as State from "../reactive/state";
import { createState, derive, effect } from ".";

const spy1 = vi.spyOn(State, "createObservable");
const spy2 = vi.spyOn(State, "createSubscriber");

afterEach(() => {
  spy1.mockClear();
  spy2.mockClear();
});

describe("unit test", () => {
  it("derive state val", () => {
    const state = createState(1);
    const derived = derive(() => {
      const value = state.val + 1;
      return value;
    });

    expect(derived.val).toBe(2);

    state.val++;

    expect(derived.val).toBe(3);
    expect(State.createObservable).toBeCalledTimes(1);
    expect(State.createSubscriber).toBeCalledTimes(1);
  });

  it("derive state object", () => {
    const state = createState<{ count: number }>();
    const derived = derive(() => {
      const value = (state.val?.count ?? 0) + 1;
      return value;
    });

    expect(derived.val).toBe(1);

    state.val = { count: 1 };
    state.val.count++;

    expect(state.val?.count).toBe(2);
    expect(derived.val).toBe(3);
    expect(State.createObservable).toBeCalledTimes(1);
    expect(State.createSubscriber).toBeCalledTimes(1);
  });

  it("derive state object with initial value", () => {
    const state = createState<{ count: number }>({ count: 0 });
    const derived = derive(() => {
      const value = state.val.count + 1;
      return value;
    });

    expect(derived.val).toBe(1);

    state.val.count++;

    expect(state.val.count).toBe(1);
    expect(derived.val).toBe(2);
    expect(State.createObservable).toBeCalledTimes(1);
    expect(State.createSubscriber).toBeCalledTimes(1);
  });

  it("derive stateView", () => {
    const state = createState(1);
    const derived = derive(() => {
      return state.val + 1;
    });

    const test = derive(() => {
      return derived.val * derived.val;
    });

    expect(test.val).toBe(4);

    state.val++;

    expect(test.val).toBe(9);
  });
  it("state in derive", () => {
    vi.useFakeTimers();
    const stubFn = vi.fn();
    const interval = createState(1000);
    const dispose = effect(() => {
      const handler = setInterval(stubFn, interval.val);
      return () => {
        clearInterval(handler);
      };
    });
    vi.advanceTimersToNextTimer();
    expect(stubFn).toBeCalledTimes(1);

    // Note that a effect clear is done here
    dispose.val();

    interval.val = 2000;
    vi.advanceTimersByTime(2000);

    // effect won't be reexecute due to effect clear
    expect(stubFn).toBeCalledTimes(1);
  });
  it("effect with two state", () => {
    const count = createState(0);
    const hidden = createState(false);
    const mockFn = vi.fn((v, v2) => {});
    effect(() => {
      mockFn(count.val, hidden.val);
    });
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFn).toBeCalledTimes(3);
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFn).toBeCalledTimes(5);
  });
  it("effect with condition", () => {
    const count = createState(0);
    const hidden = createState(false);
    const mockFn1 = vi.fn();
    const mockFn2 = vi.fn();
    effect(() => {
      mockFn2();
      if (!hidden.val) {
        mockFn1(count.val);
      }
    });
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFn1).toBeCalledTimes(2);
    expect(mockFn2).toBeCalledTimes(3);
    count.val++;
    hidden.val = !hidden.val;
    expect(mockFn1).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(4);
  });
  it("effect with complex state", () => {
    const state = createState({ value: { data: { count: 1 }, count: 1 } });
    const mockFn = vi.fn((v) => {});
    const mockFn2 = vi.fn((v) => {});

    effect(() => {
      mockFn(state.val.value.count);
    });
    effect(() => {
      mockFn2(state.val.value.data.count);
    });
    expect(mockFn).toBeCalledTimes(1);
    expect(mockFn2).toBeCalledTimes(1);

    state.val.value.count++;
    expect(mockFn).toBeCalledTimes(2);
    expect(mockFn2).toBeCalledTimes(2);
    expect(state.val).toMatchInlineSnapshot(`
      bound Object {
        "value": bound Object {
          "count": 2,
          "data": bound Object {
            "count": 1,
          },
        },
      }
    `);
    state.val.value.data.count++;
    expect(mockFn).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(3);
    expect(state.val).toMatchInlineSnapshot(`
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
