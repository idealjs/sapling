export type ComponentType<P = Record<string, unknown | undefined>> = new (
  props: P,
) => {
  render(): SaplingNode;
};

export type SaplingNode = Element | Primitive | SaplingNode[] | null;

export type Primitive = string | number | boolean | bigint;
