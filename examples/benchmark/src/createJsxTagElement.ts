import memo from "./memo";
import type { SaplingNode } from "./types";

const createJsxTagElement = (fn: () => SaplingNode) => {
  return memo(
    () => {
      return fn();
    },
    () => [],
  );
};

export default createJsxTagElement;
