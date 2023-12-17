import { createState, upsert, useEffect } from "@idealjs/sapling";

const TodoItem = (props: { name: number }) => {
  const { name } = props;
  const state = createState(0);
  const ref = createState<HTMLParagraphElement>(null);
  useEffect(() => {
    const handler = setInterval(() => {
      state.val++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  return (
    <p>
      {() => {
        return `${name} counter ${state.val}`;
      }}
    </p>
  );
};

const Component = () => {
  const items = createState<{ id: number }[]>([]);
  return (
    <div>
      <button
        onClick={() => {
          items.val = [...(items.val ?? []), { id: items.val.length }];
        }}
      >
        add new item
      </button>
      <div>{() => `add item ${items.val.length}`}</div>
      <div>
        {() =>
          items.val.map((item) => {
            return <TodoItem name={item.id} key={item.id} />;
          })
        }
      </div>
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
