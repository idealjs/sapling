export function isObjectLike(value: unknown): value is object {
  return typeof value === "object" && value !== null;
}
