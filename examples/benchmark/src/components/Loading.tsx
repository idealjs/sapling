import { createSignal, useEffect } from "@idealjs/reactive";

const Loading = () => {
  const [state, setState] = createSignal(0);

  const rotate = () => {
    setState((p) => p + 1);
    requestAnimationFrame(rotate);
  };
  useEffect(() => {
    rotate();
  });
  return (
    <div
      onClick={() => {
        state();
      }}
      style={() => {
        return {
          display: "flex",
          justifyContent: "center",
          transform: `rotate(${state()}deg)`,
        };
      }}
    >
      hello
    </div>
  );
};

export default Loading;
