import { createRoot } from "@idealjs/sapling";

import App from "./App.tsx";

const domNode = document.getElementById("root");
const root = createRoot(domNode!);

root.render(<App />);
