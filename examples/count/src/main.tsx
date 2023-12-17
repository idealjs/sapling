import { createState, upsert, useEffect } from "@idealjs/sapling";

const Hello = () => {
  const count = createState(0);
  const hidden = createState(false);

  useEffect(() => {
    console.log("test test");
    if (!hidden.val) {
      console.log("test test count", count.val);
    }
  });

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
    </div>
  );
};
const root = document.getElementById("app")!;

upsert(root, <Hello />);
