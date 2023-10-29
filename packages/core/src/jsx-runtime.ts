import type { JSXNode, TagOption } from "./createElement";
import type { Key, TagNameMap } from "./type";

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

export { default as jsx, default as jsxDEV } from "./createElement";
