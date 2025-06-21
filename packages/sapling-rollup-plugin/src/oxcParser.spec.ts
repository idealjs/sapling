import { readFileSync } from "fs";
import oxc from "oxc-parser";
import path from "path";
import { describe, expect, it } from "vitest";

describe("test", () => {
  it("convert component", async (t) => {
    const source = readFileSync(
      path.resolve(__dirname, "./fixtures/Test.tsx"),
      "utf-8",
    );
    const result = oxc.parseSync("Test.tsx", source, {
      sourceType: "module",
    });

    expect(JSON.stringify(result.program, null, 2)).toMatchFileSnapshot(
      path.resolve(__dirname, "./fixtures/Test.oxc.ast.json"),
    );
  });
});
