import { Rule } from "eslint";

export const findParent = (
  node: Rule.NodeParentExtension,
  search: (parent?: Rule.Node) => boolean,
): Rule.Node | null => {
  if (node.parent == null) {
    return null;
  }
  if (search(node.parent)) {
    return node.parent;
  }
  return findParent(node.parent, search);
};
