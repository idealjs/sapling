import { describe, expect, it } from "vitest";
import { createProxy } from "./createProxy";

describe("createProxy - optional notify callback", () => {
  it("should allow property sets without notify callback", () => {
    const target = { name: "John" };

    const proxy = createProxy(target);

    proxy.name = "Jane";

    expect(target.name).toBe("Jane");
  });
});
