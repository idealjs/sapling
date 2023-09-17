import { createState, derive } from "@idealjs/reactive";

derive(() => {
  createState();
  derive();
});
