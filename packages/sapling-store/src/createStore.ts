import { createProxy, type ProxyOperation } from "./createProxy";

type Listener = (paths: string[], operation: ProxyOperation) => void;
type Unsubscribe = () => void;

export const createStore = <T extends object>(defaultValue: T) => {
  const listeners = new Set<Listener>();
  const proxy = createProxy(defaultValue, {
    notifySet(paths, operation) {
      listeners.forEach((listener) => {
        listener(paths, operation);
      });
    },
  });

  const subscribe = (listener: Listener): Unsubscribe => {
    listeners.add(listener);
    return () => {
      listeners.delete(listener);
    };
  };

  return {
    proxy,
    subscribe,
  };
};
