import { isObjectLike } from "./isObjectLike";
import { isTrackableProp } from "./isTrackableProp";
import { segmentToKey } from "./segmentToKey";
import type { PathKey } from "./types";

export type ProxyOperation = "get" | "set" | "delete";

type CreateProxyContext = {
  notifySet?: (paths: PathKey[], operation: ProxyOperation) => void;
  basePathPrefix?: PathKey[];
  proxyCache?: WeakMap<object, object>;
};

export function createProxy<TValue extends object>(
  original: TValue,
  context: CreateProxyContext = {},
): TValue {
  const {
    notifySet,
    basePathPrefix = [],
    proxyCache = new WeakMap(),
  } = context;
  let initializing = true;
  if (proxyCache.has(original)) {
    return proxyCache.get(original) as TValue;
  }

  const proxy = new Proxy(Object.create(Object.getPrototypeOf(original)), {
    set: (target, prop, value, receiver) => {
      const prevValue = Reflect.get(target, prop, receiver);
      if (
        !initializing &&
        (Object.is(prevValue, value) ||
          Object.is(prevValue, proxyCache.get(value)))
      ) {
        return true;
      }

      const nextValue =
        !proxyCache.has(value) && isObjectLike(value)
          ? createProxy(value, {
              notifySet,
              basePathPrefix: isTrackableProp(prop)
                ? [...basePathPrefix, segmentToKey(prop)]
                : basePathPrefix,
              proxyCache,
            })
          : value;

      const result = Reflect.set(target, prop, nextValue, receiver);
      Reflect.set(original, prop, value);
      if (!initializing) {
        notifySet?.([...basePathPrefix, segmentToKey(prop)], "set");
      }

      return result;
    },
    deleteProperty: (nextTarget, prop) => {
      const result = Reflect.deleteProperty(nextTarget, prop);
      notifySet?.([...basePathPrefix, segmentToKey(prop)], "delete");
      return result;
    },
  });
  proxyCache.set(original, proxy);

  Reflect.ownKeys(original).forEach((key) => {
    const desc = Reflect.getOwnPropertyDescriptor(original, key);
    if (desc?.value != null && desc.writable) {
      proxy[key as keyof TValue] = original[key as keyof TValue];
    }
  });

  initializing = false;
  return proxy;
}
