import { createProxy, createRoot, useEffect } from "@idealjs/sapling";

const count = createProxy({ val: 0 });

const App = () => {
  const hidden = createProxy({ val: false });

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

createRoot(document.getElementById("app")!).render(<App />);
