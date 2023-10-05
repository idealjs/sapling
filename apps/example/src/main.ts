import {
  add,
  createElement,
  createState,
  upsert,
  useEffect,
} from "@idealjs/reactive";

const TodoItem2 = (props: { name: number }) => {
  const { name } = props;
  const state = createState(0);

  useEffect(() => {
    const handler = setInterval(() => {
      state.val++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  return createElement("p", {
    children: () => ({ el: `${name} counter ${state.val}` }),
  });
};

const Component2 = () => {
  const items = createState<number[]>([]);

  return createElement("div", {
    children: [
      createElement("button", {
        children: () => ({
          el: `add item ${items.val.length}`,
          disposeStack: [],
        }),
        onclick: () => {
          items.val = [...(items.val ?? []), items.val.length];
        },
        style: {
          color: "blue",
        },
      }),
      () => {
        const fragment = document.createDocumentFragment();

        const components = items.val.map((item) => {
          const component = createElement(TodoItem2, { name: item });
          add(fragment, component.el);
          return component;
        });

        return {
          el: fragment,
          disposeStack: components.flatMap(
            (component) => component.disposeStack,
          ),
        };
      },
    ],
  });
};

const root = document.getElementById("app")!;

upsert(root, createElement(Component2));
