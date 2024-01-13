import {
  createProxy,
  createRef,
  derive,
  upsert,
  useEffect,
} from "@idealjs/sapling";
import { JSX } from "@idealjs/sapling/jsx-runtime";

const counter = createProxy({ val: 0 });

const Test = () => {
  const value = counter.val + 1;
  return (
    <div>
      {() => counter.val + value}
      <button
        onClick={() => {
          counter.val++;
        }}
      >
        plus
      </button>
    </div>
  );
};

const Component = () => {
  const Comp = createRef<() => JSX.Element>();
  useEffect(() => {
    console.log("test test panel useEffect");
    Comp.current = Test;
  });
  return (
    <div>
      {() => {
        console.log("test test Comp.current");
        return Comp.current ? <Comp.current /> : null;
      }}
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
