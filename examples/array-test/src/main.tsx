import { upsert } from "@idealjs/sapling";

const Test = () => {
  return [<div>A</div>, <div>B</div>];
};

const Component = () => {
  return (
    <div>
      <div>{() => <Test />}</div>
      <div>{() => [<Test />, <Test />]}</div>
      <Test />
      {() => [<Test />, <Test />]}
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
