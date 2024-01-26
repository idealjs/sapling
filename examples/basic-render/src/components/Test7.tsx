import { createProxy, useEffect } from "@idealjs/sapling";

const Test6 = () => {
  const state1 = createProxy({ count: 0 });
  const state2 = createProxy({ count: 0 });
  const state3 = createProxy({ count: 0 });
  const state4 = createProxy({ count: 0 });

  useEffect(() => {
    const handler = setInterval(() => {
      state1.count++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  useEffect(() => {
    const handler = setInterval(() => {
      state2.count++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  useEffect(() => {
    const handler = setInterval(() => {
      state3.count++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  useEffect(() => {
    const handler = setInterval(() => {
      state4.count++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  return (
    <div>
      <div>hello1</div>
      {() => {
        return state1.count;
      }}
      <div>hello2</div>
      {() => {
        return state2.count;
      }}
      <div>hello3</div>
      {() => {
        return state3.count;
      }}
      <div>hello4</div>
      {() => {
        return state4.count;
      }}
      <div>hello5</div>
    </div>
  );
};

export default Test6;
