import { JSXChildren, JSXNode } from "./createElement";
import { effect } from "./reactive";
import { Key } from "./type";
import arrify from "./utils/arrify";
import isPrimitive from "./utils/isPrimitive";

const Fragment = (props: { children: JSXChildren }) => {
  const { children } = props;
  document.createDocumentFragment();
  const cacheNodes = new Map<Key, JSXNode>();
  const upsertCache = new Set<JSXNode>();
  
  effect(() => {
    const _children = typeof children === "function" ? children() : children;
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
  });
};

export default Fragment;
