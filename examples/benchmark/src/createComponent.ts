import memo from "./memo";
import type { ComponentType } from "./types";

export function createComponent<
  C extends ComponentType<P>,
  P = Record<string, unknown | undefined>,
>(Comp: C, props: P) {
  return memo(
    () => {
      let v = new Comp(props);
      return v.render();
    },
    () => {},
  );
}

export default createComponent;
