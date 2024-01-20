import { effect } from "@idealjs/sapling-reactive";
import * as CSS from "csstype";

import { TagNameMap } from "./type";

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

const setAttribute = (element: Element, key: string, value: unknown) => {
  // Convert Style Object
  if (key === "style") {
    const attr = styleToString(value as CSS.Properties);
    element.setAttribute(key, attr);
    return;
  }

  if (typeof value === "number") {
    if (key === "tabIndex") {
      element.setAttribute("tabindex", value.toString());
      return;
    }
  }

  // Set String Attribute
  if (typeof value === "string") {
    if (key === "className") {
      element.setAttribute("class", value);
      return;
    }
    if (key === "htmlFor") {
      element.setAttribute("for", value);
      return;
    }

    element.setAttribute(key, value);
    return;
  }
};

const hyperNS =
  (ns?: string) =>
  <K extends keyof TagNameMap>(
    tagName: K,
    options?: Record<string, unknown>,
  ): TagNameMap[K] => {
    const element = (
      ns == null
        ? document.createElement(tagName)
        : document.createElementNS(ns, tagName)
    ) as TagNameMap[K];

    for (const [key, value] of Object.entries(options ?? {})) {
      // Auto Update Attribute
      if (typeof value === "function" && !key.startsWith("on")) {
        effect(() => {
          let attr = value();
          setAttribute(element, key, attr);
        });
        continue;
      }

      // Add Event Listener
      if (typeof value === "function" && key.startsWith("on")) {
        element.addEventListener(
          key.replace("on", "").toLowerCase(),
          value as EventListenerOrEventListenerObject,
        );
        continue;
      }
      setAttribute(element, key, value);
      continue;
    }
    return element;
  };

export type HyperObj = {
  [K in keyof TagNameMap]: (options?: Record<string, unknown>) => TagNameMap[K];
};

export interface Hyper extends HyperObj {
  <K extends keyof TagNameMap>(
    tagName: K,
    options?: Record<string, unknown>,
  ): TagNameMap[K];
}

export const hyper = new Proxy(hyperNS(), {
  get: (func, args: keyof TagNameMap) => {
    return (options?: Record<string, unknown>) => func(args, options);
  },
});
