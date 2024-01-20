import { ProxyScope } from "./ProxyScope";
import { Listener, ReactiveScope } from "./ReactiveScope";
import { Dispose } from "./type";

export type CreateProxy = {
  <T extends object>(initialValue: T): T;
  <T = undefined>(): Partial<T>;
};

const notifyFunctions: (string | symbol)[] = [
  // Array
  "push",
  "pop",
  // Set
  "add",
  "delete",
  "clear",
  // Map
  "set",
  "delete",
  "clear",
];

const bindProxyFunctions: (string | symbol)[] = ["reduce", "forEach", "map"];

export const createReactive = () => {
  const reactiveScope = new ReactiveScope();
  const proxyScope = new ProxyScope();

  const notifyChange = (dep: object) => {
    if (
      reactiveScope.hasCircularDep(dep) &&
      import.meta.env?.MODE === "development"
    ) {
      console.error(JSON.stringify(dep));
      throw new Error("reactive hasCircularDep setting");
    }
    const listeners = reactiveScope.getListeners(dep);
    Array.from(listeners ?? []).forEach((listener) => listener());
  };

  const createProxy: CreateProxy = <T extends object>(value: T = {} as T) => {
    const cachedProxy =
      proxyScope.getRawValue(value) != null
        ? value
        : proxyScope.getTrackedProxy(value) ?? null;
    if (cachedProxy) {
      return cachedProxy;
    }
    const proxyValue = new Proxy(value, {
      get(target, p, receiver) {
        let value = Reflect.get(target, p) as
          | unknown
          | ((...parans: unknown[]) => unknown);

        if (typeof value === "function") {
          const _value = new Proxy(value, {
            apply(target, thisArg, argArray) {
              const res = Reflect.apply(target, thisArg, argArray);
              if (notifyFunctions.includes(p)) {
                notifyChange(receiver);
              }
              return res;
            },
          });
          if (bindProxyFunctions.includes(p)) {
            value = _value.bind(receiver);
          } else {
            value = _value.bind(target);
          }
        }
        if (
          value != null &&
          typeof value === "object" &&
          proxyScope.getRawValue(value) == null
        ) {
          value = createProxy(value);
          Reflect.set(target, p, value);
        }
        reactiveScope.addDep(receiver);
        return value;
      },
      set(target, p, newValue, receiver) {
        const prevValue = Reflect.get(target, p);
        if (Object.is(prevValue, newValue)) {
          return true;
        }
        let nextValue = newValue;

        if (typeof newValue === "object" && newValue != null) {
          nextValue = createProxy(newValue);
        }

        Reflect.set(target, p, nextValue);
        notifyChange(receiver);
        return true;
      },
    });
    proxyScope.trackProxy(proxyValue, value);
    return proxyValue;
  };

  const subscribe = <T extends object>(val: T, callback: Listener) => {
    const removeListener = reactiveScope.addListener(val, callback);
    const unsubscribe = () => {
      removeListener();
    };
    return unsubscribe;
  };

  const effect = (callback: () => Dispose | void) => {
    let unsubscribe: () => void;
    let dispose: Dispose | void;
    const deps = new Set<object>();
    const state = {
      val: () => {
        dispose?.();
        unsubscribe();
        deps.clear();
      },
    };
    const next = () => {
      deps.clear();
      unsubscribe?.();
      const resume = reactiveScope.collectDeps(deps);
      dispose = callback();
      unsubscribe = reactiveScope.subscribeDeps(next);
      resume();
    };
    next();
    return state as { readonly val: Dispose | void };
  };

  return {
    notifyChange,
    reactiveScope,
    proxyScope,
    createProxy,
    subscribe,
    effect,
  };
};

export const {
  notifyChange,
  reactiveScope,
  proxyScope,
  createProxy,
  subscribe,
  effect,
} = createReactive();
