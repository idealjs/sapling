import path from "path";
import { rollup } from "rollup";
import esbuild from "rollup-plugin-esbuild";
import { describe, expect, it, vi } from "vitest";

import saplingRolldownPlugin from ".";

describe("test", () => {
  it("convert component", async (t) => {
    const bundle = await rollup({
      input: path.resolve(__dirname, "./fixtures/Test.tsx"),
      plugins: [esbuild(), saplingRolldownPlugin()],
      external: ["react/jsx-runtime"],
    });

    const res = await bundle.generate({
      format: "esm",
    });
    const [{ code }] = res.output;
    expect(code).toMatchInlineSnapshot(`
      "import { jsx } from 'react/jsx-runtime';

      const style = values => {
        return "";
      };
      const Test = () => {
        return /* @__PURE__ */jsx("div", {
          className: style(),
          children: "Test"
        });
      };

      export { Test as default };
      "
    `);
  });
});
