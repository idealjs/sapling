import { hyper } from "./hyper";
import { effect, StateView } from "./reactive";
import {
  InnerElement,
  Key,
  OrFunction,
  Primitive,
  TagNameMap,
  TagOption,
} from "./type";
import arrify from "./utils/arrify";
import isPrimitive from "./utils/isPrimitive";

type Dispose = () => void;

export type JSXNode = {
  childNode: Node | null;
  disposeStack: StateView<Dispose | void>[];
};

export type PrimitiveChild = Primitive;

export type JSXChildren = OrFunction<
  JSXNode | PrimitiveChild | null | (JSXNode | PrimitiveChild | null)[]
>;

type JSXTag<P> =
  | keyof TagNameMap
  | ((props: P) => JSXNode | PrimitiveChild | null);

export class JSXScope {
  private disposeStack: StateView<Dispose | void>[] | null = null;
  private nodeCache: Map<Key, JSXNode> = new Map();

  constructor() {}

  public getCache = (key: Key | undefined) => {
    if (key == null) {
      return null;
    }
    return this.nodeCache.get(key);
  };

  public setCache = (key: Key, value: JSXNode) => {
    this.nodeCache.set(key, value);
  };

  public collectDispose = (disposeStack: StateView<Dispose | void>[]) => {
    const temp = this.disposeStack;
    this.disposeStack = disposeStack;
    return () => {
      this.disposeStack = temp;
    };
  };

  public collectNodeCache = (nodeCache: Map<Key, JSXNode>) => {
    const temp = this.nodeCache;
    this.nodeCache = nodeCache;
    return () => {
      this.nodeCache = temp;
    };
  };

  public addDispose = (dispose: StateView<Dispose | void>) => {
    this.disposeStack?.push(dispose);
  };

  public getDisposeStack = () => this.disposeStack ?? [];
}

const JSXFactory = () => {
  const jsxScope = new JSXScope();

  const upsert = (
    element: Node,
    children: JSXChildren,
    nodeCache = new Map<Key, JSXNode>(),
  ) => {
    let upsertCache = new Set<JSXNode>();

    return effect(() => {
      const _upserCache = new Set<JSXNode>();
      const resume = jsxScope.collectNodeCache(nodeCache);
      const _children = typeof children === "function" ? children() : children;
      resume();

      const childrenNode = new Set(
        arrify(_children)
          .map((child) => {
            if (isPrimitive(child)) {
              return {
                childNode: new Text(child.toString()),
                disposeStack: [],
              };
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

      upsertCache.clear();
      upsertCache = _upserCache;
      return () => {};
    });
  };

  const createElement = <P extends object>(
    jsxTag: JSXTag<P>,
    options?: TagOption<keyof InnerElement> | P,
    key?: Key,
    _isStaticChildren?: boolean,
    _source?: {
      columnNumber: number;
      fileName: string;
      lineNumber: number;
    },
    _self?: unknown,
  ): JSXNode => {
    const cache = jsxScope.getCache(key);
    if (cache != null) {
      return cache;
    }
    const disposeStack: StateView<void | Dispose>[] = [];
    const resume = jsxScope.collectDispose(disposeStack);

    if (typeof jsxTag === "function") {
      const node = jsxTag(options as P);
      resume();
      const jsxNode =
        isPrimitive(node) || node == null
          ? {
              childNode: node == null ? null : new Text(node.toString()),
              disposeStack,
            }
          : node;
      if (key != null) {
        jsxScope.setCache(key, jsxNode);
      }
      return jsxNode;
    }

    const { children, ref, ...props } = (options ?? {}) as TagOption<
      keyof InnerElement
    >;

    const el = hyper(jsxTag, props);
    if (!(el instanceof DocumentFragment) && ref != null) {
      ref.val = el;
    }

    children != null && upsert(el, children);
    resume();
    const jsxNode = {
      childNode: el,
      disposeStack: disposeStack,
    };

    if (key != null) {
      jsxScope.setCache(key, jsxNode);
    }

    return jsxNode;
  };

  const useEffect = (callback: () => Dispose | void) => {
    jsxScope.addDispose(effect(callback));
  };

  return { jsxScope, createElement, upsert, useEffect };
};

export const { jsxScope, createElement, useEffect, upsert } = JSXFactory();

export default createElement;
