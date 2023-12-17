import { createState, upsert } from "@idealjs/sapling";

const items = createState<{ id: number; hidden: boolean }[]>([]);

const updateList = () => {
  const values = new Array(10).fill("").map((v, index) => {
    return {
      id: index,
      hidden: (Math.random() * 10) % 2 > 1,
    };
  });
  console.log("test test", (Math.random() * 10) % 2, values);
  items.val = values;
};

const Component = () => {
  const ref = createState<HTMLDivElement>(null);
  return (
    <div>
      <button
        onClick={updateList}
        style={() => ({
          color: "blue",
        })}
      >
        update list
      </button>
      <div>
        {() =>
          items.val.map((item) => {
            return item.hidden ? null : <div ref={ref}>{item.id}</div>;
          })
        }
      </div>

      <div>
        {() => {
          return [<div></div>, <div></div>];
        }}
      </div>
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
