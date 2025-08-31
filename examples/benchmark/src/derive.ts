function Derive<T>(
  getter: () => T,
  context: ClassGetterDecoratorContext<unknown, T>,
) {
  return () => getter();
}

export default Derive;
