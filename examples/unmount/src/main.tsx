import { upsert } from "@idealjs/sapling";
import App from "./App";

const root = document.getElementById("app")!;

const unmount = upsert(root, <App />);

const button = document.createElement("button");
button.textContent = "unmount";
button.onclick = unmount;

root.append(button);
