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
