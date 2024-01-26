import { upsert } from "@idealjs/sapling";
import App from "./App";

const root = document.getElementById("app")!;

upsert(root, <App />);
