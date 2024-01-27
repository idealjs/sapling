import { createRoot } from "@idealjs/sapling";

const Test = () => {
  return [<div>A</div>, <div>B</div>];
};

const App = () => {
  return (
    <div>
      <div>{() => <Test />}</div>
      <div>{() => [<Test />, <Test />]}</div>
      <Test />
      {() => [<Test />, <Test />]}
    </div>
  );
};

createRoot(document.getElementById("app")!).render(<App />);
