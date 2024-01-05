import { hyper } from "./hyper";
import { derive, effect, StateView } from "./reactive";
import {
  AsJSXChildren,
  InnerElement,
  Key,
  Primitive,
  TagNameMap,
  TagOption,
} from "./type";
import arrify from "./utils/arrify";
import isPrimitive from "./utils/isPrimitive";

type Dispose = () => void;

export type PrimitiveChild = Primitive;

export type RawChild = JSXNode | PrimitiveChild | null;

export type JSXChildren = AsJSXChildren<RawChild>;

type JSXTag<P> =
  | keyof TagNameMap
  | ((props: P) => JSXNode | PrimitiveChild | null);

export class JSXScope {
  private disposeStack: StateView<Dispose | void>[] | null = null;
  private nodeCache: Map<Key, JSXNode> | null = null;

  constructor() {}

  public getCache = (key: Key | undefined) => {
    if (key == null) {
      return null;
    }
    return this.nodeCache?.get(key);
  };

  public setCache = (key: Key, value: JSXNode) => {
    this.nodeCache?.set(key, value);
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

export class JSXNode {
  private _el: Node | null = null;
  private disposeStack: StateView<Dispose | void>[] = [];
  private children: Set<JSXNode> = new Set();
  private parent: JSXNode | null = null;

  public get el() {
    return this._el;
  }

  constructor(params?: {
    node?: Node | null;
    disposeStack?: StateView<Dispose | void>[];
    children?: Set<JSXNode> | null;
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

  public appendChildJSXNode = (childrenNode: JSXNode[]) => {
    [...childrenNode].reduceRight((p: JSXNode | null, c): JSXNode => {
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
    }, null);
    return this;
  };

  public removeExtraNodes = (childrenNode: Set<JSXNode>) => {
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

function prepareChildrenNodes(children: RawChild | RawChild[]): JSXNode[] {
  return arrify(children)
    .flatMap((child) => {
      if (isPrimitive(child)) {
        return primitiveToJSXNode(child);
      }
      return child;
    })
    .filter((v): v is JSXNode => v != null);
}

const primitiveToJSXNode = (primitive: Primitive) =>
  new JSXNode({
    node: new Text(primitive.toString()),
    disposeStack: [],
  });

const JSXFactory = () => {
  const jsxScope = new JSXScope();

  const upsert = (element: Node, child: JSXNode) => {
    child.el && element.appendChild(child.el);
    return () => {
      child.dispose();
    };
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

    if (typeof jsxTag === "function") {
      // collect user component effect's dispose
      const disposeStack: StateView<void | Dispose>[] = [];
      const resume = jsxScope.collectDispose(disposeStack);
      const node = jsxTag(options as P);
      resume();
      const jsxNode =
        isPrimitive(node) || node == null
          ? new JSXNode({
              node: node == null ? null : new Text(node.toString()),
              disposeStack,
              children: null,
            })
          : node.mergeDisposeStack(disposeStack);
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

    const jsxNode = new JSXNode({
      node: el,
    });

    if (children != null) {
      let nodeCaches: (Map<Key, JSXNode> | undefined)[] = [];
      effect(() => {
        const childrenNode = arrify(children).map((child, index) => {
          const nodeCache = (nodeCaches[index] ||= new Map<Key, JSXNode>());
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
  };

  const useEffect = (callback: () => Dispose | void) => {
    jsxScope.addDispose(effect(callback));
  };

  return { jsxScope, createElement, upsert, useEffect };
};

export const { jsxScope, createElement, useEffect, upsert } = JSXFactory();

export default createElement;
