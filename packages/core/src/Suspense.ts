import { effect } from "@idealjs/sapling-reactive";

import {
  prepareSaplingElement,
  SaplingElement,
  SaplingNode,
} from "./createElement";
import { Key } from "./type";

const Suspense = (props: {
  fallback?: () => SaplingNode;
  children: () => SaplingNode;
}) => {
  const { fallback, children } = props;

  const currentElement = new SaplingElement({
    children:
      fallback != null
        ? new Set([prepareSaplingElement(fallback())])
        : undefined,
  });

  let nodeCaches: Map<Key, SaplingElement>[] = [];

  let childrenElement: SaplingElement | null = null;

  effect(() => {
    const promise = new Promise<SaplingNode>((resolve, reject) => {
      resolve(children());
    });
    promise.then((children) => {
      const currentChildrenElement = prepareSaplingElement(
        children,
        nodeCaches,
      );
      currentElement.migrate(childrenElement, currentChildrenElement);
      childrenElement = currentChildrenElement;
    });
  });

  return currentElement;
};

export default Suspense;
