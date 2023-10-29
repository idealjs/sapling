import * as CSS from "csstype";

import { hyper } from "./hyper";
import { derive, effect, State, StateView } from "./reactive";
import { Key, OrFunction, Primitive, TagNameMap } from "./type";

type Dispose = () => void;

export type JSXNode = {
  childNode: Node | null;
  disposeStack: StateView<Dispose | void>[];
};

export type PrimitiveChild = Primitive;

export type JSXChildren = OrFunction<
  JSXNode | PrimitiveChild | null | (JSXNode | PrimitiveChild | null)[]
>;

export const isPrimitive = (value: unknown): value is Primitive => {
  return typeof value !== "object" && typeof value !== "function";
};

let exchangeDisposeStack: StateView<Dispose | void>[] = [];
let exchangeNodeCache: Map<Key, JSXNode> = new Map();

export const useEffect = (callback: () => Dispose | void) => {
  const disposeState = effect(callback);
  exchangeDisposeStack.push(disposeState);
};

const arrify = <T>(v: T | T[]) => {
  if (Array.isArray(v)) {
    return v;
  }
  return [v];
};

export const upsert = (
  element: Node,
  children: JSXChildren,
  nodeCache: Map<Key, JSXNode> = new Map(),
) => {
  let upsertCache = new Set<JSXNode>();

  derive(() => {
    const _upserCache = new Set<JSXNode>();
    const temp = exchangeNodeCache;
    exchangeNodeCache = nodeCache;
    const _children = typeof children === "function" ? children() : children;
    exchangeNodeCache = temp;

    const childrenNode = new Set(
      arrify(_children)
        .map((child) => {
          if (isPrimitive(child)) {
            return { childNode: new Text(child.toString()), disposeStack: [] };
          }
          if (child?.childNode instanceof Node) {
            return child;
          }
        })
        .filter((v): v is JSXNode => v != null),
    );

    upsertCache.forEach((cacheNode) => {
      if (!childrenNode.has(cacheNode)) {
        cacheNode.childNode?.parentElement?.removeChild(cacheNode.childNode);
        cacheNode.disposeStack.forEach((dispose) => dispose.val?.());
      }
    });

    childrenNode.forEach((child) => {
      if (child.childNode != null) {
        _upserCache.add(child);
        return element.appendChild(child.childNode);
      }
    });
    // debugger;

    upsertCache.clear();
    upsertCache = _upserCache;
  });
};

export type TagOption<K extends keyof TagNameMap> = Partial<
  Omit<TagNameMap[K], "children" | "style">
> & {
  ref?: State<TagNameMap[K] | null>;
  children?: JSXChildren;
  style?: OrFunction<CSS.Properties>;
};

type JSXTag<P> =
  | keyof TagNameMap
  | ((props: P) => JSXNode | PrimitiveChild | null);

const getCache = (cache: Map<Key, JSXNode>, key: Key | undefined) => {
  if (key == null) {
    return null;
  }
  return cache.get(key);
};

function createElement<P extends object>(
  jsxTag: JSXTag<P>,
  options?: TagOption<keyof TagNameMap> | P,
  key?: Key,
  isStaticChildren?: boolean,
  source?: {
    columnNumber: number;
    fileName: string;
    lineNumber: number;
  },
  self?: unknown,
  nodeCache: Map<Key, JSXNode> = exchangeNodeCache,
): JSXNode {
  const cache = getCache(nodeCache, key);
  if (cache != null) {
    return cache;
  }

  if (typeof jsxTag === "function") {
    const disposeStack: StateView<void | Dispose>[] = [];
    const temp = exchangeDisposeStack;
    exchangeDisposeStack = disposeStack;
    const node = jsxTag(options as P);
    exchangeDisposeStack = temp;
    const jsxNode =
      isPrimitive(node) || node == null
        ? {
            childNode: node == null ? null : new Text(node.toString()),
            disposeStack,
          }
        : node;
    if (key != null) {
      nodeCache.set(key, jsxNode);
    }
    return jsxNode;
  }

  const { children, ref, ...props } = (options ?? {}) as TagOption<
    keyof TagNameMap
  >;

  const el = hyper(jsxTag, props);
  if (!(el instanceof DocumentFragment) && ref != null) {
    ref.val = el;
  }

  const jsxNode = {
    childNode: el,
    disposeStack: [...exchangeDisposeStack],
  };

  if (key != null) {
    nodeCache.set(key, jsxNode);
  }

  const _nodeCache = new Map<Key, JSXNode>();
  children && upsert(el, children, _nodeCache);

  return jsxNode;
}

export default createElement;
