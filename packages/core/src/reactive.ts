const fStack: { stateReader: () => unknown; state: State<unknown> }[] = [];

export class State<T> implements State<T> {
  private _val: T;
  private _oldVal: T | undefined;
  private _listeners = new Set<{
    stateReader: () => unknown;
    state: State<unknown>;
  }>();
  constructor(val: T) {
    this._val = val;
  }

  get val() {
    fStack[fStack.length - 1] && this._listeners.add(fStack[fStack.length - 1]);
    return this._val;
  }

  get oldVal() {
    fStack[fStack.length - 1] && this._listeners.add(fStack[fStack.length - 1]);
    return this._oldVal;
  }

  set val(v: T) {
    this._oldVal = this._val;
    this._val = v;
    const listeners = Array.from(this._listeners);
    this._listeners.clear();
    listeners.forEach(({ stateReader, state }) => {
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
  fStack.push({ stateReader: f, state });
  state.val = f();
  fStack.pop();
  return state as StateView<T>;
};

export type Dispose = () => void;

export const effect = <R = Dispose | undefined>(
  f: () => R,
  state = createState<R>(),
) => {
  return derive(f, state);
};
