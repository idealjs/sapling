import { describe, expect, it } from "vitest";

import moduleA from ".";

describe("parseModuleFromContent", () => {
  it("export default", async () => {
    expect(moduleA).toBe("moduleA");
  });
});
