import { vi } from "vitest";

export const sleep = (milliseconds: number) => {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
    vi.advanceTimersToNextTimer();
  });
};
