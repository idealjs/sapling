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

export default State;
