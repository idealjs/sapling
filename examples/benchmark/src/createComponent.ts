export type ComponentType<P = Record<string, unknown | undefined>> = new (
  props: P,
) => unknown;
type PropsOf<C> = C extends ComponentType<infer P> ? P : unknown;

export function createComponent<
  C extends ComponentType<P>,
  P = Record<string, unknown | undefined>,
>(comp: C, props: P) {
  return;
}

export default createComponent;
