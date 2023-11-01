const fStack: (() => unknown)[] = [];
const stateMap = new WeakMap<() => unknown, State<unknown>>();

export class State<T> implements State<T> {
  private _val: T;
  private _oldVal: T | undefined;
  private _listeners = new Set<() => unknown>();
  private _localStateMap = new WeakMap<
    () => unknown,
    State<unknown> | undefined
  >();
  constructor(val: T) {
    this._val = val;
  }

  private collectDeps() {
    const f = fStack[fStack.length - 1];
    f && this._listeners.add(f);
    f && this._localStateMap.set(f, stateMap.get(f));
  }

  get val() {
    this.collectDeps();
    return this._val;
  }

  get oldVal() {
    this.collectDeps();
    return this._oldVal;
  }

  set val(v: T) {
    this._oldVal = this._val;
    this._val = v;
    const listeners = Array.from(this._listeners);
    this._listeners.clear();
    listeners.forEach((stateReader) => {
      const state = this._localStateMap.get(stateReader);
      this._localStateMap.delete(stateReader);
      derive(stateReader, state);
    });
  }
}

export interface StateView<T> extends State<T> {
  readonly val: T;
  readonly oldVal: T | undefined;
}

export function createState<T>(initialValue: T): State<T>;

export function createState<T>(initialValue: T | null): StateView<T | null>;

export function createState<T = undefined>(): State<T | undefined>;

export function createState<T>(val?: T) {
  return new State(val);
}

export const derive = <T>(
  f: () => T,
  state = createState<T>(),
): StateView<T> => {
  fStack.push(f);
  stateMap.set(f, state);
  state.val = f();
  fStack.pop();
  stateMap.delete(f);
  return state as StateView<T>;
};

export type Dispose = () => void;

export const effect = <R = Dispose | undefined>(
  f: () => R,
  state = createState<R>(),
) => {
  return derive(f, state);
};
