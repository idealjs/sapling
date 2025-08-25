// import { createRoot } from "@idealjs/sapling";

// import App from "./App.tsx";

// const domNode = document.getElementById("root");
// const root = createRoot(domNode!);

// root.render(<App />);

import init, { transfrom } from "@idealjs/sapling-transformer";

await init();
let code = transfrom("const a = <div/>");
console.log(code);
