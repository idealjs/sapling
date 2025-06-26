"use client";
import { a } from "./A.ts";

const style = (values: unknown): string => {
  return "";
};

const Test = () => {
  return <div className={style({})}>{"Test"}</div>;
};

export default Test;
