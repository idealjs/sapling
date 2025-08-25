import path from "path";
import { rollup } from "rollup";
import esbuild from "rollup-plugin-esbuild";
import { describe, expect, it, vi } from "vitest";

import saplingRollupPlugin from ".";

describe("test", () => {
  it("convert component", async (t) => {
    const bundle = await rollup({
      input: path.resolve(__dirname, "./fixtures/Test.tsx"),
      plugins: [
        saplingRollupPlugin(),
        esbuild({
          target: "es2022",
        }),
      ],
    });

    const res = await bundle.generate({
      format: "esm",
    });
    const [{ code }] = res.output;
    expect(code).toMatchInlineSnapshot(`
      "import { State } from '@idealjs/sapling';

      var __create = Object.create;
      var __defProp = Object.defineProperty;
      var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
      var __knownSymbol = (name, symbol) => (symbol = Symbol[name]) ? symbol : Symbol.for("Symbol." + name);
      var __typeError = (msg) => {
        throw TypeError(msg);
      };
      var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
      var __decoratorStart = (base) => [, , , __create(null)];
      var __decoratorStrings = ["class", "method", "getter", "setter", "accessor", "field", "value", "get", "set"];
      var __expectFn = (fn) => fn !== void 0 && typeof fn !== "function" ? __typeError("Function expected") : fn;
      var __decoratorContext = (kind, name, done, metadata, fns) => ({ kind: __decoratorStrings[kind], name, metadata, addInitializer: (fn) => done._ ? __typeError("Already initialized") : fns.push(__expectFn(fn || null)) });
      var __decoratorMetadata = (array, target) => __defNormalProp(target, __knownSymbol("metadata"), array[3]);
      var __runInitializers = (array, flags, self, value) => {
        for (var i = 0, fns = array[flags >> 1], n = fns && fns.length; i < n; i++) flags & 1 ? fns[i].call(self) : value = fns[i].call(self, value);
        return value;
      };
      var __decorateElement = (array, flags, name, decorators, target, extra) => {
        var fn, it, done, ctx, access, k = flags & 7, s = false, p = false;
        var j = array.length + 1 , key = __decoratorStrings[k + 5];
        var initializers = (array[j - 1] = []), extraInitializers = array[j] || (array[j] = []);
        var desc = ((target = target.prototype), __getOwnPropDesc({ get [name]() {
          return __privateGet(this, extra);
        }, set [name](x) {
          return __privateSet(this, extra, x);
        } }, name));
        for (var i = decorators.length - 1; i >= 0; i--) {
          ctx = __decoratorContext(k, name, done = {}, array[3], extraInitializers);
          {
            ctx.static = s, ctx.private = p, access = ctx.access = { has: (x) => name in x };
            access.get = (x) => x[name];
            access.set = (x, y) => x[name] = y;
          }
          it = (0, decorators[i])({ get: desc.get, set: desc.set } , ctx), done._ = 1;
          if (it === void 0) __expectFn(it) && (desc[key] = it );
          else if (typeof it !== "object" || it === null) __typeError("Object expected");
          else __expectFn(fn = it.get) && (desc.get = fn), __expectFn(fn = it.set) && (desc.set = fn), __expectFn(fn = it.init) && initializers.unshift(fn);
        }
        return desc && __defProp(target, name, desc), target;
      };
      var __accessCheck = (obj, member, msg) => member.has(obj) || __typeError("Cannot " + msg);
      var __privateGet = (obj, member, getter) => (__accessCheck(obj, member, "read from private field"), member.get(obj));
      var __privateAdd = (obj, member, value) => member.has(obj) ? __typeError("Cannot add the same private member more than once") : member instanceof WeakSet ? member.add(obj) : member.set(obj, value);
      var __privateSet = (obj, member, value, setter) => (__accessCheck(obj, member, "write to private field"), member.set(obj, value), value);
      var _count_dec, _init, _count;
      _count_dec = [State];
      class Test {
        constructor() {
          __privateAdd(this, _count, __runInitializers(_init, 8, this, 0)), __runInitializers(_init, 11, this);
        }
        render() {
          return _$createJsxTagElement(() => {
            let _el$ = _$createElement("div");
            let _el$1 = _$createElement("button");
            _$setProp(_el$1, "onClick", () => {
              this.count++;
            });
            _$insertNode(_el$1, _$createTextNode("+"));
            _$insertNode(_el$, _el$1);
            _$effect(() => {
              _$insert(_el$, this.count);
            });
            let _el$2 = _$createElement("button");
            _$setProp(_el$2, "onClick", () => {
              this.count--;
            });
            _$insertNode(_el$2, _$createTextNode("-"));
            _$insertNode(_el$, _el$2);
            return _el$;
          });
        }
      }
      _init = __decoratorStart();
      _count = new WeakMap();
      __decorateElement(_init, 4, "count", _count_dec, Test, _count);
      __decoratorMetadata(_init, Test);

      export { Test as default };
      "
    `);
  });
});
