export type PathSegment = string | number | symbol;

export type PathKey = string;

export type Selector<T extends object, R> = (state: T) => R;

export type Listener<R> = (value: R) => void;

export interface Subscription<T extends object, R> {
  selector: Selector<T, R>;
  listener: Listener<R>;
  dependencies: Set<PathKey>;
  snapshot: R;
}

export interface TrackingState {
  paths: Set<PathKey>;
}
