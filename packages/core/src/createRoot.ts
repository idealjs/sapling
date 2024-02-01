import {} from "./createElement";
import { SaplingElement } from "./type";

const createRoot = (element: Node) => {
  const render = (node: SaplingElement) => {};

  return {
    render,
  };
};

export default createRoot;
