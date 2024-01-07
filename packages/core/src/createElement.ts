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
import arrify from "./utils/arrify";
import isPrimitive from "./utils/isPrimitive";

type Dispose = () => void;

type JSXTag<P> =
  | keyof TagNameMap
  | ((props: P) => SaplingElement | SaplingElement[] | PrimitiveChild | null);

export type JSXElementType<P> = (props: P) => SaplingNode | SaplingElement;

export type SaplingNode =
  | SaplingElement
  | PrimitiveChild
  | Iterable<SaplingNode>
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

  public appendChildJSXNode = (childrenNode: SaplingElement[]) => {
    [...childrenNode].reduceRight(
      (p: SaplingElement | null, c): SaplingElement => {
        if (c.el?.parentElement != null) {
          return c;
        }
        if (p == null && c.el != null) {
          this.el?.appendChild(c.el);
        }
        if (p?.el != null && c.el != null) {
          this.el?.insertBefore(c.el, p.el);
        }
        this.children.add(c);
        c.parent = this;
        return c;
      },
      null,
    );
    return this;
  };

  public removeExtraNodes = (childrenNode: Set<SaplingElement>) => {
    this.children.forEach((child) => {
      if (!childrenNode.has(child)) {
        child.dispose();
      }
    });
  };

  public mergeDisposeStack = (disposeStack: StateView<Dispose | void>[]) => {
    this.disposeStack.push(...disposeStack);
    return this;
  };
}

function prepareChildrenNodes(
  children: SaplingNode | SaplingNode[],
): SaplingElement[] {
  return arrify(children)
    .flatMap((child) => {
      if (isPrimitive(child)) {
        return primitiveToJSXNode(child);
      }
      return child;
    })
    .filter((v): v is SaplingElement => v != null);
}

const primitiveToJSXNode = (primitive: Primitive) =>
  new SaplingElement({
    node: new Text(primitive.toString()),
    disposeStack: [],
  });

const JSXFactory = () => {
  const jsxScope = new JSXScope();

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
    jsxTag: (
      props: P,
    ) => SaplingElement | SaplingElement[] | PrimitiveChild | null,
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

      let jsxNode: SaplingElement;
      if (Array.isArray(node)) {
        jsxNode = new SaplingElement({
          node: document.createDocumentFragment(),
        });
        jsxNode.appendChildJSXNode(node);
      } else if (isPrimitive(node) || node == null) {
        jsxNode = new SaplingElement({
          node: node == null ? null : new Text(node.toString()),
          disposeStack,
          children: null,
        });
      } else {
        jsxNode = node;
      }
      jsxNode.mergeDisposeStack(disposeStack);
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

    const jsxNode = new SaplingElement({
      node: el,
    });

    if (children != null) {
      let nodeCaches: (Map<Key, SaplingElement> | undefined)[] = [];
      effect(() => {
        const childrenNode = arrify(children).map((child, index) => {
          const nodeCache = (nodeCaches[index] ||= new Map<
            Key,
            SaplingElement
          >());
          const resume = jsxScope.collectNodeCache(nodeCache);
          const children = prepareChildrenNodes(
            typeof child === "function" ? child() : child,
          );
          resume();
          return children;
        });

        jsxNode.removeExtraNodes(
          new Set(childrenNode.flatMap((child) => child)),
        );
        jsxNode.appendChildJSXNode(childrenNode.flatMap((child) => child));
      });
    }

    if (key != null) {
      jsxScope.setCache(key, jsxNode);
    }

    return jsxNode;
  }

  const useEffect = (callback: () => Dispose | void) => {
    jsxScope.addDispose(effect(callback));
  };

  return { jsxScope, createElement, upsert, useEffect };
};

export const { jsxScope, createElement, useEffect, upsert } = JSXFactory();

export default createElement;
