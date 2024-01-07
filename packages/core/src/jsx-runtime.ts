/* eslint-disable @typescript-eslint/no-namespace */
/* eslint-disable @typescript-eslint/no-explicit-any */

import type { JSXElementType, SaplingElement } from "./createElement";
import type { InnerElement, Key, TagOption } from "./type";

export namespace JSX {
  export type ElementType = string | JSXElementType<any>;
  export interface ElementAttributesProperty {
    props: object;
  }
  export interface ElementChildrenAttribute {
    children: object;
  }

  export interface Element extends SaplingElement {}

  export interface IntrinsicAttributes {
    key?: Key;
  }

  export type IntrinsicElements = {
    [K in keyof InnerElement]: TagOption<K>;
  };
}
