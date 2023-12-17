import { noNestedDeriveCall } from "./noNestedDeriveCall";
import { noStateCreatedInDerive } from "./noStateCreatedInDerive";

const plugin = {
  name: "@idealjs/sapling",
  rules: {
    "no-nested-derive-call": noNestedDeriveCall,
    "no-state-created-in-derive": noStateCreatedInDerive,
  },
};

export default plugin;
