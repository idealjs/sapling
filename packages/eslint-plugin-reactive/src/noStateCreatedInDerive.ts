import type { Rule } from "eslint";

import { findParent } from "./utils";

export const noStateCreatedInDerive: Rule.RuleModule = {
  meta: {
    messages: {
      "state-created-in-derive": "state create in derive.",
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
        if (node.callee.name === "createState") {
          const targetParent = findParent(node, (parent) => {
            return (
              parent?.type === "CallExpression" &&
              parent.callee.type === "Identifier" &&
              parent.callee.name === "derive"
            );
          });
          if (targetParent != null) {
            context.report({
              messageId: "state-created-in-derive",
              node,
            });
          }
        }
      }
    },
  }),
};
