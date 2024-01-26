import { SaplingNode, createProxy, useEffect } from "@idealjs/sapling";

interface IProps {
  children?: SaplingNode;
}

const Test2 = (props: IProps) => {
  const { children } = props;
  const state = createProxy({ count: 0 });
  useEffect(() => {
    const handler = setInterval(() => {
      state.count++;
    }, 1000);

    return () => {
      clearInterval(handler);
    };
  });

  useEffect(() => {
    console.log(state.count);
  });

  return (
    <div>
      {() => {
        return [state.count, "children:", children ?? null];
      }}
    </div>
  );
};

export default Test2;
