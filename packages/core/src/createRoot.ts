import {
  prepareSaplingElement,
  SaplingElement,
  SaplingNode,
} from "./createElement";
import { Key } from "./type";

const createRoot = (element: Node) => {
  const currentElement = new SaplingElement({
    node: element,
  });
  let nodeCaches: Map<Key, SaplingElement>[] = [];

  const render = (node: SaplingNode) => {
    const currentChildrenElement = prepareSaplingElement(node, nodeCaches);
    currentElement.migrate(null, currentChildrenElement);
    return () => {
      currentElement.dispose();
    };
  };

  return {
    render,
  };
};

export default createRoot;
