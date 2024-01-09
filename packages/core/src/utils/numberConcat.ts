const numberConcat = (a: number, b: number) =>
  (a << (Math.ceil(Math.log2(b)) + 1)) + b;

export default numberConcat;
