import { createProxy, type ProxyOperation } from "./createProxy";
import useSelector from "./useSelector";

type Subscribe = (listener: Listener) => Unsubscribe;
type Listener = (paths: string[], operation: ProxyOperation) => void;
type Unsubscribe = () => void;

type Selector<T extends object, R = unknown> = (value: T) => R;

export type Store<T extends object> = {
  originalValue: T;
  proxy: T;
  subscribe: Subscribe;
};

export type UseStore<T extends object> = Store<T> & {
  (): T;
  <TSelected>(selector: Selector<T, TSelected>): TSelected;
};

const createUseStore = <T extends object>(originalValue: T): UseStore<T> => {
  const listeners = new Set<Listener>();
  const proxy = createProxy(originalValue, {
    notifyChange(paths, operation) {
      listeners.forEach((listener) => {
        listener(paths, operation);
      });
    },
  });

  const subscribe: Subscribe = (listener) => {
    listeners.add(listener);
    return () => {
      listeners.delete(listener);
    };
  };

  const store: Store<T> = {
    originalValue,
    proxy,
    subscribe,
  };

  function useStore(): T;
  function useStore<TSelected>(selector: Selector<T, TSelected>): TSelected;
  function useStore<TSelected>(selector?: Selector<T, TSelected>) {
    const resolvedSelector =
      selector ?? ((value: T) => value as unknown as TSelected);
    return useSelector(store, resolvedSelector);
  }

  return Object.assign(useStore, store);
};

export default createUseStore;
