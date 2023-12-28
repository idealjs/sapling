import { describe, expect, it, vi } from "vitest";

import { createSignal, derive, effect, subscribe } from ".";

describe("derive with createSignal", () => {
  it("derive val", () => {
    const [state, setState] = createSignal(1);
    const derived = derive(() => {
      const value = state() + 1;
      return value;
    });

    expect(derived.val).toBe(2);

    setState((p) => p + 1);

    expect(derived.val).toBe(3);
  });

  it("derive object", () => {
    const [state, setState] = createSignal<{ count: number }>();
    const derived = derive(() => {
      const value = (state()?.count ?? 0) + 1;
      return value;
    });

    expect(derived.val).toBe(1);

    setState({ count: 1 });
    setState((p) => ({
      count: (p?.count ?? 0) + 1,
    }));

    expect(state()?.count).toBe(2);
    expect(derived.val).toBe(3);
  });

  it("derive array with push value", () => {
    const [stateA, setStateA] = createSignal([1, 2, 3]);
    const derivedA = derive(() => {
      return stateA().reduce((p, c) => p + c);
    });
    expect(derivedA.val).toBe(6);
    setStateA((v) => [...v, 4]);
    expect(derivedA.val).toBe(10);
  });

  it("derive array with reduce", () => {
    const [stateA] = createSignal([{ count: 1 }, { count: 2 }, { count: 3 }]);
    const derivedA = derive(() => {
      return stateA().reduce((p, c) => p + c.count, 0);
    });
    expect(derivedA.val).toBe(6);
    stateA()[3] = { count: 4 };
    expect(derivedA.val).toBe(10);
    stateA()[0].count = 6;
    expect(derivedA.val).toBe(15);
  });

  it("derive array with map", () => {
    const [stateA] = createSignal([{ count: 1 }, { count: 2 }, { count: 3 }]);
    const derivedA = derive(() => {
      return stateA().map((p) => p.count);
    });
    expect(derivedA.val).toMatchInlineSnapshot(`
      bound Array [
        1,
        2,
        3,
      ]
    `);
    stateA()[3] = { count: 4 };
    expect(derivedA.val).toMatchInlineSnapshot(`
      bound Array [
        1,
        2,
        3,
        4,
      ]
    `);
    stateA()[0].count = 6;
    expect(derivedA.val).toMatchInlineSnapshot(`
      bound Array [
        6,
        2,
        3,
        4,
      ]
    `);
  });

  it("derive array with foreach", () => {
    const [stateA] = createSignal([{ count: 1 }, { count: 2 }, { count: 3 }]);
    const derivedA = derive(() => {
      const value: number[] = [];
      stateA().forEach((v) => value.push(v.count));
      return value;
    });
    expect(derivedA.val).toMatchInlineSnapshot(`
      bound Array [
        1,
        2,
        3,
      ]
    `);
    stateA()[3] = { count: 4 };
    expect(derivedA.val).toMatchInlineSnapshot(`
      bound Array [
        1,
        2,
        3,
        4,
      ]
    `);
    stateA()[0].count = 6;
    expect(derivedA.val).toMatchInlineSnapshot(`
    bound Array [
      6,
      2,
      3,
      4,
    ]
  `);
  });

  it("derive object with initial value", () => {
    const [state] = createSignal<{ count: number }>({ count: 0 });
    const derived = derive(() => {
      const value = state().count + 1;
      return value;
    });

    expect(derived.val).toBe(1);

    state().count++;

    expect(state().count).toBe(1);
    expect(derived.val).toBe(2);
  });

  it("derive derived", () => {
    const [state, setState] = createSignal(1);
    const derived = derive(() => {
      return state() + 1;
    });

    const test = derive(() => {
      return derived.val * derived.val;
    });

    expect(test.val).toBe(4);

    setState((p) => p + 1);

    expect(test.val).toBe(9);
  });
});

describe("effect with createSignal", () => {
  it("effect with timer", () => {
    vi.useFakeTimers();
    const stubFn = vi.fn();
    const [timeout, setIimeout] = createSignal(1000);
    const subscriber = effect(() => {
      const handler = setInterval(stubFn, timeout());
      return () => {
        clearInterval(handler);
      };
    });
    vi.advanceTimersToNextTimer();
    expect(stubFn).toBeCalledTimes(1);

    // Note that a effect clear is done here
    subscriber.val?.();

    setIimeout(2000);
    vi.advanceTimersByTime(2000);

    // effect won't be reexecute due to effect clear
    expect(stubFn).toBeCalledTimes(1);
  });

  it("effect with two reactive object", () => {
    const [count, setCount] = createSignal(0);
    const [hidden, setHidden] = createSignal(false);
    const mockFn = vi.fn((v, v2) => {});
    effect(() => {
      mockFn(count(), hidden());
    });
    setCount((p) => p + 1);
    setHidden((p) => !p);
    expect(mockFn).toBeCalledTimes(3);
    setCount((p) => p + 1);
    setHidden((p) => !p);
    expect(mockFn).toBeCalledTimes(5);
  });

  it("effect with condition", () => {
    const [count, setCount] = createSignal(0);
    const [hidden, setHidden] = createSignal(false);
    const mockFnA = vi.fn();
    const mockFn2 = vi.fn();
    effect(() => {
      mockFn2();
      if (!hidden()) {
        mockFnA(count());
      }
    });
    setCount((p) => p + 1);
    setHidden((p) => !p);
    expect(mockFnA).toBeCalledTimes(2);
    expect(mockFn2).toBeCalledTimes(3);
    setCount((p) => p + 1);
    setHidden((p) => !p);
    expect(mockFnA).toBeCalledTimes(3);
    expect(mockFn2).toBeCalledTimes(4);
  });

  it("effect with complex object", () => {
    const [state, setState] = createSignal({
      value: { data: { count: 1 }, count: 1 },
    });
    const mockFn = vi.fn((v) => {});
    const mockFn2 = vi.fn((v) => {});

    effect(() => {
      mockFn(state().value.count);
    });
    effect(() => {
      mockFn2(state().value.data.count);
    });
    expect(mockFn).toBeCalledTimes(1);
    expect(mockFn2).toBeCalledTimes(1);

    state().value.count++;
    expect(mockFn).toBeCalledTimes(2);
    expect(mockFn2).toBeCalledTimes(2);
    expect(state()).toMatchInlineSnapshot(`
      bound Object {
        "value": bound Object {
          "count": 2,
          "data": bound Object {
            "count": 1,
          },
        },
      }
    `);
    state().value.data.count++;
    expect(mockFn).toBeCalledTimes(2);
    expect(mockFn2).toBeCalledTimes(3);
    expect(state()).toMatchInlineSnapshot(`
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
