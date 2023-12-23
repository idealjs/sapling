import { createState, upsert, useEffect } from "@idealjs/sapling";

const TodoItem = (props: { name: number; count: number }) => {
  let { name, count } = props;
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
      <div>
        {() => {
          return `${name} timmer ${state.val}.
          count won't update until timmer update: ${count}`;
        }}
      </div>
      
      <button
        onClick={() => {
          count++;
        }}
      >
        plus
      </button>
    </p>
  );
};

const Component = () => {
  const items = createState<{ id: number; count: number }[]>([]);
  return (
    <div>
      <button
        onClick={() => {
          items.val = [
            ...(items.val ?? []),
            { id: items.val.length, count: 0 },
          ];
        }}
      >
        add new item
      </button>
      <div>{() => `add item ${items.val.length}`}</div>
      <div>
        {() =>
          items.val.map((item) => {
            return <TodoItem name={item.id} key={item.id} count={item.count} />;
          })
        }
      </div>
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
