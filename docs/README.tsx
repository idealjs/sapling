function Derive<T>(
  getter: () => T,
  context: ClassGetterDecoratorContext<unknown, T>,
) {
  return () => getter();
}

function State<T>(
  value: ClassAccessorDecoratorTarget<unknown, T>,
  context: ClassAccessorDecoratorContext<unknown, T>,
) {
  return {
    get() {
      return value.get();
    },
    set(v: T) {
      value.set(v);
    },
  };
}

const batch = (func: () => void) => {};

const effect = (func: () => void) => {};

const getData = async (search?: string): Promise<number> => {
  return 0;
};

class Example {
  constructor() {
    effect(() => {
      getData(this.inputValue).then((value) => {
        this.value = value;
      });
    });
  }
  @State accessor inputValue: string = "";
  @State accessor value: number = 0;

  @Derive get doubleValue() {
    return this.value * 2;
  }
  async initData() {
    this.value = await getData();
  }

  public render() {
    return (
      <div
        onClick={() => {
          batch(() => {
            this.value++;
            this.value++;
            this.value++;
          });
        }}
      >
        {this.value}
      </div>
    );
  }
}
