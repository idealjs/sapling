import type { PathSegment } from "./types";

export function segmentToKey(segment: PathSegment): string {
  if (typeof segment === "symbol") {
    return `symbol:${segment.description ?? segment.toString()}`;
  }

  return String(segment);
}
