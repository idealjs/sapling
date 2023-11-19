import { describe, expect, it, vi } from "vitest";

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
  it("with htmlFor", () => {
    const el = hyper("label", {
      htmlFor: "hello",
    });
    expect(el).toMatchInlineSnapshot(`
      <label
        for="hello"
      />
    `);
  });
  it("with eventHandler", () => {
    const mockFn = vi.fn();
    const el = hyper("button", {
      onClick: mockFn,
    });
    expect(el).toMatchInlineSnapshot("<button />");
    el.click();
    expect(mockFn).toBeCalledTimes(1);
  });
  it("with style object", () => {
    const el = hyper("button", {
      style: { color: "blue" },
    });
    expect(el).toMatchInlineSnapshot(`
      <button
        style="color:blue;"
      />
    `);
  });
  it("with style function", () => {
    const el = hyper("button", {
      style: () => ({ color: "blue" }),
    });
    expect(el).toMatchInlineSnapshot(`
      <button
        style="color:blue;"
      />
    `);
  });
  it("with tabIndex string", () => {
    const el = hyper("label", {
      tabIndex: "0",
    });
    expect(el).toMatchInlineSnapshot(`
      <label
        tabindex="0"
      />
    `);
  });
  it("with tabIndex number", () => {
    const el = hyper("label", {
      tabIndex: 0,
    });
    expect(el).toMatchInlineSnapshot(`
      <label
        tabindex="0"
      />
    `);
  });
});
