import type { Scope } from "./scope";

export type State<T = unknown> = {
  val: T;
};

export interface StateView<T> extends State<T> {
  readonly val: T;
}

export type Subscriber<T = unknown> = {
  readonly val: T | undefined;
  next: () => void;
};

export type Observable<T = unknown> = {
  val: T;
  addSubscriber: (subscriber: Subscriber) => void;
  deleteSubscriber: (subscriber: Subscriber) => void;
};

export function createObservable<T>(scope: Scope): Observable<T | undefined>;
export function createObservable<T>(scope: Scope, val: T): Observable<T>;
export function createObservable<T>(scope: Scope, val?: T): Observable<T> {
  let _val: T = val as T;
  const _subscribers = new Set<Subscriber>();

  const noticeSubscribers = () => {
    const subscribers = Array.from(_subscribers);
    _subscribers.clear();
    subscribers.forEach((subscriber) => {
      subscriber.next();
    });
  };

  const addSubscriber = (subscriber: Subscriber) => {
    _subscribers.add(subscriber);
  };

  const deleteSubscriber = (subscriber: Subscriber) => {
    _subscribers.delete(subscriber);
  };

  const observable = {
    get val() {
      scope.addDeps(observable);
      return _val;
    },
    set val(v: T) {
      if (v == _val) {
        return;
      }
      _val = v;
      noticeSubscribers();
    },
    addSubscriber,
    deleteSubscriber,
  };

  return observable;
}

export function createSubscriber<T>(
  scope: Scope,
  callback: () => T,
): Subscriber<T> {
  const subscriber = Object.assign(createObservable<T>(scope), {
    next: () => {
      const deps = new Set<Observable>();
      subscriber.val = scope.collectDeps(deps, () => {
        const val = callback();
        deps.forEach((dep) => {
          dep.addSubscriber(subscriber);
        });
        return val;
      });
    },
  });
  return subscriber;
}
