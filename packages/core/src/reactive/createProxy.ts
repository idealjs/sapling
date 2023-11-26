export type CreateProxy = {
  <T extends object>(initialValue: T): T;
  <T = undefined>(): Partial<T>;
};

export const createProxyUtil =
  (onSet: (value: unknown) => void): CreateProxy =>
  <T extends object>(val: T = {} as T) => {
    const _val = val as T;
    return new Proxy(_val, {
      get(target, prop, receiver) {
        return Reflect.get(target, prop, receiver);
      },
      set(
        target: T,
        prop: string | symbol,
        newValue: unknown,
        receiver: object,
      ) {
        const hasPrevValue = Reflect.has(target, prop);
        const prevValue = Reflect.get(target, prop, receiver);
        if (hasPrevValue && Object.is(prevValue, newValue)) {
          return true;
        }
        let nextValue: unknown = newValue;
        if (newValue != null && typeof newValue === "object") {
          nextValue = createProxyUtil(onSet)(newValue);
        }
        Reflect.set(target, prop, nextValue, receiver);
        onSet(nextValue);
        return true;
      },
    });
  };
