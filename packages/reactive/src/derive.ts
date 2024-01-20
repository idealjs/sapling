import { createProxy, effect } from "./createReactive";
import { Dispose } from "./type";

export const derive = <T>(callback: () => T) => {
  const state = createProxy<{ val: T; dispose: void | Dispose }>();
  const dispose = effect(() => {
    state.val = callback();
  });
  state.dispose = dispose.val;
  return state as { readonly val: T };
};
