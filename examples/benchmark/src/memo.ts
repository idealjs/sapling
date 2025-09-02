const memo = <T>(fn: () => T, dep: () => unknown): T => {};

export default memo;
