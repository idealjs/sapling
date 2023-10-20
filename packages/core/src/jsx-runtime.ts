import * as CSS from "csstype";

import { ComponentChild, ComponentChildren } from "./createElement";
import { ChildElement } from "./hyper";
import { Dispose, StateView } from "./reactive";
import type { Tags } from "./type";

type HTMLAttributes<T> = Partial<Omit<T, "style" | "children">> & {
  style?: CSS.Properties;
  ref?: { val: T };
  children?: ComponentChildren | (() => ChildElement);
};

type SVGAttributes<T> = Partial<Omit<T, "style" | "children">> & {
  style?: CSS.Properties;
  ref?: { val: T };
};

type FragmentAttributes<T> = Partial<Omit<T, "children">> & {
  children: ComponentChild | ComponentChild[] | (() => ComponentChild);
};

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace JSX {
  export interface ElementAttributesProperty {
    props: object;
  }
  export interface ElementChildrenAttribute {
    children: object;
  }

  export type Element = ComponentChild;

  export interface IntrinsicAttributes {}

  type InnerElement = {
    [K in keyof Tags]: Tags[K] extends HTMLElement
      ? HTMLAttributes<Tags[K]>
      : Tags[K] extends SVGElement
      ? SVGAttributes<Tags[K]>
      : FragmentAttributes<Tags[K]> & {
          dispose?: StateView<Dispose>[];
        };
  };

  export interface IntrinsicElements extends InnerElement {}
}

export { default as jsx } from "./createElement";
