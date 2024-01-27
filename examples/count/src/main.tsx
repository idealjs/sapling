import { createProxy, createRoot, useEffect } from "@idealjs/sapling";

const App = () => {
  const count = createProxy({ val: 0 });
  const hidden = createProxy({ val: false });

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

createRoot(document.getElementById("app")!).render(<App />);
