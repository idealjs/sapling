export type Listener = () => void;

export class ReactiveScope {
  private deps: Set<object> | null = null;
  public collectDeps = (scopeDeps: Set<object>) => {
    const temp = this.deps;
    this.deps = scopeDeps;
    return () => {
      this.deps = temp;
    };
  };
  public pauseCollectDeps = () => {
    const temp = this.deps;
    this.deps = null;
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
