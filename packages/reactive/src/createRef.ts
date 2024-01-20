import { notifyChange, reactiveScope } from "./createReactive";

export type Ref<T = unknown> = {
  current: T;
};

export interface RefView<T> extends Ref<T> {
  readonly current: T;
}

export type CreateRef = {
  <T>(initialValue: T): Ref<T>;
  <T>(initialValue: T | null): RefView<T | null>;
  <T = unknown>(): Ref<T | undefined>;
};

export const createRef: CreateRef = (value?: unknown) => {
  let _val = value;
  const state = {
    get current() {
      reactiveScope.addDep(state);
      return _val;
    },
    set current(v) {
      _val = v;
      notifyChange(state);
    },
  };

  return state;
};
