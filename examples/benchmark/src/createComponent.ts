type PropsOf<C> = C extends new (...args: infer A) => unknown
  ? A extends [infer P, infer F, infer Chi]
    ? Chi extends (item: infer I) => unknown
      ? P extends object
        ? // constructor with (props, fallback, children) -> keep props and infer children item type
          P & { fallback?: F; children?: (item: I) => unknown }
        : P
      : P
    : A extends [infer P]
      ? P
      : unknown
  : unknown;

/* overloads for better editor UX */
export function createComponent<C>(comp: C, props: PropsOf<C>): unknown;
export function createComponent(comp: unknown, props: unknown) {
  return;
}

export default createComponent;
