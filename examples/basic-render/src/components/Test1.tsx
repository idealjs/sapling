import { createProxy, useEffect } from "@idealjs/sapling";

const Test1 = () => {
  const state = createProxy({ count: 0 });
  useEffect(() => {
    const handler = setInterval(() => {
      state.count++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  return (
    <div>
      {() => {
        return state.count;
      }}
    </div>
  );
};

export default Test1;
