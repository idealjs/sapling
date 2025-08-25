// import { createRoot } from "@idealjs/sapling";

// import App from "./App.tsx";

// const domNode = document.getElementById("root");
// const root = createRoot(domNode!);

// root.render(<App />);

import init, { transform } from "@idealjs/sapling-transformer-web";

await init();
let code = transform("const a = <div/>");
console.log(code);
