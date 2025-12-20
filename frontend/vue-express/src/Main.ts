// deno-lint-ignore-file
import { createServer } from "./api/Server.ts";
import { getApp } from "./WasmApp.ts";
import { NotCleanWasmApp } from "./workarounds/NotCleanWasmApp.ts";

const port = 4000;

const wasmApp = await getApp()
const notCleanWasmApp = new NotCleanWasmApp(wasmApp)
await notCleanWasmApp.initialize()

const app = await createServer(wasmApp, notCleanWasmApp);

app.listen(port, () => {
  console.log(`[EXPRESS APP] Server running at http://localhost:${port}`);
});
