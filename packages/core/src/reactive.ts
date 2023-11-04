class State<T> {
  private _val: T;
  private _oldVal: T | undefined;
  private _listeners = new Set<() => unknown>();
  private _localStateMap = new WeakMap<
    () => unknown,
    State<unknown> | undefined
  >();
  private r: Reactive;
  static getDerive = <T>(r: Reactive) => {
    return (f: () => T, state: State<T | undefined>): StateView<T> => {
      r.fStack.push(f);
      r.stateMap.set(f, state);
      state.val = f();
      r.fStack.pop();
      r.stateMap.delete(f);
      return state as StateView<T>;
    };
  };
  constructor(r: Reactive, val: T) {
    this._val = val;
    this.r = r;
  }

  private collectDeps() {
    const f = this.r.fStack[this.r.fStack.length - 1];
    f && this._listeners.add(f);
    f && this._localStateMap.set(f, this.r.stateMap.get(f));
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
      State.getDerive(this.r)(
        stateReader,
        state ?? new State(this.r, this._val),
      );
    });
  }
}

type CreateState = {
  <T>(initialValue: T): State<T>;
  <T>(initialValue: T | null): StateView<T | null>;
  <T = undefined>(): State<T | undefined>;
};

class Reactive {
  public fStack: (() => unknown)[] = [];
  public stateMap = new WeakMap<() => unknown, State<unknown>>();

  public createState: CreateState = <T>(val?: T) => {
    return new State(this, val);
  };

  public derive = <T>(f: () => T, state = createState<T>()) =>
    State.getDerive<T>(this)(f, state);

  public effect = <R = Dispose | undefined>(f: () => R) => {
    return derive(f);
  };
}

export type { State };

export interface StateView<T> extends State<T> {
  readonly val: T;
  readonly oldVal: T | undefined;
}

export type Dispose = () => void;

export const { createState, effect, derive } = new Reactive();
