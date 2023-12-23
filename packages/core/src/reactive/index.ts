export type Readonly<T> = T extends object
  ? {
      readonly [K in keyof T]: Readonly<T[K]>;
    }
  : T;

export type Payload<T> = T | ((prev: T) => T);
export type Getter<T> = () => T;
export type Setter<T> = (v: Payload<T>) => void;
export type Listener = () => void;
export type Dispose = () => void;
export type State<T = unknown> = {
  val: T;
};

export interface StateView<T> extends State<T> {
  readonly val: T;
}

export const isSetterFunction = <T>(
  payload: Payload<T>,
): payload is (prev: T) => T => {
  if (typeof payload === "function") {
    return true;
  }
  return false;
};

export type CreateSignal = {
  <T>(initialValue: T): [Getter<T>, Setter<T>];
  <T = unknown>(): [Getter<T | undefined>, Setter<T | undefined>];
};

export type CreateProxy = {
  <T extends object>(initialValue: T): T;
  <T = undefined>(): Partial<T>;
};

export type CreateState = {
  <T>(initialValue: T): State<T>;
  <T>(initialValue: T | null): StateView<T | null>;
  <T = unknown>(): State<T | undefined>;
};

export class ReactiveScope {
  private deps: Set<object> | null = null;
  public collectDeps = (scopeDeps: Set<object>) => {
    const temp = this.deps;
    this.deps = scopeDeps;
    return () => {
      this.deps = temp;
    };
  };
  public addDep = (getter: object) => {
    this.deps?.add(getter);
  };
  public hasCircularDep = (dep: object) => {
    return this.deps?.has(dep);
  };

  private listeners = new WeakMap<object, Set<Listener>>();
  public subscribeDeps = (callback: Listener) => {
    const unsubscribes = Array.from(this.deps ?? []).map((dep) => {
      return this.addListener(dep, callback);
    });
    return () => {
      unsubscribes.forEach((unsubscribe) => unsubscribe());
    };
  };
  public addListener = <Key extends object>(key: Key, callback: Listener) => {
    const listeners = this.listeners.get(key) ?? new Set();
    listeners.add(callback);
    this.listeners.set(key, listeners);
    return () => {
      listeners.delete(callback);
    };
  };
  public getListeners = <Key extends object>(key: Key) => {
    return this.listeners.get(key);
  };
}

export class ProxyScope {
  private trackedProxy = new WeakMap<object, object>();
  private trackedValue = new WeakMap<object, object>();
  public getRawValue = <T extends object>(proxy: T) => {
    return this.trackedValue.get(proxy) as T | undefined;
  };
  public getTrackedProxy = <T extends object>(rawValue: T) => {
    return this.trackedProxy.get(rawValue) as T | undefined;
  };
  public trackProxy = <T extends object>(proxy: T, rawValue: T) => {
    this.trackedValue.set(proxy, rawValue);
    this.trackedProxy.set(rawValue, proxy);
  };
}

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
        let value = Reflect.get(target, p);

        if (typeof value === "function") {
          value = value.bind(target);
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

  const derive = <T>(callback: () => T) => {
    const state = createProxy<{ val: T }>();
    let unsubscribe: () => void;
    const deps = new Set<object>();

    const next = () => {
      deps.clear();
      unsubscribe?.();
      const resume = reactiveScope.collectDeps(deps);
      const val = callback();
      unsubscribe = reactiveScope.subscribeDeps(next);
      resume();
      state.val = val;
    };
    next();
    return state as StateView<T>;
  };

  const effect = (callback: () => Dispose | void) => {
    const state: { val?: () => void } = {};
    let unsubscribe: () => void;
    const deps = new Set<object>();

    const next = () => {
      deps.clear();
      unsubscribe?.();
      const resume = reactiveScope.collectDeps(deps);
      const val = callback();
      unsubscribe = reactiveScope.subscribeDeps(next);
      resume();
      state.val = () => {
        val?.();
        unsubscribe();
        deps.clear();
      };
    };
    next();
    return state as StateView<Dispose | void>;
  };

  return { proxyScope, createProxy, subscribe, derive, effect };
};

export const { proxyScope, createProxy, subscribe, derive, effect } =
  createReactive();

export const createState: CreateState = (value?: unknown) => {
  const proxyValue = createProxy({
    value,
  });
  return {
    get val() {
      return proxyValue.value;
    },
    set val(v) {
      proxyValue.value = v;
    },
  };
};

export const createSignal: CreateSignal = (value?: unknown) => {
  const proxyData = createProxy({
    value,
  });

  const get = () => {
    return proxyData.value;
  };
  const set = (payload: Payload<unknown>) => {
    let nextValue = payload;
    if (isSetterFunction(payload)) {
      nextValue = payload(proxyScope.getRawValue(proxyData)?.value);
    }
    proxyData.value = nextValue;
  };
  return [get, set];
};
