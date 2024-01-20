import { createProxy, proxyScope } from "./createReactive";

export type Payload<T> = T | ((prev: T) => T);

export const isSetterFunction = <T>(
  payload: Payload<T>,
): payload is (prev: T) => T => {
  if (typeof payload === "function") {
    return true;
  }
  return false;
};

export type Getter<T> = () => T;
export type Setter<T> = (v: Payload<T>) => void;

export type CreateSignal = {
  <T>(initialValue: T): [Getter<T>, Setter<T>];
  <T = unknown>(): [Getter<T | undefined>, Setter<T | undefined>];
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
