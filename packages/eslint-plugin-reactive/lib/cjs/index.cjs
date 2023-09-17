'use strict';

const findParent = (node, search) => {
  if (node.parent == null) {
    return null;
  }
  if (search(node.parent)) {
    return node.parent;
  }
  return findParent(node.parent, search);
};

const noNestedDeriveCall = {
  meta: {
    messages: {
      "nested-derive-call": "Nested 'derive' call in the derive's callback."
    },
    type: "problem",
    schema: [],
    docs: {
      description: ""
    }
  },
  create: (context) => ({
    CallExpression: (node) => {
      if (node.callee.type === "Identifier") {
        if (node.callee.name === "derive") {
          const targetParent = findParent(node, (parent) => {
            return (parent == null ? void 0 : parent.type) === "CallExpression" && parent.callee.type === "Identifier" && parent.callee.name === "derive";
          });
          if (targetParent != null) {
            context.report({
              messageId: "nested-derive-call",
              node
            });
          }
        }
      }
    }
  })
};

const noStateCreatedInDerive = {
  meta: {
    messages: {
      "state-created-in-derive": "state created in derive."
    },
    type: "problem",
    schema: [],
    docs: {
      description: ""
    }
  },
  create: (context) => ({
    CallExpression: (node) => {
      if (node.callee.type === "Identifier") {
        if (node.callee.name === "createState") {
          const targetParent = findParent(node, (parent) => {
            return (parent == null ? void 0 : parent.type) === "CallExpression" && parent.callee.type === "Identifier" && parent.callee.name === "derive";
          });
          if (targetParent != null) {
            context.report({
              messageId: "state-created-in-derive",
              node
            });
          }
        }
      }
    }
  })
};

const plugin = {
  name: "@idealjs/reactive",
  rules: {
    "no-nested-derive-call": noNestedDeriveCall,
    "no-state-created-in-derive": noStateCreatedInDerive
  }
};

module.exports = plugin;
