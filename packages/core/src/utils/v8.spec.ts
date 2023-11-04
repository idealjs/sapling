import { describe, expect, it } from "vitest";

import { getNodes, getState, readSnapshotFile } from "./v8";

describe("unit test", () => {
  it("getState", () => {
    expect(
      getState(getNodes(readSnapshotFile("src/utils/v8.spec.case"))).length,
    ).toBe(36);
  });
});
