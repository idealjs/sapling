import type { JSXNode, Key, TagNameMap, TagOption } from ".";

export * from ".";

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace JSX {
  export interface ElementAttributesProperty {
    props: object;
  }
  export interface ElementChildrenAttribute {
    children: object;
  }

  export type Element = JSXNode;

  export interface IntrinsicAttributes {
    key?: Key;
  }

  type InnerElement = {
    [K in keyof TagNameMap]: TagOption<K> & {
      key?: Key;
    };
  };

  export interface IntrinsicElements extends InnerElement {}
}
