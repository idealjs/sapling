import { useCallback, useSyncExternalStore } from "react";
import type { Store } from "./createUseStore";

type Selector<T extends object, R = unknown> = (value: T) => R;

const useSelector = <TValue extends object, TSelected = unknown>(
  store: Store<TValue>,
  selector: Selector<TValue, TSelected>,
): TSelected => {
  const subscribe = useCallback(
    (onStoreChange: () => void): (() => void) => {
      const unSub = store.subscribe(() => {
        onStoreChange();
      });
      return () => {
        unSub();
      };
    },
    [store],
  );

  const getSnapshot = useCallback((): TSelected => {
    return selector(store.originalValue);
  }, [selector, store.originalValue]);

  return useSyncExternalStore<TSelected>(subscribe, getSnapshot);
};

export default useSelector;
