import { Dispose, effect, reactiveScope } from "@idealjs/sapling-reactive";

import { hyper } from "./hyper";
import TreeNode from "./TreeNode";
import {
  DisposeStack,
  InnerElement,
  Key,
  Primitive,
  PrimitiveChild,
  TagNameMap,
  TagOption,
} from "./type";
import isPrimitive from "./utils/isPrimitive";
import numberConcat from "./utils/numberConcat";

type JSXTag<P> = keyof TagNameMap | ((props: P) => SaplingNode);

export type JSXElementType<P> = (props: P) => SaplingNode | SaplingElement;

export type SaplingNode =
  | SaplingElement
  | PrimitiveChild
  | SaplingNode[]
  | (() => SaplingNode)
  | null;

export class JSXScope {
  private disposeStack: DisposeStack | null = null;
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

  public collectDispose = (disposeStack: DisposeStack) => {
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

  public addDispose = (dispose: { val: Dispose | void }) => {
    this.disposeStack?.push(dispose);
  };

  public getDisposeStack = () => this.disposeStack ?? [];
}

export class SaplingElement extends TreeNode<Node> {}

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
        staticContainer: true,
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
      const nodeCache =
        nodeCaches != null
          ? (nodeCaches[cacheKey] ||= new Map<Key, SaplingElement>())
          : null;

      const saplingElement =
        nodeCache?.get(saplingNode) || primitiveToJSXNode(saplingNode);

      nodeCache?.set(saplingNode, saplingElement);
      return saplingElement;
    }
    if (typeof saplingNode === "function") {
      const nodeCache =
        nodeCaches == null
          ? null
          : (nodeCaches[cacheKey] ||= new Map<Key, SaplingElement>());
      const resume =
        nodeCache != null ? jsxScope.collectNodeCache(nodeCache) : null;

      const element = prepareSaplingElement(saplingNode(), nodeCaches);

      resume?.();
      return element;
    }
    return new SaplingElement();
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
    const resumeCollectDeps = reactiveScope.pauseCollectDeps();

    const cache = jsxScope.getCache(key);
    if (cache != null) {
      resumeCollectDeps();
      return cache;
    }

    if (typeof jsxTag === "function") {
      // collect user component effect's dispose
      const disposeStack: DisposeStack = [];
      const resume = jsxScope.collectDispose(disposeStack);
      const node = jsxTag(options as P);
      const currentElement = new SaplingElement({
        staticContainer: true,
        disposeStack,
      });
      currentElement.migrate(null, prepareSaplingElement(node));

      resume();
      if (key != null) {
        jsxScope.setCache(key, currentElement);
      }
      resumeCollectDeps();
      return currentElement;
    }

    const { children, ref, ...props } = (options ?? {}) as TagOption<
      keyof InnerElement
    >;

    const el = hyper(jsxTag, props);
    if (!(el instanceof DocumentFragment) && ref != null) {
      ref.current = el;
    }

    const currentElement = new SaplingElement({
      node: el,
    });

    let childrenElement: SaplingElement | null = null;

    if (children != null) {
      let nodeCaches: Map<Key, SaplingElement>[] = [];
      effect(() => {
        const currentChildrenElement = prepareSaplingElement(
          children,
          nodeCaches,
        );
        currentElement.migrate(childrenElement, currentChildrenElement);
        childrenElement = currentChildrenElement;
      });
    }

    if (key != null) {
      jsxScope.setCache(key, currentElement);
    }
    resumeCollectDeps();
    return currentElement;
  }

  const useEffect = (callback: () => Dispose | void) => {
    jsxScope.addDispose(effect(callback));
  };

  return { jsxScope, createElement, useEffect, prepareSaplingElement };
};

export const { jsxScope, createElement, useEffect, prepareSaplingElement } =
  JSXFactory();

export default createElement;
