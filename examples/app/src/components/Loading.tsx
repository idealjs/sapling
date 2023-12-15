import { useCallback, useEffect, useState } from "react";
const Loading = () => {
  const [state, setState] = useState(0);
  const rotate = useCallback(() => {
    setState((p) => p + 1);
    requestAnimationFrame(rotate);
  }, []);
  useEffect(() => {
    rotate();
  }, [rotate]);
  return (
    <div
      style={{
        transform: `rotate(${state}deg)`,
      }}
    >
      hello
    </div>
  );
};

export default Loading;
