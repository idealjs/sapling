import { transformSync } from "@babel/core";
import { parse } from "@babel/parser";
import traverse from "@babel/traverse";
import { Plugin } from "rollup";

const virtualExtractFilter = /\.virtual\.file\?source=.*$/;

const saplingRolldownPlugin = (): Plugin => {
  return {
    name: "sapling-rolldown-plugin",
    resolveId(id) {
      if (!virtualExtractFilter.test(id)) {
        return;
      }

      return null; // other ids should be handled as usually
    },
    load(id) {
      if (!virtualExtractFilter.test(id)) {
        return null;
      }
      if (id === "virtual-module") {
        // the source code for "virtual-module"
        return 'export default "This is virtual!"';
      }
      return null; // other ids should be handled as usually
    },
    transform(code, id) {
      const babelFileResult = transformSync(code, {
        presets: ["@babel/preset-typescript", "@babel/preset-react"],
        filename: id,
        ast: true,
      });
      console.log("test test", babelFileResult);

      return babelFileResult?.code;
    },
  };
};

export default saplingRolldownPlugin;
