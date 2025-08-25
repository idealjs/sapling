import { parse } from "@babel/parser";
import traverse from "@babel/traverse";
import { transform } from "@idealjs/sapling-transformer-nodejs";
import { Plugin } from "rollup";

const virtualExtractFilter = /\.virtual\.file\?source=.*$/;

const saplingRollupPlugin = (): Plugin => {
  return {
    name: "sapling-rollup-plugin",
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
    async transform(code, id) {
      const result = transform(code);

      return result;
    },
  };
};

export default saplingRollupPlugin;
