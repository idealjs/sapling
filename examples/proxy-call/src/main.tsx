import { createState, upsert } from "@idealjs/sapling";

const Component = () => {
  const ref = createState<HTMLDivElement>(null);
  return (
    <div>
      <button
        onClick={() => {
          if (ref.val) {
            ref.val.click();
            ref.val.innerHTML = "hello";
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

const root = document.getElementById("app")!;

upsert(root, <Component />);
