import { performance } from "perf_hooks";

export const N_SUBSCRIBERS = 1000;
export const N_UPDATES = 1000;
export const ARRAY_SIZE = 1000;
export const RUNS = 10;

export function makeArrayState() {
  return {
    items: Array.from(
      { length: ARRAY_SIZE },
      (_, i) => ({ id: i, value: i }),
    ),
  };
}

export function time(fn: () => void) {
  const start = performance.now();
  fn();
  const end = performance.now();
  return end - start;
}

export interface BenchResult {
  subscribeTime: number;
  updateTime: number;
}

export function removeOutliers(values: number[]): number[] {
  if (values.length <= 2) {
    return values;
  }

  const sorted = [...values].sort((a, b) => a - b);
  // Remove min and max to discard outliers
  return sorted.slice(1, -1);
}

export function computeStats(values: number[]): {
  mean: number;
  median: number;
  min: number;
  max: number;
} {
  if (values.length === 0) {
    return { mean: 0, median: 0, min: 0, max: 0 };
  }

  const mean = values.reduce((a, b) => a + b, 0) / values.length;
  const sorted = [...values].sort((a, b) => a - b);
  const median =
    sorted.length % 2 === 0
      ? (sorted[sorted.length / 2 - 1] + sorted[sorted.length / 2]) / 2
      : sorted[Math.floor(sorted.length / 2)];

  return {
    mean,
    median,
    min: sorted[0],
    max: sorted[sorted.length - 1],
  };
}
