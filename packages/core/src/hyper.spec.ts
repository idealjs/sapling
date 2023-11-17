import { describe, expect, it } from "vitest";

import { hyper } from "./hyper";

describe("unit test", () => {
  it("with className", () => {
    const el = hyper("div", {
      className: "hello",
    });
    expect(el).toMatchInlineSnapshot(`
      <div
        class="hello"
      />
    `);
  });
});
