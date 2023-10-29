import fs from "fs";
import { writeHeapSnapshot } from "v8";
import { describe, expect, it, vi } from "vitest";

import { createState, derive, effect } from "./reactive";
import { getNodes, getState, readSnapshotFile } from "./utils/v8";

describe("unit test", () => {
  it("derive state val", () => {
    const state = createState(1);
    const derived = derive(() => {
      return state.val + 1;
    });

    expect(derived.val).toBe(2);
    expect(derived.oldVal).toBe(undefined);

    state.val++;

    expect(derived.val).toBe(3);
    expect(derived.oldVal).toBe(2);
  });
  it("derive state oldVal", () => {
    const state = createState(1);
    const derived = derive(() => {
      return (state.oldVal ?? 0) + 1;
    });

    expect(derived.val).toBe(1);
    expect(derived.oldVal).toBe(undefined);

    state.val++;

    expect(derived.val).toBe(2);
    expect(derived.oldVal).toBe(1);
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
    expect(test.oldVal).toBe(undefined);

    state.val++;

    expect(test.val).toBe(9);
    expect(test.oldVal).toBe(4);
  });
  it.only("state in derive", () => {
    vi.useFakeTimers();
    const stubFn = vi.fn();
    const interval = createState(1000);
    const dispose = effect(() => {
      const handler = setInterval(stubFn, interval.val);
      return () => {
        clearInterval(handler);
      };
    });
    const snapshot1 = writeHeapSnapshot();
    vi.advanceTimersToNextTimer();
    expect(stubFn).toBeCalledTimes(1);

    // Note that a timer clear is done here
    dispose.val();

    interval.val = 2000;
    const snapshot2 = writeHeapSnapshot();
    vi.advanceTimersByTime(2000);
    expect(stubFn).toBeCalledTimes(2);
    expect(
      getState(getNodes(readSnapshotFile(snapshot1))).length,
    ).toMatchInlineSnapshot("2");
    expect(
      getState(getNodes(readSnapshotFile(snapshot2))).length,
    ).toMatchInlineSnapshot("2");
    fs.unlinkSync(snapshot1);
    fs.unlinkSync(snapshot2);
  });
});
