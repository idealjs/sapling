import * as CSS from "csstype";

import { effect } from "./reactive";
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
          if (key === "style") {
            attr = styleToString(attr);
          }
          element.setAttribute(key, attr);
        });
        continue;
      }

      // Add Event Listener
      if (typeof value === "function" && key.startsWith("on")) {
        element.addEventListener(
          key.replace("on", ""),
          value as EventListenerOrEventListenerObject,
        );
        continue;
      }

      // Convert Style Object
      if (key === "style") {
        const attr = styleToString(value as CSS.Properties);
        element.setAttribute(key, attr);
        continue;
      }

      // Set String Attribute
      if (typeof value === "string") {
        element.setAttribute(key, value);
        continue;
      }
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
