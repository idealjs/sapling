import { Primitive } from "../type";

const isPrimitive = (value: unknown): value is Primitive => {
  return typeof value !== "object" && typeof value !== "function";
};

export default isPrimitive;
