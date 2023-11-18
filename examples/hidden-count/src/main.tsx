import { createState, upsert, useEffect } from "@idealjs/reactive";

const count = createState(0);

const Hello = () => {
  const hidden = createState(false);

  return (
    <div>
      <button
        type="button"
        onClick={() => {
          count.val++;
        }}
      >
        {() => count.val}
      </button>
      <button
        onClick={() => {
          hidden.val = !hidden.val;
        }}
      >
        {() => hidden.val}
      </button>
      <button
        onClick={() => {
          hidden.val = false;
        }}
      >
        {() => hidden.val}
      </button>
      <div>
        {() => {
          console.log("test test");
          return hidden.val ? null : <Counter />;
        }}
      </div>
    </div>
  );
};

const Counter = () => {
  useEffect(() => {
    console.log("test test Counter", count.val);
  });
  return <div>{() => count.val}</div>;
};
const root = document.getElementById("app")!;

upsert(root, <Hello />);
