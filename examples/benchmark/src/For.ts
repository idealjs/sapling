import type { ComponentType } from "./createComponent";

class ForImpl<T> {
  constructor(props: {
    each: T[];
    children: (item: T[]) => unknown;
    fallback: () => unknown;
  }) {}
}

const For = ForImpl as ComponentType;

export default For;
