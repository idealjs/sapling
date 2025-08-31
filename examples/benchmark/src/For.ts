class For<T = unknown> {
  constructor(
    props: { each: T[] },
    fallback: unknown,
    children: (item: T) => unknown,
  ) {}
}

export default For;
