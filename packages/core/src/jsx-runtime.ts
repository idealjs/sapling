/* eslint-disable @typescript-eslint/no-namespace */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type {
  InnerElement,
  JSXElementType,
  SaplingElement,
  TagOption,
} from "./type";

export namespace JSX {
  export type ElementType = string | JSXElementType<any>;
  export interface ElementAttributesProperty {
    props: object;
  }
  export interface ElementChildrenAttribute {
    children: object;
  }

  export type Element = SaplingElement;
  export interface IntrinsicAttributes {
    key?: number | string | symbol;
  }

  export type IntrinsicElements = {
    [K in keyof InnerElement]: TagOption<K>;
  };
}
