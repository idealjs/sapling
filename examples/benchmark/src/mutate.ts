function Mutate<
  Args extends unknown[],
  R
>(
  original: (...args: Args) => R,
  context: ClassMethodDecoratorContext<unknown, (...args: Args) => R>,
) {
  // 返回一个包装函数，保留 this 和参数，并在调用前后可以插入副作用逻辑
  // 这里示例：在调用前打印方法名（如果可用），然后调用原始函数并返回结果
  return function (this: unknown, ...args: Args): R {
    // 尽量安全地获取方法名（某些 context.name 可能是 symbol）
    let name: string;
    try {
      name = typeof context.name === "symbol" ? context.name.toString() : String(context.name);
    } catch {
      name = "<unknown>";
    }

    // 示例副作用：简单的调试输出（不会影响逻辑）
    // console.debug(`Mutate: calling ${name} with`, args);

    // 调用原始方法并返回结果
    return original.apply(this, args);
  };
}

export default Mutate;
