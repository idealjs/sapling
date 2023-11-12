import { createScope } from "./scope";

export const { createState, derive, effect } = createScope();
export type { State, StateView } from "./state";
