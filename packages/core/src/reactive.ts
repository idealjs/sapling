const fStack: { stateReader: () => any; state: State<any> }[] = [];

class State<T> {
  private _val: T;
  public _listeners = new Set<{ stateReader: () => any; state: State<any> }>();
  constructor(val: T) {
    this._val = val;
  }
  get val() {
    fStack[0] && this._listeners.add(fStack[0]);
    return this._val;
  }

  set val(v: T) {
    this._val = v;
    const listeners = Array.from(this._listeners);
    this._listeners.clear();
    listeners.forEach(({ stateReader, state }) => {
      derive(stateReader, state);
    });
  }
}

export type StateView<T> = {
  readonly val: T;
};

export function createState<T>(initialValue: T): State<T>;

export function createState<T>(initialValue: T | null): StateView<T>;

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
