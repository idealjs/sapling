import { format } from "pretty-format";
import { expect } from "vitest";

const flatTreeNode = (node) => {
  const { el, children, staticContainer } = node;
  return {
    el,
    children: new Set([...children].map((child) => flatTreeNode(child))),
    staticContainer,
  };
};

expect.addSnapshotSerializer({
  serialize(val, config, indentation, depth, refs, printer) {
    return format(flatTreeNode(val), {
      compareKeys(a, b) {
        if (a === "staticContainer") {
          return -1;
        }
        if (a === "el") {
          return -1;
        }
        return a > b ? -1 : 1;
      },
    });
  },
  test(val) {
    return val?.staticContainer != null;
  },
});
