import { isObjectLike } from "./isObjectLike";
import { isTrackableProp } from "./isTrackableProp";
import { segmentToKey } from "./segmentToKey";
import type { PathKey } from "./types";

type ProxyOperation = "get" | "set" | "delete";

type CreateProxyContext = {
  notify?: (paths: PathKey[], operation: ProxyOperation) => void;
  basePathPrefix?: PathKey[];
  proxyCache?: WeakMap<object, object>;
};

export function createProxy<TValue extends object>(
  target: TValue,
  context: CreateProxyContext = {},
): TValue {
  const { notify, basePathPrefix = [], proxyCache = new WeakMap() } = context;

  if (proxyCache.has(target)) {
    return proxyCache.get(target) as TValue;
  }

  const proxy = new Proxy(target, {
    get: (target, prop, receiver) => {
      const value = Reflect.get(target, prop, receiver);

      notify?.([...basePathPrefix, segmentToKey(prop)], "get");
      if (isObjectLike(value)) {
        const nextPathPrefix = isTrackableProp(prop)
          ? [...basePathPrefix, segmentToKey(prop)]
          : basePathPrefix;

        return createProxy(value, {
          notify,
          basePathPrefix: nextPathPrefix,
          proxyCache,
        });
      }
      return value;
    },
    set: (nextTarget, prop, value, receiver) => {
      const result = Reflect.set(nextTarget, prop, value, receiver);
      if (result && notify && isTrackableProp(prop)) {
        const pathsToNotify = [...basePathPrefix, segmentToKey(prop)];
        notify(pathsToNotify, "set");
      }
      return result;
    },
    deleteProperty: (nextTarget, prop) => {
      const result = Reflect.deleteProperty(nextTarget, prop);
      if (result && notify && isTrackableProp(prop)) {
        const pathsToNotify = [...basePathPrefix, segmentToKey(prop)];
        notify(pathsToNotify, "delete");
      }
      return result;
    },
  });

  proxyCache.set(target, proxy);

  return proxy;
}
