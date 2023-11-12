import type { Observable, State, StateView } from "./state";
import { createObservable, createSubscriber } from "./state";
export type Dispose = () => void;

export type CreateState = {
  <T>(initialValue: T): State<T>;
  <T>(initialValue: T | null): StateView<T | null>;
  <T = undefined>(): State<T | undefined>;
};

export type Scope = {
  createState: CreateState;
  derive: <T>(callback: () => T) => State<T>;
  effect: (callback: () => Dispose | void) => State<Dispose>;
  addDeps: (dep: Observable) => void;
  deleteDeps: (dep: Observable) => void;
  collectDeps: <V>(deps: Set<Observable>, callback: () => V) => V;
};

export function createScope() {
  let _deps: Set<Observable> | null = null;

  const addDeps = (dep: Observable) => {
    _deps?.add(dep);
  };

  const deleteDeps = (dep: Observable) => {
    _deps?.delete(dep);
  };
  const collectDeps = <V>(deps: Set<Observable>, callback: () => V) => {
    const temp = _deps;
    _deps = deps;
    const val = callback();
    _deps = temp;
    return val;
  };

  const scope: Scope = {
    addDeps,
    deleteDeps,
    collectDeps,
    createState: <T>(val?: T) => createObservable(scope, val),
    derive: <T>(callback: () => T) => {
      let deps: Observable[] = [];
      const subscriber = createSubscriber(scope, () => {
        deps.forEach((dep) => {
          dep.deleteSubscriber(subscriber);
        });
        const result = callback();
        deps = Array.from(_deps ?? []);
        return result;
      });
      subscriber.next();
      return subscriber as State<T>;
    },
    effect: (callback: () => Dispose | void) => {
      let deps: Observable[] = [];
      const subscriber = createSubscriber(scope, () => {
        deps.forEach((dep) => {
          dep.deleteSubscriber(subscriber);
        });
        const dispose = callback();
        deps = Array.from(_deps ?? []);
        return () => {
          deps.forEach((deps) => {
            deps.deleteSubscriber(subscriber);
          });
          dispose?.();
        };
      });
      subscriber.next();
      return subscriber as State<Dispose>;
    },
  };

  return scope;
}
