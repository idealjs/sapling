export function isTrackableProp(
  prop: PropertyKey,
): prop is string | number | symbol {
  return (
    typeof prop === "string" ||
    typeof prop === "number" ||
    typeof prop === "symbol"
  );
}
