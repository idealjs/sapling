import { describe, expect, it } from "vitest";
import { makeArrayState } from "../bench.utils";

describe("calc 500000500", () => {
  it("test raw calc", () => {
    let count = 0;
    const state = makeArrayState();
    const calc = (v: number) => {
      count += v;
    };
    for (let u = 0; u < 1000; u++) {
      for (let i = 0; i < 1000; i++) {
        if (i <= u) {
          calc(state.items[u].meta.levelOne.levelTwo.value + 1);
        } else {
          calc(state.items[u].meta.levelOne.levelTwo.value);
        }
      }
    }
    expect(count).toBe(500000500);
    console.table({
      "calc-500000500": {
        count,
      },
    });
  });
});
