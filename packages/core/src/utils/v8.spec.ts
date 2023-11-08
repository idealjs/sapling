import { describe, expect, it } from "vitest";

import { getNodes, getState, readSnapshotFile } from "./v8";

describe("unit test", () => {
  it("getState", () => {
    const nodes = getState(
      getNodes(readSnapshotFile("src/utils/v8.spec.case")),
    );
    expect(nodes.length).toBe(36);
  });
});
