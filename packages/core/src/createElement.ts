import { diff } from "@egjs/list-differ";
import { Dispose, effect, reactiveScope } from "@idealjs/sapling-reactive";

import { hyper } from "./hyper";
import {
  DisposeStack,
  InnerElement,
  Key,
  Primitive,
  PrimitiveChild,
  SaplingElement,
  TagNameMap,
  TagOption,
} from "./type";
import arrify from "./utils/arrify";
import isPrimitive from "./utils/isPrimitive";
import numberConcat from "./utils/numberConcat";

type JSXTag<P> = keyof TagNameMap | ((props: P) => SaplingElement);

export type WithoutFn<T> = T extends () => SaplingNode ? never : T;

export type WithOutRecursion<T> = T extends () => unknown
  ? never
  : T extends () => (infer U extends (infer A)[])[]
    ? never
    : T;

export type SaplingNode =
  | SaplingElement
  | PrimitiveChild
  | SaplingNode[]
  | (() => WithOutRecursion<SaplingNode>)
  | null;

export class JSXScope {
  private disposeStack: DisposeStack | null = null;

  constructor() {}

  public collectDispose = (disposeStack: DisposeStack) => {
    const temp = this.disposeStack;
    this.disposeStack = disposeStack;
    return () => {
      this.disposeStack = temp;
    };
  };

  public addDispose = (dispose: { val: Dispose | void }) => {
    this.disposeStack?.push(dispose);
  };

  public getDisposeStack = () => this.disposeStack ?? [];
}

const dispose = (saplingNode: SaplingNode, el: Node) => {};

const appendChild = (node: SaplingNode, el: Node) => {
  if (node instanceof Node) {
    el.appendChild(node);
  } else if (Array.isArray(node)) {
    node.forEach((v) => appendChild(v, el));
  } else if (isPrimitive(node)) {
    el.textContent = node.toString();
  } else if (typeof node === "function") {
    throw new Error("appendChild should not has a function");
  }
};

const mountChildren = (el: Node, children: SaplingNode) => {
  if (children instanceof Node) {
    el.appendChild(children);
  } else if (isPrimitive(children)) {
    el.appendChild(document.createTextNode(children.toString()));
  } else if (Array.isArray(children)) {
    children.forEach((child) => mountChildren(el, child));
  } else if (typeof children === "function") {
    effect(() => {
      const node = children();
      mountChildren(el, node);
    });
  }
};

const JSXFactory = () => {
  const jsxScope = new JSXScope();

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
    jsxTag: (props: P) => SaplingElement,
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
    // console.log("test test", jsxTag);
    if (typeof jsxTag === "function") {
      return jsxTag(options as P);
    }

    const { children, ref, ...props } = (options ?? {}) as TagOption<
      keyof InnerElement
    >;
    const el = hyper(jsxTag, props);
    children != null && mountChildren(el, children);

    return el;
  }

  const useEffect = (callback: () => Dispose | void) => {
    jsxScope.addDispose(effect(callback));
  };

  return { jsxScope, createElement, useEffect };
};

export const { jsxScope, createElement, useEffect } = JSXFactory();

export default createElement;
