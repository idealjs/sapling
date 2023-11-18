import type { JSXNode } from "./createElement";
import type { InnerElement, Key, TagOption } from "./type";

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

  export type IntrinsicElements = {
    [K in keyof InnerElement]: TagOption<K>;
  };
}
