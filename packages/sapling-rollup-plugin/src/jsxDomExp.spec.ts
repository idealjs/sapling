import { babel } from "@rollup/plugin-babel";
import path from "path";
import { rolldown } from "rolldown";
import { rollup } from "rollup";
import esbuild from "rollup-plugin-esbuild";
import { describe, expect, it, vi } from "vitest";

import saplingRolldownPlugin from ".";

describe("test", () => {
  it("convert component", async (t) => {
    const bundle = await rollup({
      input: path.resolve(__dirname, "./fixtures/Test.tsx"),
      plugins: [
        babel({
          babelHelpers: "bundled",
          extensions: [".js", ".jsx", ".ts", ".tsx"],
          presets: ["@babel/preset-typescript"],
          plugins: [
            [
              "babel-plugin-jsx-dom-expressions",
              {
                moduleName: "dom-expressions",
              },
            ],
          ],
        }),
      ],
    });

    const res = await bundle.generate({
      format: "esm",
    });
    const [{ code }] = res.output;
    expect(code).toMatchInlineSnapshot(`
      "import { template, effect } from 'dom-expressions';

      var _tmpl$ = /*#__PURE__*/template(\`<div>Test\`);
      const style = values => {
        return "";
      };
      const Test = () => {
        return (() => {
          var _el$ = _tmpl$();
          effect(() => _el$.className = style());
          return _el$;
        })();
      };

      export { Test as default };
      "
    `);
  });
});
