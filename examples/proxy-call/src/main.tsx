import { createRef, createRoot } from "@idealjs/sapling";

const App = () => {
  const ref = createRef<HTMLDivElement>(null);
  return (
    <div>
      <button
        onClick={() => {
          if (ref.current) {
            ref.current.click();
            ref.current.innerHTML = "hello";
          }
        }}
      >
        click
      </button>
      <div
        ref={ref}
        onClick={() => {
          console.log("hello");
        }}
      ></div>
    </div>
  );
};

createRoot(document.getElementById("app")!).render(<App />);
