export type CreateProxy = {
  <T extends object>(initialValue: T): T;
  <T = undefined>(): Partial<T>;
};

export const createProxyUtil =
  (onSet: (value: unknown) => void): CreateProxy =>
  <T extends object>(val: T = {} as T) => {
    const _val = val as T;
    return new Proxy(_val, {
      get(target, prop) {
        const value = Reflect.get(target, prop);
        if (typeof value === "function") {
          return value.bind(target);
        }
        return value;
      },
      set(target: T, prop: string | symbol, newValue: unknown, receiver) {
        const prevValue = Reflect.get(target, prop);
        if (Object.is(prevValue, newValue)) {
          return true;
        }
        let nextValue: unknown = newValue;
        if (newValue != null && typeof newValue === "object") {
          nextValue = createProxyUtil(onSet)(newValue);
        }
        Reflect.set(target, prop, nextValue);
        onSet(nextValue);
        return true;
      },
    });
  };
