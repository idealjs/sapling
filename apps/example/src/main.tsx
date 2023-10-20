import {
  ComponentChild,
  createState,
  State,
  upsert,
  useEffect,
} from "@idealjs/reactive";

const TodoItem = (props: { name: number }) => {
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

  return (
    <p>
      {() => {
        return { el: `${name} counter ${state.val}` };
      }}
    </p>
  );
};

const map = <T,>(
  values: State<T[]>,
  callbackfn: (value: T) => ComponentChild,
) => {
  return () => {
    const components = values.val.map(callbackfn);
    return (
      <fragment
        dispose={components.flatMap(
          (component) => component.disposeStack ?? [],
        )}
      >
        {components}
      </fragment>
    );
  };
};

const Component = () => {
  const items = createState<number[]>([]);
  return (
    <div>
      <button
        onclick={() => {
          items.val = [...(items.val ?? []), items.val.length];
        }}
      >
        add new item
      </button>
      <div>{() => `add item ${items.val.length}`}</div>
      {map(items, (item) => {
        return <TodoItem name={item} />;
      })}
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
