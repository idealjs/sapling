import * as CSS from "csstype";

import { derive, State } from "./reactive";
import { OrFunction, Primitive, Tags } from "./type";

export type ValidChildElementValue = Primitive | Node | null | undefined;

export type ChildElement = ValidChildElementValue | ChildElement[];

const styleToString = (style: CSS.Properties) => {
  return Object.entries(style).reduce(
    (acc, key) =>
      acc +
      key[0]
        .split(/(?=[A-Z])/)
        .join("-")
        .toLowerCase() +
      ":" +
      key[1] +
      ";",
    "",
  );
};

const mergeStyle = <P extends Record<string, unknown>>(
  props: P,
  style?: CSS.Properties | (() => CSS.Properties),
): P & {
  style?: string | (() => string);
} => {
  if (typeof style === "function") {
    return {
      ...props,
      style: () => {
        return styleToString(style());
      },
    };
  }

  if (style == null) {
    return props;
  }

  return {
    ...props,
    style: styleToString(style),
  };
};

export type HyperOption<K extends keyof Tags> = Omit<
  OrFunction<Partial<Tags[K]>>,
  "children" | "style"
> & {
  children?: ChildElement[];
  style?: CSS.Properties;
};

const hyperNS =
  (ns?: string) =>
  <K extends keyof Tags>(tagName: K, options?: HyperOption<K>) => {
    const { children, ...props } = mergeStyle(
      options ?? ({} as HyperOption<K>),
      options?.style,
    );
    const element =
      ns == null
        ? document.createElement(tagName)
        : document.createElementNS(ns, tagName);
    for (const [key, value] of Object.entries(props ?? {})) {
      if (typeof value === "function" && key.startsWith("on")) {
        element.addEventListener(key.replace("on", ""), value);
        continue;
      }

      if (value instanceof State) {
        derive(() => {
          element.setAttribute(key, value.val);
        });
        continue;
      }

      if (typeof value === "function" && !key.startsWith("on")) {
        derive(() => {
          element.setAttribute(key, value());
        });
        continue;
      }
      if (typeof value === "string") {
        element.setAttribute(key, value);
        continue;
      }
    }
    return add(element, ...(children ?? []));
  };

export type HyperObj = {
  [K in keyof Tags]: (options?: HyperOption<K>) => Tags[K];
};

export interface Hyper extends HyperObj {
  <K extends keyof Tags>(tagName: K, options?: HyperOption<K>): Tags[K];
}

export const hyper = new Proxy(hyperNS(), {
  get: (func, args: keyof Tags) => {
    return <K extends keyof Tags>(options?: HyperOption<K>) =>
      func(args, options);
  },
}) as unknown as Hyper;

export const add = (
  element: Node,
  ...children: (ChildElement | (() => ChildElement))[]
): Node => {
  children.forEach((child) => {
    if (Array.isArray(child)) {
      return add(element, ...child);
    }
    if (typeof child === "function") {
      return derive(() => {
        const el = child();
        return add(element, el);
      }).val;
    }
    if (child instanceof Node) {
      return element.appendChild(child);
    }
    if (child != null) {
      return element.appendChild(new Text(child.toString()));
    }
  });
  return element;
};
