class State<T = unknown> {
  private _val: T;
  private _oldVal: T | undefined;
  private localObserverStack = new Set<Observer<unknown>>();
  protected r: Reactive;

  constructor(r: Reactive);
  constructor(r: Reactive, val: T);
  constructor(r: Reactive, val?: T) {
    this._val = val as T;
    this.r = r;
  }

  private collectDeps() {
    const observer = this.r.observerStack[this.r.observerStack.length - 1];
    this.localObserverStack.add(observer);
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
    const observers = Array.from(this.localObserverStack);
    this.localObserverStack.clear();
    observers.forEach((observer) => {
      observer?.next();
    });
  }
}

class Observer<T> extends State<T> {
  private callback: () => T;
  constructor(r: Reactive, callback: () => T) {
    super(r);
    this.callback = callback;
    this.next();
  }
  public next = () => {
    this.r.observerStack.push(this);
    this.val = this.callback();
    this.r.observerStack.pop();
  };
}

type CreateState = {
  <T>(initialValue: T): State<T>;
  <T>(initialValue: T | null): StateView<T | null>;
  <T = undefined>(): State<T | undefined>;
};

class Reactive {
  public observerStack: Observer<unknown>[] = [];
  public createState: CreateState = <T>(val?: T) => {
    return new State(this, val);
  };

  public derive = <T>(f: () => T) => new Observer(this, f);

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
