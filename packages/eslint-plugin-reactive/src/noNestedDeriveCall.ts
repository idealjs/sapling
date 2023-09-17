import type { Rule } from "eslint";

import { findParent } from "./utils";

export const noNestedDeriveCall: Rule.RuleModule = {
  meta: {
    messages: {
      "nested-derive-call": "derive call in the derive's callback.",
    },
    type: "problem",
    schema: [],
    docs: {
      description: "",
    },
  },
  create: (context) => ({
    CallExpression: (node) => {
      if (node.callee.type === "Identifier") {
        if (node.callee.name === "derive") {
          const targetParent = findParent(node, (parent) => {
            return (
              parent?.type === "CallExpression" &&
              parent.callee.type === "Identifier" &&
              parent.callee.name === "derive"
            );
          });
          if (targetParent != null) {
            context.report({
              messageId: "nested-derive-call",
              node,
            });
          }
        }
      }
    },
  }),
};
