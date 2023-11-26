export type CreateProxy = {
  <T extends object>(initialValue: T): T;
  <T = undefined>(): Partial<T>;
};

export const createScopedProxyUtil = () => {
  const proxyHandlers = new WeakMap<object, Set<() => void>>();
  const proxyMap = new WeakMap<object, object>();
  const notifyChange = (proxy: object) => {
    proxyHandlers.get(proxy)?.forEach((trigger) => {
      trigger();
    });
  };
  const createProxyUtil =
    (onSet: () => void): CreateProxy =>
    <T extends object>(val: T = {} as T) => {
      const _val = val as T;
      const proxy = new Proxy(_val, {
        get(target, prop) {
          if (prop === "__proxy__") {
            return true;
          }
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

          if (prevValue != null && typeof prevValue === "object") {
            const isPrevProxyValue = Reflect.get(prevValue, "__proxy__");
            if (isPrevProxyValue) {
              proxyHandlers.get(prevValue)?.delete(onSet);
            }
          }

          let nextValue = newValue;
          if (newValue != null && typeof newValue === "object") {
            const isProxy = Reflect.get(newValue, "__proxy__");
            const cachedProxy = proxyMap.get(newValue);
            if (!isProxy && cachedProxy != null) {
              nextValue = cachedProxy;
            }

            if (!isProxy && cachedProxy == null) {
              nextValue = createProxyUtil(onSet)(newValue);
              proxyMap.set(newValue, nextValue as object);
            }

            proxyHandlers.get(nextValue as object)?.add(onSet);
          }
          Reflect.set(target, prop, nextValue);
          notifyChange(receiver);
          return true;
        },
      });
      const handlers = new Set<() => void>();
      handlers.add(onSet);
      proxyHandlers.set(proxy, handlers);
      return proxy;
    };

  return createProxyUtil;
};

export const createProxyUtil = createScopedProxyUtil();
