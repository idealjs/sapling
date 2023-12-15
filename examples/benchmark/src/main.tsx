import { upsert } from "@idealjs/reactive";

import Loading from "./components/Loading";

const Component = () => {
  return (
    <div>
      {Array(1000)
        .fill(0)
        .map((v) => {
          return <Loading />;
        })}
    </div>
  );
};

const root = document.getElementById("app")!;

upsert(root, <Component />);
