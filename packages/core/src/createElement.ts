import * as CSS from "csstype";

import { hyper } from "./hyper";
import { derive, effect, StateView } from "./reactive";
import { OrFunction, Primitive, Tags } from "./type";

type Dispose = () => void;

export type ComponentChild = {
  el?: Primitive | Element | Node | DocumentFragment | null | undefined;
  disposeStack?: StateView<Dispose>[];
};

type PrimitiveChild = Primitive;

export type ComponentChildren =
  | ComponentChild
  | PrimitiveChild
  | (() => ComponentChild | PrimitiveChild)
  | (
      | ComponentChild
      | PrimitiveChild
      | (() => ComponentChild | PrimitiveChild)
    )[];

const isPrimitive = (value: unknown): value is Primitive => {
  return value !== Object(value);
};

let globalDisposeStack: StateView<Dispose | void>[] = [];

export const useEffect = (callback: () => Dispose | void) => {
  const disposeState = effect(callback);
  globalDisposeStack.push(disposeState);
};

const findNextSibling = (
  upsertCache: (
    | {
        el: Node | null | undefined;
        disposeStack?: (StateView<Dispose> | undefined)[];
        fragmentChildren?: Node[];
      }
    | undefined
  )[],
  currentIndex: number,
) => {
  let nextSibling;
  for (let i = currentIndex + 1; i < upsertCache.length; i++) {
    if (upsertCache[i]?.el != null) {
      if (
        upsertCache[i]?.el instanceof DocumentFragment &&
        upsertCache[i]?.fragmentChildren?.[0] != null
      ) {
        nextSibling = upsertCache[i]?.fragmentChildren?.[0];
        break;
      }
      if (
        upsertCache[i]?.el instanceof DocumentFragment &&
        (upsertCache[i]?.fragmentChildren == null ||
          upsertCache[i]?.fragmentChildren?.length === 0)
      ) {
        continue;
      }
      if (upsertCache[i]?.el instanceof Element) {
        nextSibling = upsertCache[i]?.el;
      }
    }
  }
  return nextSibling;
};

export const upsert = (
  element: Node,
  ...children: (
    | ComponentChild
    | PrimitiveChild
    | (() => ComponentChild | PrimitiveChild)
  )[]
) => {
  const upsertCache: (
    | {
        el: Node | null | undefined;
        disposeStack?: StateView<Dispose>[];
        fragmentChildren?: Node[];
      }
    | undefined
  )[] = [];
  children.forEach((child, index) => {
    if (typeof child === "function") {
      derive(() => {
        const cache = upsertCache[index];
        cache?.disposeStack?.forEach((dispose) => dispose?.val());
        const _result = child();
        let result: ComponentChild;
        if (isPrimitive(_result)) {
          result = {
            el: _result,
            disposeStack: [],
          };
        } else {
          result = _result;
        }
        // fragment replace
        if (
          cache?.el != null &&
          result.el != null &&
          result.el instanceof DocumentFragment
        ) {
          const nextSibling = findNextSibling(upsertCache, index);
          const fragmentChildren = [...result.el.childNodes];
          upsertCache[index]?.fragmentChildren?.forEach((child) =>
            element.removeChild(child),
          );
          if (nextSibling != null) {
            element.insertBefore(nextSibling, result.el);
          } else {
            element.appendChild(result.el);
          }

          upsertCache[index] = {
            el: result.el,
            disposeStack: result.disposeStack,
            fragmentChildren: fragmentChildren,
          };
          return element;
        }

        // remount
        if (cache != null && cache?.el == null && result.el != null) {
          const nextSibling = findNextSibling(upsertCache, index);

          if (nextSibling != null && result.el instanceof Node) {
            element.insertBefore(nextSibling, result.el);
            upsertCache[index] = {
              el: result.el,
              disposeStack: result.disposeStack,
            };
            return element;
          }
          if (nextSibling == null && result.el instanceof Node) {
            element.appendChild(result.el);
            upsertCache[index] = {
              el: result.el,
              disposeStack: result.disposeStack,
            };
            return element;
          }
          if (nextSibling != null && !(result.el instanceof Node)) {
            const text = new Text(result.el.toString());
            element.insertBefore(nextSibling, text);
            upsertCache[index] = {
              el: text,
              disposeStack: result.disposeStack,
            };
            return element;
          }
          if (nextSibling == null && !(result.el instanceof Node)) {
            const text = new Text(result.el.toString());
            element.appendChild(text);
            upsertCache[index] = {
              el: text,
              disposeStack: result.disposeStack,
            };
            return element;
          }

          return element;
        }

        // unmount
        if (cache?.el != null && result.el == null) {
          element.removeChild(cache.el);
          if (cache.el instanceof DocumentFragment) {
            cache.fragmentChildren?.forEach((child) =>
              element.removeChild(child),
            );
          }
          upsertCache[index] = {
            el: result.el,
            disposeStack: result.disposeStack,
          };
          return element;
        }

        // replace
        if (
          cache?.el != null &&
          result.el != null &&
          result.el instanceof Element
        ) {
          element.replaceChild(result.el, cache.el);
          upsertCache[index] = {
            el: result.el,
            disposeStack: result.disposeStack,
          };
          return element;
        }

        if (
          cache?.el != null &&
          result.el != null &&
          !(result.el instanceof Node)
        ) {
          const text = new Text(result.el.toString());
          element.replaceChild(text, cache.el);
          upsertCache[index] = { el: text, disposeStack: result.disposeStack };
          return element;
        }

        // add
        if (cache == null && result.el != null && result.el instanceof Node) {
          upsertCache[index] = {
            el: result.el,
            disposeStack: result.disposeStack,
          };
          return element.appendChild(result.el);
        }

        if (
          cache == null &&
          result.el != null &&
          !(result.el instanceof Node)
        ) {
          const text = new Text(result.el.toString());
          upsertCache[index] = { el: text, disposeStack: result.disposeStack };
          return element.appendChild(text);
        }
      });
      return element;
    }

    if (isPrimitive(child)) {
      const text = new Text(child.toString());
      upsertCache[index] = { el: text };
      return element.appendChild(text);
    }

    if (child.el instanceof Node) {
      upsertCache[index] = { el: child.el };
      return element.appendChild(child.el);
    }

    if (child.el != null) {
      const text = new Text(child.el.toString());
      upsertCache[index] = { el: text };
      return element.appendChild(text);
    }
  });
};

export type TagOption<K extends keyof Tags> = Omit<
  OrFunction<Partial<Tags[K]>>,
  "children" | "style"
> & {
  children?: ComponentChildren;
  style?: CSS.Properties;
  dispose?: StateView<Dispose>[];
};

function createElement<P = object>(
  jsxTag: (props: P) => ComponentChild,
  options?: P,
): ComponentChild;

function createElement(
  jsxTag: keyof Tags,
  options?: TagOption<keyof Tags>,
): ComponentChild;

function createElement<P = object>(
  jsxTag: keyof Tags | ((props: P) => ComponentChild),
  options?: TagOption<keyof Tags> | P,
): ComponentChild {
  if (typeof jsxTag === "function") {
    const temp = globalDisposeStack;
    const disposeStack: StateView<Dispose>[] = [];
    globalDisposeStack = disposeStack;
    const component = jsxTag(options as P);
    globalDisposeStack = temp;
    return { el: component.el, disposeStack };
  }

  const { children, dispose, ...props } = options as TagOption<keyof Tags>;

  const temp = globalDisposeStack;
  const disposeStack: StateView<Dispose>[] = dispose ?? [];
  globalDisposeStack = disposeStack;

  const el =
    jsxTag === "fragment"
      ? document.createDocumentFragment()
      : hyper(jsxTag, props);

  if (typeof children === "function") {
    upsert(el, children);
  } else if (Array.isArray(children)) {
    upsert(el, ...children);
  } else if (children != null) {
    upsert(el, children);
  }

  globalDisposeStack = temp;
  return {
    el: el,
    disposeStack,
  };
}

export default createElement;

export const lazy =
  <V>(v: V) =>
  () =>
    v;
