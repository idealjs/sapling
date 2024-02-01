import { diff } from "@egjs/list-differ";
import { describe, expect, it } from "vitest";

describe("diff test", () => {
  it("should check diff result", () => {
    const result = diff([1, 2, 3, 4, 5, 6, 7], [4, 3, 8, 2, 1, 7], (e) => e);
    expect(result).toMatchInlineSnapshot(`
      Result {
        "added": [
          2,
        ],
        "changed": [
          [
            3,
            0,
          ],
          [
            2,
            1,
          ],
          [
            1,
            3,
          ],
          [
            0,
            4,
          ],
          [
            6,
            5,
          ],
        ],
        "changedBeforeAdded": [
          [
            3,
            0,
          ],
          [
            2,
            1,
          ],
          [
            1,
            2,
          ],
          [
            0,
            3,
          ],
          [
            4,
            4,
          ],
        ],
        "fixed": [
          false,
          false,
          false,
          false,
          false,
        ],
        "list": [
          4,
          3,
          8,
          2,
          1,
          7,
        ],
        "maintained": [
          [
            3,
            0,
          ],
          [
            2,
            1,
          ],
          [
            1,
            3,
          ],
          [
            0,
            4,
          ],
          [
            6,
            5,
          ],
        ],
        "prevList": [
          1,
          2,
          3,
          4,
          5,
          6,
          7,
        ],
        "removed": [
          5,
          4,
        ],
      }
    `);
  });
});
