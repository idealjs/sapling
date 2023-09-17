import { RuleTester } from "eslint";
import { noNestedDeriveCall } from "src/noNestedDeriveCall";
import { noStateCreatedInDerive } from "src/noStateCreatedInDerive";
import { describe, it } from "vitest";

const tester = new RuleTester({
  parser: require.resolve("@typescript-eslint/parser"),
});

describe("", () => {
  it("nested derive test", () => {
    tester.run("noNestedDeriveCall", noNestedDeriveCall, {
      valid: [
        {
          code: `
          derive(()=>{
          })`,
        },
      ],
      invalid: [
        {
          code: `
          derive(()=>{
            derive(()=>{
            })
          })`,
          errors: ["derive call in the derive's callback."],
        },
        {
          code: `
          derive(()=>{
            derive(()=>{
              derive(()=>{
              })
            })
          })`,
          errors: [
            "derive call in the derive's callback.",
            "derive call in the derive's callback.",
          ],
        },
      ],
    });
  });
  it("createState in derive test", () => {
    tester.run("noStateCreatedInDerive", noStateCreatedInDerive, {
      valid: [
        {
          code: `
          const state = createState();
          derive(()=>{
            console.log(state.val);
          })`,
        },
      ],
      invalid: [
        {
          code: `
          derive(()=>{
            createState();
          })`,
          errors: ["state created in derive."],
        },
        {
          code: `
          derive(()=>{
            const state = createState();
            console.log(state.val);
          })`,
          errors: ["state created in derive."],
        },
        {
          code: `
          derive(()=>{
            const state = createState();
            createState();
          })`,
          errors: ["state created in derive.", "state created in derive."],
        },
      ],
    });
  });
});
