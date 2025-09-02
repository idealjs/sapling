import memo from "./memo";
import type { SaplingNode } from "./types";

const createElement = (tag: string): SaplingNode => {
  return memo(
    () => {
      return document.createElement(tag);
    },
    () => {},
  );
};

export default createElement;
