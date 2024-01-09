import { hyper } from "./hyper";
import { effect, StateView } from "./reactive";
import {
  InnerElement,
  Key,
  Primitive,
  PrimitiveChild,
  TagNameMap,
  TagOption,
} from "./type";
import isPrimitive from "./utils/isPrimitive";
import numberConcat from "./utils/numberConcat";

type Dispose = () => void;

type JSXTag<P> = keyof TagNameMap | ((props: P) => SaplingNode);

export type JSXElementType<P> = (props: P) => SaplingNode | SaplingElement;

export type SaplingNode =
  | SaplingElement
  | PrimitiveChild
  | SaplingNode[]
  | (() => SaplingNode)
  | null;

export class JSXScope {
  private disposeStack: StateView<Dispose | void>[] | null = null;
  private nodeCache: Map<Key, SaplingElement> | null = null;

  constructor() {}

  public getCache = (key: Key | undefined) => {
    if (key == null) {
      return null;
    }
    return this.nodeCache?.get(key);
  };

  public setCache = (key: Key, value: SaplingElement) => {
    this.nodeCache?.set(key, value);
  };

  public collectDispose = (disposeStack: StateView<Dispose | void>[]) => {
    const temp = this.disposeStack;
    this.disposeStack = disposeStack;
    return () => {
      this.disposeStack = temp;
    };
  };

  public collectNodeCache = (nodeCache: Map<Key, SaplingElement>) => {
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

export class SaplingElement {
  private _el: Node | null = null;
  private disposeStack: StateView<Dispose | void>[] = [];
  private children: Set<SaplingElement> = new Set();
  private parent: SaplingElement | null = null;

  public get el() {
    return this._el;
  }

  constructor(params?: {
    node?: Node | null;
    disposeStack?: StateView<Dispose | void>[];
    children?: Set<SaplingElement> | null;
  }) {
    if (params?.node != null) {
      this._el = params.node;
    }
    if (params?.disposeStack != null) {
      this.disposeStack = params.disposeStack;
    }
    if (params?.children != null) {
      this.children = params.children;
      Array.from(params.children).forEach(
        (child) => child.el != null && this._el?.appendChild(child.el),
      );
    }
  }

  public dispose = () => {
    this.el?.parentNode?.removeChild(this.el);
    Array.from(this.children).forEach((child) => {
      child.dispose();
    });
    this.disposeStack.forEach((dispose) => dispose.val?.());
    this.parent?.children.delete(this);
  };

  public append = (child: SaplingElement) => {
    if (child.el != null) {
      if (child.el.parentElement != null) {
        // skip append for optimization
        return child;
      }
      this.el?.appendChild(child.el);
      this.children.add(child);
      return child;
    } else {
      return Array.from(child.children).reduce(
        (p: SaplingElement | null, c): SaplingElement | null => {
          if (c.el == null) {
            return this.append(c) ?? p;
          }
          if (c.el != null) {
            if (c.el?.parentElement != null) {
              return c;
            }
            if (p == null) {
              // start from here
              this.el?.appendChild(c.el);
            }
            if (p?.el != null) {
              // continue insert
              if (p.el.nextSibling != null) {
                this.el?.insertBefore(c.el, p.el.nextSibling);
              } else {
                this.el?.appendChild(c.el);
              }
            }
            this.children.add(c);
            c.parent = this;
          }

          return c;
        },
        null,
      );
    }
  };

  public hasChild = (childElement: SaplingElement): boolean => {
    return (
      this.children.has(childElement) ||
      Array.from(this.children).reduce((p, c) => {
        return p || c.hasChild(childElement);
      }, false)
    );
  };

  public disposeElementNotIn = (childElement: SaplingElement) => {
    this.children.forEach((child) => {
      if (!childElement.hasChild(child)) {
        child.dispose();
      }
    });
  };

  public mergeDisposeStack = (disposeStack: StateView<Dispose | void>[]) => {
    this.disposeStack.push(...disposeStack);
    return this;
  };
}

const primitiveToJSXNode = (primitive: Primitive) =>
  new SaplingElement({
    node: new Text(primitive.toString()),
    disposeStack: [],
  });

const JSXFactory = () => {
  const jsxScope = new JSXScope();

  // parse SaplingNode to SaplingElement
  const prepareSaplingElement = (
    saplingNode: SaplingNode,
    nodeCaches?: Map<Key, SaplingElement>[],
    cacheKey: number = 0,
  ): SaplingElement => {
    if (saplingNode instanceof SaplingElement) {
      return saplingNode;
    }
    if (Array.isArray(saplingNode)) {
      return new SaplingElement({
        children: new Set(
          saplingNode.map((child, index) =>
            prepareSaplingElement(
              child,
              nodeCaches,
              numberConcat(index, cacheKey),
            ),
          ),
        ),
      });
    }
    if (isPrimitive(saplingNode)) {
      return primitiveToJSXNode(saplingNode);
    }
    if (typeof saplingNode === "function") {
      let resume;

      if (nodeCaches != null) {
        const nodeCache = (nodeCaches[cacheKey] ||= new Map<
          Key,
          SaplingElement
        >());
        resume = jsxScope.collectNodeCache(nodeCache);
      }

      const element = prepareSaplingElement(saplingNode(), nodeCaches);
      resume?.();
      return element;
    }
    return new SaplingElement();
  };

  const upsert = (element: Node, child: SaplingElement) => {
    child.el && element.appendChild(child.el);
    return () => {
      child.dispose();
    };
  };

  function createElement(
    jsxTag: keyof TagNameMap,
    options?: TagOption<keyof InnerElement>,
    key?: Key,
    _isStaticChildren?: boolean,
    _source?: {
      columnNumber: number;
      fileName: string;
      lineNumber: number;
    },
    _self?: unknown,
  ): SaplingElement;

  function createElement<P extends object>(
    jsxTag: (props: P) => SaplingNode,
    options?: P,
    key?: Key,
    _isStaticChildren?: boolean,
    _source?: {
      columnNumber: number;
      fileName: string;
      lineNumber: number;
    },
    _self?: unknown,
  ): SaplingElement;

  function createElement<P extends object>(
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
  ): SaplingElement {
    const cache = jsxScope.getCache(key);
    if (cache != null) {
      return cache;
    }

    if (typeof jsxTag === "function") {
      // collect user component effect's dispose
      const disposeStack: StateView<void | Dispose>[] = [];
      const resume = jsxScope.collectDispose(disposeStack);
      const node = jsxTag(options as P);
      resume();
      const element = prepareSaplingElement(node);
      element.mergeDisposeStack(disposeStack);
      if (key != null) {
        jsxScope.setCache(key, element);
      }
      return element;
    }

    const { children, ref, ...props } = (options ?? {}) as TagOption<
      keyof InnerElement
    >;

    const el = hyper(jsxTag, props);
    if (!(el instanceof DocumentFragment) && ref != null) {
      ref.val = el;
    }

    const currentElement = new SaplingElement({
      node: el,
    });

    if (children != null) {
      let nodeCaches: Map<Key, SaplingElement>[] = [];
      effect(() => {
        const element = prepareSaplingElement(children, nodeCaches);
        currentElement.disposeElementNotIn(element);
        currentElement.append(element);
      });
    }

    if (key != null) {
      jsxScope.setCache(key, currentElement);
    }

    return currentElement;
  }

  const useEffect = (callback: () => Dispose | void) => {
    jsxScope.addDispose(effect(callback));
  };

  return { jsxScope, createElement, upsert, useEffect };
};

export const { jsxScope, createElement, useEffect, upsert } = JSXFactory();

export default createElement;
